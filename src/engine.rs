//! The search server state of one session and the tool entry points.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};

use serde_json::{Value, json};

use crate::batch;
use crate::cache::{CachedFile, CachedResult, ResultCache};
use crate::config::{self, Key, Settings};
use crate::content::DecodeOptions;
use crate::cursor::{self, Cursor};
use crate::error::{ErrorCode, Report};
use crate::hints;
use crate::languages::LanguageTable;
use crate::output;
use crate::output::{
    BlockOptions, Change, Detail, Highlight, Hint, Limits, Page, Response, Since, Sort, sort_files,
};
use crate::output_file::{self, FileTarget, Head};
use crate::query::{self, ParseOptions};
use crate::relevance;
use crate::request;
use crate::search::{self, Archives, Binary, KindOptions, Progress, SearchOptions};
use crate::warning::{WarningCode, Warnings};

/// Settings of an [`Engine`] which don't come from a request.
#[derive(Debug, Clone)]
pub struct EngineOptions {
    /// The directory the server was started in: the default working directory.
    pub start_dir: PathBuf,
    /// The home directory: `~` in paths of the configuration.
    pub home: Option<PathBuf>,
    /// Whether the global excludes file of git (`core.excludesFile`) is respected.
    pub git_global_excludes: bool,
    /// A fixed moment for relative dates; `None` — the clock when a request arrives.
    pub now: Option<jiff::Timestamp>,
    /// The local time zone of dates without an offset.
    pub time_zone: jiff::tz::TimeZone,
    /// The effective configuration.
    pub settings: config::Settings,
    /// What `path_case = native` means: case-insensitive paths, by default on macOS and Windows.
    pub native_case_insensitive: bool,
    /// A test hook: every read of a file's content takes this long, cut short at the deadline of the request.
    #[doc(hidden)]
    pub read_delay: Option<std::time::Duration>,
}

impl EngineOptions {
    /// Options with the defaults for a server started in `start_dir`.
    #[must_use]
    pub fn new(start_dir: PathBuf) -> Self {
        Self {
            settings: config::Settings::defaults(&start_dir),
            native_case_insensitive: NATIVE_CASE_INSENSITIVE,
            read_delay: None,
            start_dir,
            home: None,
            git_global_excludes: true,
            now: None,
            time_zone: jiff::tz::TimeZone::system(),
        }
    }
}

/// The text result of a tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolOutput {
    /// The text content item.
    pub text: String,
    /// `isError` of the tool result.
    pub is_error: bool,
}

impl ToolOutput {
    fn error(report: &Report) -> Self {
        Self {
            text: report.to_text(),
            is_error: true,
        }
    }
}

impl From<Result<String, Report>> for ToolOutput {
    fn from(result: Result<String, Report>) -> Self {
        match result {
            Ok(text) => Self {
                text,
                is_error: false,
            },
            Err(report) => Self::error(&report),
        }
    }
}

/// A tool name the server doesn't have.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown tool '{0}'")]
pub struct UnknownTool(pub String);

/// `path_case = native` by default: case-insensitive on macOS and Windows.
const NATIVE_CASE_INSENSITIVE: bool = cfg!(any(target_os = "macos", target_os = "windows"));

/// The search server state of one session: options and result ids.
#[derive(Debug)]
pub struct Engine {
    options: EngineOptions,
    /// The session settings: the settings of the options changed by `update_config`.
    settings: RwLock<Settings>,
    /// The results kept for `within` and `since`.
    results: Mutex<ResultCache>,
    /// The threads shared by the running searches.
    slots: Arc<Slots>,
    /// The files of `output.file = true` written without a result id: `out-<n>`.
    unnamed: AtomicUsize,
}

/// The threads of `server.threads`, shared by the running searches (TOOLS.md#concurrency): the number taken.
#[derive(Debug, Default)]
struct Slots {
    busy: Mutex<usize>,
    freed: std::sync::Condvar,
}

impl Slots {
    /// Takes the free ones of `capacity` threads, at least one, waiting while none is free; `None` when `stop` says
    /// the search is over while it waits (its deadline, its cancellation).
    fn acquire(self: &Arc<Self>, capacity: usize, stop: &dyn Fn() -> bool) -> Option<Lease> {
        let capacity = capacity.max(1);
        let mut busy = self.busy.lock().unwrap_or_else(PoisonError::into_inner);
        while *busy >= capacity {
            if stop() {
                return None;
            }
            busy = self
                .freed
                .wait_timeout(busy, std::time::Duration::from_millis(50))
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        let count = capacity - *busy;
        *busy += count;
        Some(Lease {
            slots: Arc::clone(self),
            count,
        })
    }
}

/// Threads taken by a search, given back when it is dropped.
#[derive(Debug)]
struct Lease {
    slots: Arc<Slots>,
    count: usize,
}

impl Lease {
    /// The number of threads.
    fn count(&self) -> usize {
        self.count
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        let mut busy = self
            .slots
            .busy
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        *busy -= self.count;
        self.slots.freed.notify_all();
    }
}

impl Engine {
    /// The names of the tools, in the order of `tools/list`.
    pub const TOOLS: [&'static str; 3] = ["find", "get_config", "update_config"];

    /// A new session.
    #[must_use]
    pub fn new(options: EngineOptions) -> Self {
        Self {
            settings: RwLock::new(options.settings.clone()),
            options,
            results: Mutex::default(),
            slots: Arc::default(),
            unnamed: AtomicUsize::new(0),
        }
    }

    /// The tools of this session, in the order of `tools/list`: `update_config` only with `server.allow_update`.
    #[must_use]
    pub fn tools(&self) -> Vec<&'static str> {
        Self::TOOLS
            .into_iter()
            .filter(|t| *t != "update_config" || self.options.settings.allow_update)
            .collect()
    }

    /// Calls a tool by name with its JSON arguments.
    ///
    /// # Errors
    ///
    /// [`UnknownTool`] if the server has no tool with this name.
    pub fn call(&self, tool: &str, arguments: &Value) -> Result<ToolOutput, UnknownTool> {
        self.call_with(tool, arguments, None)
    }

    /// Runs a tool call; a search counts its scanned and matched entries in `progress`.
    ///
    /// # Errors
    ///
    /// [`UnknownTool`] for a tool which is not served.
    pub fn call_with(
        &self,
        tool: &str,
        arguments: &Value,
        progress: Option<&Arc<Progress>>,
    ) -> Result<ToolOutput, UnknownTool> {
        match tool {
            "find" => Ok(self.find_with(arguments, progress)),
            "get_config" => Ok(ToolOutput::from(config::get_config(
                &self.settings(),
                arguments,
            ))),
            "update_config" if !self.options.settings.allow_update => Ok(ToolOutput::error(
                &Report::new(ErrorCode::UpdateDisabled, "update_config is disabled")
                    .hint("the configuration has server.allow_update = false"),
            )),
            "update_config" => {
                let mut settings = self
                    .settings
                    .write()
                    .unwrap_or_else(PoisonError::into_inner);
                Ok(ToolOutput::from(config::update_config(
                    &self.options.settings,
                    &mut settings,
                    arguments,
                )))
            }
            other => Err(UnknownTool(other.to_string())),
        }
    }

    /// The `find` tool.
    #[must_use]
    pub fn find(&self, arguments: &Value) -> ToolOutput {
        self.find_with(arguments, None)
    }

    /// `find` which counts its scanned and matched entries in `progress`.
    fn find_with(&self, arguments: &Value, progress: Option<&Arc<Progress>>) -> ToolOutput {
        if arguments.get("queries").is_some() {
            return self.batch(arguments, progress);
        }
        match self.try_find(arguments, progress, false) {
            Ok(answer) => ToolOutput {
                text: answer.text,
                is_error: false,
            },
            Err(report) if self.json_format(arguments) => ToolOutput {
                text: report.to_json(),
                is_error: true,
            },
            Err(report) => ToolOutput::error(&report),
        }
    }

    /// A batch request (OUTPUT.md#batch): every query in its section, the budget and `max_output` of the whole
    /// response split between them; `isError` only when all of them fail. `output.file` of the request writes the
    /// batch response to a file (OUTPUT.md#in-a-batch).
    fn batch(&self, arguments: &Value, progress: Option<&Arc<Progress>>) -> ToolOutput {
        let mut shared = batch::shared(arguments);
        let json = self.json_format(&shared);
        let fail = |report: &Report| ToolOutput {
            text: if json {
                report.to_json()
            } else {
                report.to_text()
            },
            is_error: true,
        };
        // the file of the request is the file of the whole batch, not of every query
        let shared_file = shared
            .get_mut("output")
            .and_then(Value::as_object_mut)
            .and_then(|output| output.remove("file"));
        let checked = batch::items(arguments).and_then(|items| {
            let request = request::parse(&shared)?;
            let settings = self
                .settings()
                .with(&request.config, &request.languages)
                .map_err(|p| p.request_report())?;
            let target = match shared_file.as_ref().map(request::file_target).transpose()? {
                Some(Some(file)) => Some(target(&settings, &file)?),
                _ => None,
            };
            Ok((items, settings, request, target))
        });
        let (items, settings, request, target) = match checked {
            Ok(checked) => checked,
            Err(report) => return fail(&report),
        };
        let own_file = |item: &Value| {
            item.get("output")
                .and_then(|o| o.get("file"))
                .is_some_and(|f| f.as_str().is_some_and(|p| !p.is_empty()) || f == &json!(true))
        };
        // a query in its own file gets no part of the shared budget and max_output
        let own = |key: &str| -> Vec<Option<usize>> {
            items
                .iter()
                .map(|item| {
                    if own_file(item) {
                        return Some(0);
                    }
                    let value = item.get("output")?.get(key)?.as_u64()?;
                    usize::try_from(value).ok()
                })
                .collect()
        };
        // with a file the totals are the values of the request, the configured ones are for the context
        let (budget_total, output_total) = if target.is_some() {
            (request.given(Key::Budget), request.given(Key::MaxOutput))
        } else {
            (
                settings.opt_whole(Key::Budget),
                Some(settings.whole(Key::MaxOutput)),
            )
        };
        let mut budget = budget_total.map(|total| batch::Split::new(total, &own("budget")));
        let mut output = output_total.map(|total| batch::Split::new(total, &own("max_output")));
        let chars_per_token = settings.number(Key::CharsPerToken);
        let mut sections = Vec::with_capacity(items.len());
        for (index, item) in items.iter().enumerate() {
            let parts = (
                budget.as_mut().map(|b| b.next(index).max(100)),
                output.as_mut().map(|o| o.next(index).max(100)),
            );
            let result = batch::item_arguments(&shared, item).and_then(|mut args| {
                let parts = (!own_file(item)).then_some(parts);
                set_item_output(&mut args, item, json, parts);
                self.try_find(&args, progress, target.is_some())
            });
            let used = result
                .as_ref()
                .map_or(0, |answer| answer.text.chars().count());
            if let Some(budget) = &mut budget {
                #[allow(
                    clippy::cast_precision_loss,
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss
                )]
                budget.used((used as f64 / chars_per_token).ceil() as usize);
            }
            if let Some(output) = &mut output {
                output.used(used);
            }
            sections.push((batch::label(item, index), result));
        }
        let is_error = sections.iter().all(|(_, result)| result.is_err());
        let (texts, heads) = answers(sections);
        let text = batch::response(&texts, json);
        // nothing is written when all the queries fail
        let Some(target) = target.filter(|_| !is_error) else {
            return ToolOutput { text, is_error };
        };
        match output_file::write(&self.file_path(target, None, json), &text) {
            Ok(written) => ToolOutput {
                text: batch::summary(&heads, json, &written),
                is_error: false,
            },
            Err(report) => fail(&report),
        }
    }

    /// Whether the response of a request is JSON: its `output.format`, or the format of the session when the request
    /// has none; an invalid one is text, the format of its error.
    fn json_format(&self, arguments: &Value) -> bool {
        match arguments.get("output").and_then(|o| o.get("format")) {
            Some(format) => format.as_str() == Some("json"),
            None => self.settings().text(Key::Format) == "json",
        }
    }

    /// A request of `find` without `queries`; `in_file`: its response goes to the file of a batch, so it has the
    /// limits for a file. With `output.file` the response is written to its file and the answer is its summary.
    fn try_find(
        &self,
        arguments: &Value,
        progress: Option<&Arc<Progress>>,
        in_file: bool,
    ) -> Result<Answer, Report> {
        let arrival = std::time::Instant::now();
        let request = request::parse(arguments)?;
        let settings = self
            .settings()
            .with(&request.config, &request.languages)
            .map_err(|p| p.request_report())?;
        // the path is checked before the search
        let target = request
            .file
            .as_ref()
            .map(|file| target(&settings, file))
            .transpose()?;
        let run = self.run(&request, &settings, arguments, arrival, progress)?;
        if progress.is_some_and(|p| p.is_cancelled()) {
            // no response is sent and no result is kept for a cancelled request
            return Err(Report::new(
                ErrorCode::Cancelled,
                "the request was cancelled",
            ));
        }
        let in_file = in_file || target.is_some();
        let (text, head, result_id) = self.respond(&settings, &request, arguments, run, in_file);
        let Some(target) = target else {
            return Ok(Answer { text, head });
        };
        let json = settings.text(Key::Format) == "json";
        let written =
            output_file::write(&self.file_path(target, result_id.as_deref(), json), &text)?;
        let summary = head.summary(&written);
        let head = match head {
            Head::Json(_) => Head::Json(summary.clone()),
            text_head @ Head::Text { .. } => text_head,
        };
        Ok(Answer {
            text: summary,
            head,
        })
    }

    /// The path of the file of a response: the path of `output.file`, or for `true` a name after the result id or
    /// `out-<n>` without one.
    fn file_path(&self, target: Target, result_id: Option<&str>, json: bool) -> PathBuf {
        match target {
            Target::Path(path) => path,
            Target::Auto(dir) => {
                let extension = if json { "json" } else { "txt" };
                let name = match result_id {
                    Some(id) => format!("{id}.{extension}"),
                    None => format!(
                        "out-{}.{extension}",
                        self.unnamed.fetch_add(1, Ordering::Relaxed) + 1
                    ),
                };
                dir.join(name)
            }
        }
    }

    /// Parses the query of a request and searches: the files of a kept result with `within`, the working
    /// directories otherwise.
    fn run(
        &self,
        request: &request::Request,
        settings: &Settings,
        arguments: &Value,
        arrival: std::time::Instant,
        progress: Option<&Arc<Progress>>,
    ) -> Result<Run, Report> {
        let languages = Arc::new(LanguageTable::new(settings.language_list()));
        let limit = request.timeout.or_else(|| settings.timeout());
        let deadline = limit.filter(|l| !l.is_zero()).map(|l| arrival + l);
        let within = request.within.map(|id| self.kept(id)).transpose()?;
        let since = request.since.map(|id| self.kept(id)).transpose()?;
        let working_dirs = match &within {
            Some(result) => result.working_dirs.clone(),
            None => self.working_dirs(request.dirs.as_deref(), settings)?,
        };
        let parse = self.parse_options(settings, Arc::clone(&languages));
        let query =
            query::parse(&request.query, &parse).map_err(|e| e.into_report(&request.query))?;
        let (page, cursor_totals) = page(arguments, request.cursor.as_deref())?;
        let mut search =
            self.search_options(settings, deadline, parse.case_insensitive_paths, languages);
        search.first_only = settings.text(Key::Detail) == "exists";
        search.progress = progress.cloned();
        let detail = detail(settings);
        search.funnel = settings.text(Key::Funnel) != "never" && detail >= Detail::Count;
        let facet_names = facet_names(settings, detail, &mut search);
        let threads = match search.threads {
            0 => std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
            n => n,
        };
        let stop = || {
            deadline.is_some_and(|d| std::time::Instant::now() >= d)
                || progress.is_some_and(|p| p.is_cancelled())
        };
        let lease = self.slots.acquire(threads, &stop);
        search.threads = lease.as_ref().map_or(1, Lease::count);
        let mut result = search_sorted(
            settings,
            &working_dirs,
            &query,
            &mut search,
            within.as_ref().map(|r| r.files.as_slice()),
        );
        if cursor_totals.is_some_and(|totals| totals != totals_of(&result.files)) {
            result.warnings.add_global(WarningCode::ResultsChanged);
        }
        Ok(Run {
            query,
            parse,
            search,
            working_dirs,
            page,
            detail,
            limit,
            facet_names,
            since,
            result,
            _lease: lease,
        })
    }

    /// The response of a search in the format of the settings, within the budget; the head of its summary and its
    /// result id. `in_file`: the limits for a file (OUTPUT.md#limits-for-a-file).
    fn respond(
        &self,
        settings: &Settings,
        request: &request::Request,
        arguments: &Value,
        run: Run,
        in_file: bool,
    ) -> (String, Head, Option<String>) {
        let Run {
            query,
            parse,
            search,
            working_dirs,
            page,
            detail,
            limit,
            facet_names,
            since,
            _lease,
            result:
                search::SearchResult {
                    mut files,
                    warnings,
                    timed_out,
                    funnel: funnel_counts,
                },
        } = run;
        let max_files = settings.whole(Key::MaxFiles);
        let shown = shown_funnel(
            &query,
            &funnel_counts,
            settings.text(Key::Funnel),
            files.len(),
            max_files,
        )
        .filter(|_| search.funnel);
        let hints = if shown.is_some() {
            let cx = hints::Context {
                working_dirs: &working_dirs,
                parse: &parse,
                search: &search,
                query: &query,
                counts: &funnel_counts,
                deadline: hints_deadline(limit),
            };
            response_hints(&cx, &files, max_files, settings.whole(Key::FacetDepth))
        } else {
            Vec::new()
        };
        let facets = output::facets::facets(
            &files,
            &facet_names,
            &query.legend,
            settings.whole(Key::FacetDepth),
        );
        if !settings.bool(Key::Meta) {
            for file in &mut files {
                file.meta = None;
            }
        }
        let result_id = if detail == Detail::Exists {
            None
        } else {
            self.keep_result(settings, arguments, &working_dirs, &files)
        };
        let (files, since) = match since {
            Some(kept) => {
                let (changed, since) = differences(&kept, files);
                (changed, Some(since))
            }
            None => (files, None),
        };
        let json = settings.text(Key::Format) == "json";
        let (limits, budget) = if in_file {
            file_limits(settings, request)
        } else {
            (limits(settings), settings.opt_whole(Key::Budget))
        };
        let response = Response {
            detail,
            group_dirs: settings.bool(Key::GroupDirs),
            group_repeated: settings.whole(Key::GroupRepeated),
            highlight: highlight(settings, json),
            time_zone: self.options.time_zone.clone(),
            page,
            funnel: shown,
            hints,
            facets,
            facet_limit: settings.whole(Key::FacetLimit),
            budget: None,
            since,
            limits,
            blocks: block_options(settings),
            result_id: result_id.as_deref(),
            legend: &query.legend,
            files: &files,
            warnings: &warnings,
            timed_out,
        };
        let (text, response) = match budget {
            Some(budget) => {
                output::fit(response, budget, settings.number(Key::CharsPerToken), json)
            }
            None if json => (response.to_json(), response),
            None => (response.to_text(), response),
        };
        let head = head(&text, &response, json);
        (text, head, result_id)
    }

    /// The options of the query of a request.
    fn parse_options(&self, settings: &Settings, languages: Arc<LanguageTable>) -> ParseOptions {
        ParseOptions {
            case_insensitive_paths: match settings.text(Key::PathCase) {
                "sensitive" => false,
                "insensitive" => true,
                _ => self.options.native_case_insensitive,
            },
            now: self.options.now.unwrap_or_else(jiff::Timestamp::now),
            time_zone: self.options.time_zone.clone(),
            languages,
            backslash_separator: query::BACKSLASH_SEPARATOR,
            max_escape_level: settings.whole(Key::MaxLevel),
            fuzzy_min_score: settings.number(Key::FuzzyMinScore),
        }
    }

    /// The working directories of a request (`server.default_dirs` without `dirs`) and their prefixes: as written,
    /// or the canonical directories with `path_style = absolute`.
    fn working_dirs(
        &self,
        dirs: Option<&[String]>,
        settings: &Settings,
    ) -> Result<Vec<(PathBuf, PathBuf)>, Report> {
        let mut working_dirs = dirs
            .unwrap_or_else(|| settings.list(Key::DefaultDirs))
            .iter()
            .map(|dir| self.working_dir(dir, settings))
            .collect::<Result<Vec<_>, _>>()?;
        if settings.text(Key::PathStyle) == "absolute" {
            for (path, prefix) in &mut working_dirs {
                *prefix = without_verbatim(path);
            }
        }
        Ok(working_dirs)
    }

    /// The options of the walk and the search of one request.
    fn search_options(
        &self,
        settings: &Settings,
        deadline: Option<std::time::Instant>,
        case_insensitive_paths: bool,
        languages: Arc<LanguageTable>,
    ) -> SearchOptions {
        let binary = match settings.text(Key::Binary) {
            "search" => Binary::Search,
            "exclude" => Binary::Exclude,
            _ => Binary::Skip,
        };
        SearchOptions {
            git_global_excludes: self.options.git_global_excludes,
            case_insensitive_paths,
            languages,
            deadline,
            threads: settings.whole(Key::Threads),
            min_depth: settings.whole(Key::MinDepth),
            max_depth: settings.opt_whole(Key::MaxDepth),
            follow_symlinks: settings.bool(Key::FollowSymlinks),
            allowed_roots: Arc::new(settings.allowed_roots.clone()),
            hidden: settings.bool(Key::Hidden),
            git_excludes: settings.bool(Key::GitExcludes),
            ignore_files: settings.list(Key::IgnoreFiles).to_vec(),
            ignore_paths: settings
                .list(Key::IgnorePaths)
                .iter()
                .map(|p| {
                    self.options
                        .start_dir
                        .join(config::expand_home(p, self.options.home.as_deref()))
                })
                .collect(),
            exclude: settings
                .list(Key::Exclude)
                .iter()
                .map(|w| {
                    query::wildcard::compile(&query::path_wildcard(w), case_insensitive_paths)
                        .expect("the wildcards of exclude are checked with their key")
                })
                .collect(),
            binary,
            decode: DecodeOptions {
                binary_as_text: binary == Binary::Search,
                encoding: match settings.text(Key::Encoding) {
                    "auto" => None,
                    label => {
                        Some(config::encoding(label).expect("the encoding is checked with its key"))
                    }
                },
                fallback: match settings.value(Key::FallbackEncoding) {
                    config::KeyValue::Text(label) => Some(
                        config::encoding(label)
                            .expect("the fallback encoding is checked with its key"),
                    ),
                    _ => None,
                },
            },
            archives: match settings.text(Key::Archives) {
                "none" => Archives::None,
                "compressed" => Archives::Compressed,
                _ => Archives::All,
            },
            time_zone: self.options.time_zone.clone(),
            max_file_size: settings.size(Key::MaxFileSize),
            max_archive_size: settings.size(Key::MaxArchiveSize),
            skip_generated: settings.bool(Key::SkipGenerated),
            git_base: Some(settings.text(Key::Base))
                .filter(|base| *base != "auto")
                .map(str::to_string),
            first_only: false,
            path_fragments: settings.bool(Key::PathFragments),
            meta: settings.bool(Key::Meta),
            rank: false,
            funnel: false,
            git_status: false,
            progress: None,
            kinds: KindOptions {
                generated_markers: settings.list(Key::GeneratedMarkers).to_vec(),
                generated_marker_lines: settings.whole(Key::GeneratedMarkerLines),
                config: settings.list(Key::KindsConfig).to_vec(),
                doc: settings.list(Key::KindsDoc).to_vec(),
                test: settings.list(Key::KindsTest).to_vec(),
                generated: settings.list(Key::KindsGenerated).to_vec(),
            },
            read_delay: self.options.read_delay,
        }
    }

    /// Keeps the result for `within` and `since`; its id, `None` when the result cache is off.
    fn keep_result(
        &self,
        settings: &Settings,
        arguments: &Value,
        working_dirs: &[(PathBuf, PathBuf)],
        files: &[search::Found],
    ) -> Option<String> {
        let cached = files
            .iter()
            .map(|f| CachedFile {
                root: f.root,
                rel: working_dirs
                    .get(f.root)
                    .and_then(|(_, prefix)| f.rel.strip_prefix(prefix).ok())
                    .unwrap_or(&f.rel)
                    .to_path_buf(),
                member: f.member.clone(),
                fingerprint: fingerprint(f),
            })
            .collect();
        self.results
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .keep(
                settings.whole(Key::ResultCache),
                arguments.clone(),
                working_dirs.to_vec(),
                cached,
            )
            .map(|n| format!("r{n}"))
    }

    /// The kept result `rN` of `within`; `result_expired` when it is unknown or evicted.
    fn kept(&self, id: u64) -> Result<CachedResult, Report> {
        self.results
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(id)
            .cloned()
            .ok_or_else(|| {
                Report::new(
                    ErrorCode::ResultExpired,
                    format!("the result r{id} is unknown or no longer kept"),
                )
                .hint("the server keeps the last server.result_cache results")
            })
    }

    /// A working directory as given: resolved against the start directory, checked and canonicalized; the path to
    /// search and the prefix of the output paths (the directory as given, `.` omitted).
    fn working_dir(&self, dir: &str, settings: &Settings) -> Result<(PathBuf, PathBuf), Report> {
        let path = self
            .options
            .start_dir
            .join(config::expand_home(dir, self.options.home.as_deref()));
        if !path.exists() {
            return Err(Report::new(
                ErrorCode::DirNotFound,
                format!("the working directory '{dir}' doesn't exist"),
            ));
        }
        if !path.is_dir() {
            return Err(Report::new(
                ErrorCode::DirNotFound,
                format!("the working directory '{dir}' is not a directory"),
            ));
        }
        let canonical = crate::paths::canonicalize(&path).unwrap_or(path);
        if !settings.is_allowed(&canonical) {
            return Err(Report::new(
                ErrorCode::DirNotAllowed,
                format!("the working directory '{dir}' is outside of the allowed roots"),
            )
            .hint("get_config shows the allowed roots"));
        }
        let prefix: PathBuf = Path::new(dir.trim_end_matches(['/', std::path::MAIN_SEPARATOR]))
            .components()
            .collect();
        Ok((canonical, prefix))
    }

    /// A snapshot of the session settings.
    fn settings(&self) -> Settings {
        self.settings
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

/// The search of a request with its files in the order of `output.sort`: ranked by relevance (its signals and the
/// metadata it needs are computed) or sorted; the metadata is kept only with `output.meta`.
fn search_sorted(
    settings: &Settings,
    working_dirs: &[(PathBuf, PathBuf)],
    query: &query::Query,
    options: &mut SearchOptions,
    within: Option<&[CachedFile]>,
) -> search::SearchResult {
    let sort = Sort::parse(settings.text(Key::Sort));
    // the hints of a huge result look at the languages and kinds
    options.meta |= settings.bool(Key::Meta) || sort.needs_meta() || options.funnel;
    options.rank = sort == Sort::Relevance;
    let mut result = match within {
        Some(files) => search_within(settings, working_dirs, files, query, options),
        None => search_all(working_dirs, query, options),
    };
    if sort == Sort::Relevance {
        relevance::rank(&mut result.files, &query.ranking);
    } else {
        sort_files(&mut result.files, sort);
    }
    result
}

/// The search of the files of a kept result (`within`) in its working directories: the paths are written as in the
/// result, a file outside of the allowed roots is skipped with `permission_denied`.
fn search_within(
    settings: &Settings,
    working_dirs: &[(PathBuf, PathBuf)],
    files: &[CachedFile],
    query: &query::Query,
    options: &SearchOptions,
) -> search::SearchResult {
    let mut found = Vec::new();
    let mut warnings = Warnings::default();
    let mut timed_out = false;
    let mut funnel: Vec<usize> = Vec::new();
    for (index, (path, prefix)) in working_dirs.iter().enumerate() {
        let mut visited = Vec::new();
        for file in files.iter().filter(|f| f.root == index) {
            let full = path.join(&file.rel);
            match crate::paths::canonicalize(&full) {
                Ok(canonical) if !settings.is_allowed(&canonical) => {
                    warnings.add(WarningCode::PermissionDenied, prefix.join(&file.rel), false);
                }
                Ok(_) => visited.push((file.rel.clone(), file.member.clone())),
                // deleted since the result
                Err(_) => {}
            }
        }
        let result = search::search_files(path, &visited, query, options);
        timed_out |= result.timed_out;
        funnel.resize(result.funnel.len().max(funnel.len()), 0);
        for (sum, n) in funnel.iter_mut().zip(&result.funnel) {
            *sum += n;
        }
        found.extend(result.files.into_iter().map(|mut f| {
            f.rel = prefix.join(&f.rel);
            f.path_fragments = f
                .path_fragments
                .map(|p| p.shown(prefix, f.member.is_some()));
            f.root = index;
            f
        }));
        warnings.extend(result.warnings, prefix);
        if timed_out {
            warnings.add_global(WarningCode::Timeout);
            break;
        }
    }
    found.sort_by(|a, b| (&a.rel, &a.member).cmp(&(&b.rel, &b.member)));
    search::SearchResult {
        files: found,
        warnings,
        timed_out,
        funnel,
    }
}

/// When the hints must stop: after 20% of the timeout, or 5 seconds without one.
fn hints_deadline(limit: Option<std::time::Duration>) -> std::time::Instant {
    let budget = match limit.filter(|l| !l.is_zero()) {
        Some(limit) => limit / 5,
        None => std::time::Duration::from_secs(5),
    };
    std::time::Instant::now() + budget
}

/// The hints which come with a shown funnel: of an empty result, of a huge one (more than 10 × `max_files` files).
fn response_hints(
    cx: &hints::Context<'_>,
    files: &[search::Found],
    max_files: usize,
    facet_depth: usize,
) -> Vec<Hint> {
    if files.is_empty() {
        hints::empty(cx)
    } else if files.len() > max_files.saturating_mul(10) {
        hints::huge(files, cx.query, facet_depth)
    } else {
        Vec::new()
    }
}

/// Where the page of a request starts: at its cursor, which must belong to the same request, or at the start; the
/// totals of the cursor.
fn page(arguments: &Value, cursor: Option<&str>) -> Result<(Page, Option<[usize; 3]>), Report> {
    let hash = cursor::request_hash(arguments);
    let Some(text) = cursor else {
        return Ok((
            Page {
                hash: Some(hash),
                ..Page::default()
            },
            None,
        ));
    };
    let cursor = Cursor::decode(text).ok_or_else(|| {
        Report::new(ErrorCode::CursorInvalid, "the cursor can't be decoded")
            .hint("pass next_cursor of the previous response unchanged")
    })?;
    if cursor.hash != hash {
        return Err(Report::new(
            ErrorCode::CursorMismatch,
            "the cursor belongs to another request",
        )
        .hint("repeat the request of the cursor with only output.cursor added"));
    }
    Ok((
        Page {
            file: cursor.file,
            fragment: cursor.fragment,
            hash: Some(hash),
        },
        Some(cursor.totals),
    ))
}

/// The steps of the funnel with their numbers of files, when it is shown: `always`, or `auto` for no files or more
/// than 10 × `max_files` (OUTPUT.md#funnel).
fn shown_funnel(
    query: &query::Query,
    counts: &[usize],
    setting: &str,
    files: usize,
    max_files: usize,
) -> Option<Vec<(String, usize)>> {
    let funnel = query.funnel.as_ref()?;
    let shown = setting == "always"
        || (setting == "auto" && (files == 0 || files > max_files.saturating_mul(10)));
    shown.then(|| {
        funnel
            .steps
            .iter()
            .zip(counts.iter().copied().chain(std::iter::repeat(0)))
            .map(|(step, count)| (step.text.clone(), count))
            .collect()
    })
}

/// The differences of the files of a result to a kept result (OUTPUT.md#changes-since): the added and changed files in
/// their order, then the removed ones in the order of the kept result; the same file has the same path and member.
fn differences(kept: &CachedResult, files: Vec<search::Found>) -> (Vec<search::Found>, Since) {
    let key = |rel: &std::path::Path, member: &Option<String>| (rel.to_path_buf(), member.clone());
    let before: std::collections::HashMap<(PathBuf, Option<String>), u64> = kept
        .files
        .iter()
        .map(|f| {
            let prefix = kept
                .working_dirs
                .get(f.root)
                .map_or_else(PathBuf::new, |(_, prefix)| prefix.clone());
            (key(&prefix.join(&f.rel), &f.member), f.fingerprint)
        })
        .collect();
    let totals = totals_of(&files);
    let now: HashSet<(PathBuf, Option<String>)> =
        files.iter().map(|f| key(&f.rel, &f.member)).collect();
    let mut unchanged = 0;
    let mut listed = Vec::new();
    let mut changes = Vec::new();
    for file in files {
        match before.get(&key(&file.rel, &file.member)) {
            None => {
                changes.push(Change::Added);
                listed.push(file);
            }
            Some(old) if *old != fingerprint(&file) => {
                changes.push(Change::Changed);
                listed.push(file);
            }
            Some(_) => unchanged += 1,
        }
    }
    for f in &kept.files {
        let prefix = kept
            .working_dirs
            .get(f.root)
            .map_or_else(PathBuf::new, |(_, prefix)| prefix.clone());
        let rel = prefix.join(&f.rel);
        if !now.contains(&key(&rel, &f.member)) {
            changes.push(Change::Removed);
            listed.push(search::Found::path_only(rel, f.member.clone()));
        }
    }
    (
        listed,
        Since {
            id: format!("r{}", kept.id),
            unchanged,
            changes,
            totals,
        },
    )
}

/// The counts of a result: files, matched lines, fragments.
fn totals_of(files: &[search::Found]) -> [usize; 3] {
    let (lines, matches) = files
        .iter()
        .filter_map(|f| f.matches.as_ref())
        .fold((0, 0), |(lines, matches), m| {
            (lines + m.lines, matches + m.fragments.len())
        });
    [files.len(), lines, matches]
}

/// The facets of `detail = facets`, also of a higher detail with a budget (none otherwise); the search then reads the
/// metadata and, for the `git` facet, the git status.
fn facet_names(settings: &Settings, detail: Detail, options: &mut SearchOptions) -> Vec<String> {
    // the budget can lower the detail to the facets
    let budget = settings.opt_whole(Key::Budget).is_some();
    if detail != Detail::Facets && !(budget && detail > Detail::Facets) {
        return Vec::new();
    }
    let names = settings.list(Key::Facets).to_vec();
    options.meta = true;
    options.git_status = names.iter().any(|f| f == "git");
    names
}

/// How much the response returns.
fn detail(settings: &Settings) -> Detail {
    match settings.text(Key::Detail) {
        "exists" => Detail::Exists,
        "count" => Detail::Count,
        "facets" => Detail::Facets,
        "files" => Detail::Files,
        _ => Detail::Lines,
    }
}

/// The limits of what is listed.
fn limits(settings: &Settings) -> Limits {
    Limits {
        max_files: settings.whole(Key::MaxFiles),
        max_lines: settings.whole(Key::MaxLines),
        max_matches: settings.whole(Key::MaxMatches),
        max_matches_per_file: settings.whole(Key::MaxMatchesPerFile),
        max_line_length: settings.whole(Key::MaxLineLength),
        max_output_per_file: settings.whole(Key::MaxOutputPerFile),
        max_output: settings.whole(Key::MaxOutput),
        budget_files: usize::MAX,
        file_cap: false,
    }
}

/// The head of the summary of a response: its first line and its warnings, or its summary fields in JSON.
fn head(text: &str, response: &Response<'_>, json: bool) -> Head {
    if json {
        Head::Json(response.summary_json())
    } else {
        Head::Text {
            first: text.lines().next().unwrap_or_default().to_string(),
            warnings: response.warnings_text(),
        }
    }
}

/// The sections of a batch: their texts and the heads of their summaries.
type Sections<T> = Vec<(String, Result<T, Report>)>;

/// The texts and the heads of the answers of the sections of a batch.
fn answers(sections: Sections<Answer>) -> (Sections<String>, Sections<Head>) {
    sections
        .into_iter()
        .map(|(label, result)| match result {
            Ok(answer) => ((label.clone(), Ok(answer.text)), (label, Ok(answer.head))),
            Err(report) => ((label.clone(), Err(report.clone())), (label, Err(report))),
        })
        .unzip()
}

/// The output fields of the arguments of a batch item: the shared format, and its parts of the shared `budget` and
/// `max_output`; `parts` is `None` for an item in its own file, which keeps only its own values of them.
fn set_item_output(
    args: &mut Value,
    item: &Value,
    json: bool,
    parts: Option<(Option<usize>, Option<usize>)>,
) {
    let Some(object) = args
        .as_object_mut()
        .and_then(|a| a.entry("output").or_insert(json!({})).as_object_mut())
    else {
        return;
    };
    object.insert("format".into(), json!(if json { "json" } else { "text" }));
    match parts {
        Some((budget, max_output)) => {
            if let Some(budget) = budget {
                object.insert("budget".into(), json!(budget));
            }
            if let Some(max_output) = max_output {
                object.insert("max_output".into(), json!(max_output));
            }
        }
        // the shared values are for the response in the context
        None => {
            for key in ["budget", "max_output"] {
                if item.get("output").and_then(|o| o.get(key)).is_none() {
                    object.remove(key);
                }
            }
        }
    }
}

/// The limits and the budget of a response written to a file (OUTPUT.md#limits-for-a-file): `max_output`,
/// `max_output_per_file` and `budget` only from the request, `max_output` at most `server.max_output_file_size`.
fn file_limits(settings: &Settings, request: &request::Request) -> (Limits, Option<usize>) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // a size in bytes
    let cap = crate::query::parse_size(&settings.max_output_file_size)
        .map_or(usize::MAX, |size| size as usize);
    let max_output = request.given(Key::MaxOutput).unwrap_or(usize::MAX);
    let limits = Limits {
        max_output: max_output.min(cap),
        file_cap: cap < max_output,
        max_output_per_file: request.given(Key::MaxOutputPerFile).unwrap_or(usize::MAX),
        ..limits(settings)
    };
    (limits, request.given(Key::Budget))
}

/// The response of a request: its text (the summary when it was written to a file) and the head of its summary.
struct Answer {
    text: String,
    head: Head,
}

/// Where the response of `output.file` goes: a checked path, or a name of the server in the output directory.
enum Target {
    Path(PathBuf),
    Auto(PathBuf),
}

/// The target of `output.file` in `server.output_dir`: `output_file_disabled` without one, `output_file_denied` for
/// a path outside of it.
fn target(settings: &Settings, file: &FileTarget) -> Result<Target, Report> {
    let dir = settings.output_dir.as_ref().ok_or_else(|| {
        Report::new(
            ErrorCode::OutputFileDisabled,
            "output to a file is disabled",
        )
        .hint("the configuration has server.output_dir = \"\"")
    })?;
    match file {
        FileTarget::Path(path) => output_file::resolve(dir, path).map(Target::Path),
        FileTarget::Auto => Ok(Target::Auto(dir.clone())),
    }
}

/// A search of a request, before its response.
struct Run {
    query: query::Query,
    parse: ParseOptions,
    search: SearchOptions,
    working_dirs: Vec<(PathBuf, PathBuf)>,
    page: Page,
    detail: Detail,
    /// The time limit of the request.
    limit: Option<std::time::Duration>,
    facet_names: Vec<String>,
    /// The kept result of `since`.
    since: Option<CachedResult>,
    result: search::SearchResult,
    /// The threads of the search, given back with the response.
    _lease: Option<Lease>,
}

/// How the fragments are shown: `auto` is markers which don't outgrow the line in text and columns in JSON.
fn highlight(settings: &Settings, json: bool) -> Highlight<'_> {
    let markers = settings.list(Key::Markers);
    match settings.text(Key::Highlight) {
        "none" => Highlight::None,
        "columns" => Highlight::Columns,
        "markers" => Highlight::Markers(&markers[0], &markers[1]),
        _ if json => Highlight::Columns,
        _ => Highlight::Auto(&markers[0], &markers[1]),
    }
}

/// How the matched lines are put into blocks: `before` and `after` default to `context`.
fn block_options(settings: &Settings) -> BlockOptions {
    let context = settings.whole(Key::Context);
    BlockOptions {
        before: settings.opt_whole(Key::Before).unwrap_or(context),
        after: settings.opt_whole(Key::After).unwrap_or(context),
        blank_context: settings.bool(Key::BlankContext),
        dedent: settings.bool(Key::Dedent),
    }
}

/// A canonical path as it is shown: without the verbatim prefix `\\?\` which Windows gives canonical paths.
fn without_verbatim(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{rest}"))
    } else if let Some(rest) = text.strip_prefix(r"\\?\") {
        PathBuf::from(rest)
    } else {
        path.to_path_buf()
    }
}

/// Searches the working directories in their order: the matched entries sorted by path, every entry once, the
/// warnings, and whether the search stopped at the deadline. With `first_only` it stops after the first directory
/// with a match.
pub(crate) fn search_all(
    working_dirs: &[(PathBuf, PathBuf)],
    query: &query::Query,
    options: &SearchOptions,
) -> search::SearchResult {
    let mut files = Vec::new();
    let mut warnings = Warnings::default();
    let mut seen = HashSet::new();
    let mut timed_out = false;
    let mut funnel: Vec<usize> = Vec::new();
    for (index, (path, prefix)) in working_dirs.iter().enumerate() {
        let result = search::search(path, query, options);
        timed_out |= result.timed_out;
        funnel.resize(result.funnel.len().max(funnel.len()), 0);
        for (sum, n) in funnel.iter_mut().zip(&result.funnel) {
            *sum += n;
        }
        files.extend(
            result
                .files
                .into_iter()
                .filter(|f| f.id.is_none_or(|id| seen.insert((id, f.member.clone()))))
                .map(|mut f| {
                    f.rel = prefix.join(&f.rel);
                    f.path_fragments = f
                        .path_fragments
                        .map(|p| p.shown(prefix, f.member.is_some()));
                    f.root = index;
                    f
                }),
        );
        warnings.extend(result.warnings, prefix);
        if timed_out {
            warnings.add_global(WarningCode::Timeout);
            break;
        }
        if options.first_only && !files.is_empty() {
            break;
        }
    }
    files.sort_by(|a, b| (&a.rel, &a.member).cmp(&(&b.rel, &b.member)));
    search::SearchResult {
        files,
        warnings,
        timed_out,
        funnel,
    }
}

/// A hash of the matched lines of a file, to tell a changed file from an unchanged one.
fn fingerprint(file: &search::Found) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    if let Some(m) = &file.matches {
        for fragment in &m.fragments {
            let first = m.content.line_of(fragment.range.start);
            let last = m
                .content
                .line_of(fragment.range.end.max(fragment.range.start + 1) - 1);
            for line in first..=last {
                m.content.line(line).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_share_the_threads() {
        let slots = std::sync::Arc::new(Slots::default());
        let never = || false;
        let first = slots.acquire(3, &never).unwrap();
        assert_eq!(first.count(), 3, "all the free threads");
        let waiting = {
            let slots = std::sync::Arc::clone(&slots);
            std::thread::spawn(move || slots.acquire(3, &|| false).unwrap().count())
        };
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!waiting.is_finished(), "no thread is free");
        drop(first);
        assert_eq!(waiting.join().unwrap(), 3);
        assert!(
            slots.acquire(2, &|| true).is_some(),
            "free threads don't wait"
        );
        let busy = slots.acquire(1, &never).unwrap();
        assert!(
            slots.acquire(1, &|| true).is_none(),
            "a stopped search doesn't wait"
        );
        drop(busy);
    }
}
