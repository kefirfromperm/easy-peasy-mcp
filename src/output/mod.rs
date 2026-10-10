//! The response of `find` in the text format (OUTPUT.md#text-format) and in JSON (`json`).

use std::borrow::Cow;
use std::fmt::Write as _;
use std::path::{Component, Path};

use crate::content::Content;
use crate::query::ast::CondId;
use crate::search::{Found, PathFragments};
use crate::warning::{WarningCode, Warnings};

mod blocks;
mod budget;
pub mod facets;
mod funnel;
mod json;
mod lines;
mod listing;
mod meta;
mod repeated;
mod sort;

pub use blocks::BlockOptions;
pub use budget::fit;
pub use funnel::Hint;
use listing::{Listed, Listing};
pub use sort::{Sort, sort_files};

pub use lines::{Fragment, matched_lines, merge_identical};

/// How much a response returns (OUTPUT.md#detail-levels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Detail {
    /// Only whether at least one file matches.
    Exists,
    /// The totals.
    Count,
    /// The totals, the legend and the files per value of the facets.
    Facets,
    /// The totals, the legend and the files.
    Files,
    /// The totals, the legend, the files and their matched lines.
    Lines,
}

/// How the fragments are shown in the lines (OUTPUT.md#highlight); `auto` is resolved by the format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Highlight<'a> {
    /// The lines as they are; JSON: `"match": true` on matched lines.
    None,
    /// Text: the columns of the fragments after a line; JSON: `fragments`.
    Columns,
    /// The opening and the closing marker inside the lines.
    Markers(&'a str, &'a str),
}

/// The limits of the listed files, lines and output (OUTPUT.md#limits-and-truncation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::struct_field_names)] // named like the keys
pub struct Limits {
    /// The files listed in `files` and `lines` detail.
    pub max_files: usize,
    /// The matched lines listed under the files.
    pub max_lines: usize,
    /// The fragments listed under the files.
    pub max_matches: usize,
    /// The fragments listed of one file.
    pub max_matches_per_file: usize,
    /// The characters of a line, longer lines are cut (OUTPUT.md#long-lines).
    pub max_line_length: usize,
    /// The characters of the lines of one file.
    pub max_output_per_file: usize,
    /// The characters of the paths and lines of all files.
    pub max_output: usize,
    /// The files listed within the budget.
    pub budget_files: usize,
}

impl Default for Limits {
    /// No limits.
    fn default() -> Self {
        Self {
            max_files: usize::MAX,
            max_lines: usize::MAX,
            max_matches: usize::MAX,
            max_matches_per_file: usize::MAX,
            max_line_length: usize::MAX,
            max_output_per_file: usize::MAX,
            max_output: usize::MAX,
            budget_files: usize::MAX,
        }
    }
}

/// Where a page starts and what its next cursor is made of (OUTPUT.md#pagination).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Page {
    /// The index of the first file of the page in the sorted result.
    pub file: usize,
    /// The index of the first fragment of that file which is listed.
    pub fragment: usize,
    /// The hash of the request; `None`: no cursor.
    pub hash: Option<String>,
}

impl Detail {
    /// The level as written in the configuration.
    #[must_use]
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Exists => "exists",
            Self::Count => "count",
            Self::Facets => "facets",
            Self::Files => "files",
            Self::Lines => "lines",
        }
    }
}

/// How the budget changed a response (OUTPUT.md#budget).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BudgetUse {
    /// The budget in tokens.
    pub tokens: usize,
    /// Whether the detail is lower than the requested one.
    pub lowered: bool,
}

/// The response of a `find` call.
pub struct Response<'a> {
    /// How much it returns.
    pub detail: Detail,
    /// How the matched lines of a file are put into blocks.
    pub blocks: BlockOptions,
    /// Text: files under directory headers, with paths relative to them.
    pub group_dirs: bool,
    /// A matched line in this many files is printed once (`lines` detail); `0`: off.
    pub group_repeated: usize,
    /// How the fragments are shown.
    pub highlight: Highlight<'a>,
    /// The time zone of the dates of `meta`.
    pub time_zone: jiff::tz::TimeZone,
    /// The limits of what is listed.
    pub limits: Limits,
    /// Where the page starts.
    pub page: Page,
    /// The steps of the funnel with their numbers of files, when it is shown.
    pub funnel: Option<Vec<(String, usize)>>,
    /// The hints which come with the funnel.
    pub hints: Vec<Hint>,
    /// `detail = facets`: the facets of all matched files.
    pub facets: Vec<facets::Facet>,
    /// The values of a facet written before `… N more (M)`.
    pub facet_limit: usize,
    /// How the budget changed the response, if it did.
    pub budget: Option<BudgetUse>,
    /// `None` when the result cache is off.
    pub result_id: Option<&'a str>,
    pub legend: &'a [(CondId, String)],
    pub files: &'a [Found],
    /// Printed after the files.
    pub warnings: &'a Warnings,
    /// The search was stopped by the timeout: `truncated: yes (timeout)`.
    pub timed_out: bool,
}

/// The separator of output paths on this platform.
const SEP: char = std::path::MAIN_SEPARATOR;

/// The locations printed for a repeated line.
const LOCATIONS: usize = 3;

impl Response<'_> {
    /// The matched lines and the fragments of all files.
    fn totals(&self) -> (usize, usize) {
        self.files
            .iter()
            .filter_map(|file| file.matches.as_ref())
            .fold((0, 0), |(lines, matches), m| {
                (lines + m.lines, matches + m.fragments.len())
            })
    }

    /// The text format.
    #[must_use]
    pub fn to_text(&self) -> String {
        let repeated = self.repeated();
        let listing = self.listing(&repeated);
        let truncated_by = self.truncated_by(&listing);
        let mut timeout = if truncated_by.is_empty() {
            String::new()
        } else {
            format!("  truncated: yes ({})", truncated_by.join(", "))
        };
        if let Some(cursor) = self.next_cursor(&listing) {
            let _ = write!(timeout, "  next cursor: {cursor}");
        }
        if self.detail == Detail::Exists {
            let exists = if self.files.is_empty() { "no" } else { "yes" };
            let mut out = format!("exists: {exists}{timeout}\n");
            self.push_warnings(&mut out);
            return out;
        }
        let (lines, matches) = self.totals();
        let mut out = format!(
            "files: {}  lines: {lines}  matches: {matches}",
            self.files.len(),
        );
        if let Some(id) = self.result_id {
            let _ = write!(out, "  result: {id}");
        }
        if let Some(budget) = self.budget.filter(|b| b.lowered) {
            let _ = write!(
                out,
                "  detail: {} (budget {})",
                self.detail.keyword(),
                budget.tokens
            );
        }
        out.push_str(&timeout);
        out.push('\n');
        if self.files.is_empty() || self.detail == Detail::Count {
            self.push_funnel(&mut out);
            self.push_warnings(&mut out);
            return out;
        }
        for (id, text) in self.legend {
            let _ = writeln!(out, "{id} {text}");
        }
        self.push_funnel(&mut out);
        if self.detail == Detail::Facets {
            for facet in &self.facets {
                let _ = writeln!(out, "{}", facet.line(self.facet_limit));
            }
            self.push_warnings(&mut out);
            return out;
        }
        out.push('\n');
        self.push_repeated(&mut out, &repeated);
        let mut current: Option<String> = None;
        for (index, file) in self.listed().iter().enumerate().take(listing.shown) {
            if listing.files[index] != Listed::LeftOut {
                self.push_file(&mut out, file, &listing, index, &mut current);
            }
        }
        self.push_warnings(&mut out);
        out
    }

    /// The listed file `index`: its line (under its directory header when it differs from `current`; `(cut)` when a
    /// per file limit cut it), its path fragments and its listed lines.
    fn push_file(
        &self,
        out: &mut String,
        file: &Found,
        listing: &Listing,
        index: usize,
        current: &mut Option<String>,
    ) {
        if self.group_dirs {
            let (dir, name) = split_found(file);
            if current.as_deref() != Some(dir.as_str()) {
                out.push_str(&dir);
                out.push('\n');
                *current = Some(dir);
            }
            out.push_str(&name);
            if file.is_dir {
                out.push(SEP);
            }
        } else {
            out.push_str(&display_found(file));
        }
        if !file.conditions.is_empty() {
            let ids: Vec<String> = file.conditions.iter().map(ToString::to_string).collect();
            let _ = write!(out, "  [{}]", ids.join(" "));
        }
        if let Some(m) = &file.matches {
            let _ = write!(out, "  lines: {}  matches: {}", m.lines, m.fragments.len());
        }
        if listing.cut[index] {
            out.push_str("  (cut)");
        }
        if let Some(encoding) = file.encoding {
            let _ = write!(out, "  {}", encoding.name());
        }
        if let Some(meta) = &file.meta {
            let _ = write!(out, "  {}", meta::text(meta, &self.time_zone));
            if let Some(score) = file.score {
                let _ = write!(out, " {:.2}", crate::relevance::shown_score(score));
            }
        }
        out.push('\n');
        if let Some(path) = &file.path_fragments {
            self.push_path_fragments(out, path, file.is_dir);
        }
        if let (Some(m), Listed::Lines(fragments)) = (&file.matches, &listing.files[index]) {
            let blocks = blocks::before(
                blocks::blocks(&m.content, fragments, &self.blocks),
                listing.until[index],
            );
            out.push_str(&lines::render(
                &m.content,
                fragments,
                &blocks,
                self.highlight,
                self.limits.max_line_length,
            ));
        }
    }

    /// The line of the path fragments in the highlight style: `  path: <path with markers>`, `  path @ …`, none
    /// with `none` (OUTPUT.md#path-fragments).
    fn push_path_fragments(&self, out: &mut String, path: &PathFragments, is_dir: bool) {
        let mut shown = path.path.clone();
        if is_dir {
            shown.push(SEP);
        }
        let content = Content::from_text(shown);
        match self.highlight {
            Highlight::Markers(..) => {
                let _ = writeln!(
                    out,
                    "  path: {}",
                    lines::shown_line(&content, &path.fragments, 0, 0, self.highlight)
                );
            }
            Highlight::Columns => {
                if let Some(columns) = lines::columns_text(&content, &path.fragments, 0) {
                    let _ = writeln!(out, "  path {columns}");
                }
            }
            Highlight::None => {}
        }
    }

    /// The files which are listed: in `files` and `lines` detail the first `max_files` from the start of the page.
    fn listed(&self) -> &[Found] {
        if self.detail >= Detail::Files {
            let rest = &self.files[self.page_start()..];
            &rest[..rest.len().min(self.files_limit())]
        } else {
            &[]
        }
    }

    /// The index of the first file of the page.
    fn page_start(&self) -> usize {
        self.page.file.min(self.files.len())
    }

    /// Whether `max_files` leaves out files after the page.
    fn files_cut(&self) -> bool {
        self.detail >= Detail::Files && self.files.len() - self.page_start() > self.files_limit()
    }

    /// The files listed at most: `max_files`, fewer within the budget.
    fn files_limit(&self) -> usize {
        self.limits.max_files.min(self.limits.budget_files)
    }

    /// The cursor of the next page: where a cursor limit left something out, unless the search was stopped by the
    /// timeout.
    fn next_cursor(&self, listing: &Listing) -> Option<String> {
        let hash = self.page.hash.as_ref().filter(|_| !self.timed_out)?;
        let start = self.page_start();
        let mut next = listing
            .next
            .map(|(file, fragment)| (start + file, fragment));
        if self.files_cut() {
            let after = (start + self.limits.max_files, 0);
            next = Some(next.map_or(after, |n| n.min(after)));
        }
        let (file, fragment) = next.filter(|(file, _)| *file < self.files.len())?;
        let (lines, matches) = self.totals();
        Some(
            crate::cursor::Cursor {
                hash: hash.clone(),
                file,
                fragment,
                totals: [self.files.len(), lines, matches],
            }
            .encode(),
        )
    }

    /// The limits which were hit, in the order of the limits table.
    fn truncated_by(&self, listing: &Listing) -> Vec<&'static str> {
        let mut by = Vec::new();
        if self.detail >= Detail::Files
            && self.files.len() - self.page_start() > self.limits.max_files
        {
            by.push("max_files");
        }
        by.extend(&listing.truncated_by);
        if self.budget.is_some() {
            by.push("budget");
        }
        if self.timed_out {
            by.push("timeout");
        }
        by
    }

    /// What is listed of the listed files: in `lines` detail their lines after the repeated lines and the limits.
    fn listing(&self, repeated: &repeated::Repeated) -> Listing {
        if self.detail == Detail::Lines {
            Listing::new(
                self.listed(),
                repeated,
                &self.limits,
                &self.blocks,
                self.page.fragment,
            )
        } else {
            Listing::without_lines(self.listed(), &self.limits)
        }
    }

    /// The repeated lines of the files, only in `lines` detail.
    fn repeated(&self) -> repeated::Repeated {
        if self.detail == Detail::Lines {
            repeated::repeated(self.listed(), self.group_repeated)
        } else {
            repeated::Repeated::default()
        }
    }

    /// The `repeated:` section: `  ×N <line>    <locations>` (OUTPUT.md#repeated-lines).
    fn push_repeated(&self, out: &mut String, repeated: &repeated::Repeated) {
        if repeated.lines.is_empty() {
            return;
        }
        out.push_str("repeated:\n");
        for line in &repeated.lines {
            let _ = writeln!(
                out,
                "  ×{} {}    {}",
                line.files.len(),
                lines::shown_line(&line.content, &line.fragments, 0, 0, self.highlight),
                self.locations(line).join(", ")
            );
            if self.highlight == Highlight::Columns {
                lines::push_columns(out, &line.content, &line.fragments, 0);
            }
        }
    }

    /// The first three locations of a repeated line, `path:line`, and `… M more`.
    fn locations(&self, line: &repeated::RepeatedLine) -> Vec<String> {
        let mut out: Vec<String> = line
            .files
            .iter()
            .take(LOCATIONS)
            .map(|&(file, i)| format!("{}:{}", display_found(&self.files[file]), i + 1))
            .collect();
        if line.files.len() > LOCATIONS {
            out.push(format!("… {} more", line.files.len() - LOCATIONS));
        }
        out
    }

    fn push_funnel(&self, out: &mut String) {
        if let Some(steps) = &self.funnel {
            out.push_str(&funnel::lines(steps, &self.hints));
        }
    }

    fn push_warnings(&self, out: &mut String) {
        let warnings = self.all_warnings();
        if !warnings.is_empty() {
            out.push('\n');
            out.push_str(&warnings.to_text());
        }
    }

    /// The warnings of the search and, in `lines` detail with markers, `marker_in_content` for the files with a
    /// matched line which contains a marker.
    fn all_warnings(&self) -> Cow<'_, Warnings> {
        let Highlight::Markers(open, close) = self.highlight else {
            return Cow::Borrowed(self.warnings);
        };
        if self.detail != Detail::Lines {
            return Cow::Borrowed(self.warnings);
        }
        let mut warnings = Cow::Borrowed(self.warnings);
        for file in self.listed() {
            let Some(m) = &file.matches else {
                continue;
            };
            let marked = lines::lines_of(&m.content, &m.fragments)
                .into_iter()
                .any(|i| {
                    let line = m.content.line(i);
                    line.contains(open) || line.contains(close)
                });
            if marked {
                let code = WarningCode::MarkerInContent;
                match &file.member {
                    Some(member) => {
                        warnings
                            .to_mut()
                            .add_member(code, file.rel.clone(), member.clone());
                    }
                    None => warnings.to_mut().add(code, file.rel.clone(), file.is_dir),
                }
            }
        }
        warnings
    }
}

/// The parts of a path to join with the separator: `..` kept, `.` dropped, an empty first part for the root of an
/// absolute path, the prefix of a Windows path (`C:`) as its first part.
fn path_parts(path: &Path) -> Vec<String> {
    let mut prefixed = false;
    path.components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            Component::ParentDir => Some("..".to_string()),
            Component::CurDir => None,
            // after a prefix (`C:`) the root is the separator which joins them
            Component::RootDir => (!prefixed).then(String::new),
            Component::Prefix(p) => {
                prefixed = true;
                Some(p.as_os_str().to_string_lossy().into_owned())
            }
        })
        .collect()
}

/// A path in the native form, a directory with a trailing separator.
pub fn display_path(rel: &Path, is_dir: bool) -> String {
    let parts = path_parts(rel);
    let mut out = parts.join(&SEP.to_string());
    if is_dir {
        out.push(SEP);
    }
    out
}

/// The full path of a matched entry: a directory with the separator, a member of an archive after `!/`.
pub fn display_found(file: &Found) -> String {
    match &file.member {
        Some(member) => format!("{}!/{member}", display_path(&file.rel, false)),
        None => display_path(&file.rel, file.is_dir),
    }
}

/// The directory header and the name of a matched entry; a member of an archive is under `<archive>!/<member dir>/`,
/// the part after `!/` with `/`.
fn split_found(file: &Found) -> (String, String) {
    let Some(member) = &file.member else {
        return split(&file.rel);
    };
    let archive = display_path(&file.rel, false);
    match member.rsplit_once('/') {
        Some((dir, name)) => (format!("{archive}!/{dir}/"), name.to_string()),
        None => (format!("{archive}!/"), member.clone()),
    }
}

/// The directory header (ending with the separator; `./` for the working directory) and the file name.
fn split(rel: &Path) -> (String, String) {
    let parts = path_parts(rel);
    let (name, dirs) = parts
        .split_last()
        .map_or((String::new(), &[][..]), |(n, d)| (n.clone(), d));
    let dir = if dirs.is_empty() {
        format!(".{SEP}")
    } else {
        let mut d = dirs.join(&SEP.to_string());
        d.push(SEP);
        d
    };
    (dir, name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn found(path: &str) -> Found {
        Found {
            rel: PathBuf::from(path),
            member: None,
            is_dir: false,
            conditions: vec![CondId::Number(1)],
            matches: None,
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
            git: None,
            root: 0,
        }
    }

    #[test]
    fn groups_files_under_directory_headers() {
        let files = [
            found("Cargo.toml"),
            found("src/a.rs"),
            found("src/b.rs"),
            found("src/x/c.rs"),
        ];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: Some("r3"),
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert_eq!(
            text,
            format!(
                "files: 4  lines: 0  matches: 0  result: r3\n#1 name ~ '*'\n\n.{s}\nCargo.toml  [#1]\nsrc{s}\na.rs  [#1]\nb.rs  [#1]\nsrc{s}x{s}\nc.rs  [#1]\n"
            )
        );
    }

    #[test]
    fn full_paths_without_groups() {
        let files = [found("Cargo.toml"), found("src/a.rs")];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: false,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert_eq!(
            text,
            format!(
                "files: 2  lines: 0  matches: 0\n#1 name ~ '*'\n\nCargo.toml  [#1]\nsrc{s}a.rs  [#1]\n"
            )
        );
    }

    #[test]
    fn files_without_conditions_have_no_brackets() {
        let files = [Found {
            rel: PathBuf::from("a.md"),
            member: None,
            is_dir: false,
            conditions: vec![],
            matches: None,
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
            git: None,
            root: 0,
        }];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &[],
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert_eq!(
            text,
            format!("files: 1  lines: 0  matches: 0  result: r1\n\n.{s}\na.md\n")
        );
    }

    #[test]
    fn directories_end_with_the_separator() {
        let mut dir = found("lib/sub");
        dir.is_dir = true;
        let files = [dir];
        let legend = [(CondId::Number(1), "type = dir".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        let s = SEP;
        assert!(
            text.ends_with(&format!("\n\nlib{s}\nsub{s}  [#1]\n")),
            "{text}"
        );
    }

    #[test]
    fn repeated_lines_show_three_locations() {
        let files: Vec<Found> = ["a", "b", "c", "d"]
            .iter()
            .map(|name| {
                let content = crate::content::Content::from_bytes(b"use x;\n");
                let fragments = vec![Fragment {
                    range: 4..5,
                    ids: vec![(1, CondId::Number(1))],
                    escape: None,
                }];
                Found {
                    matches: Some(crate::search::FileMatches {
                        content,
                        fragments,
                        lines: 1,
                    }),
                    ..found(name)
                }
            })
            .collect();
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        assert_eq!(
            text,
            "files: 4  lines: 4  matches: 4\n#1 text has 'x'\n\nrepeated:\n  ×4 use ⟦#1:x⟧;    a:1, b:1, c:1, … 1 more\n"
        );
    }

    #[test]
    fn a_marker_in_a_matched_line_is_a_warning() {
        let content = crate::content::Content::from_bytes("a ⟧ b\nc\n".as_bytes());
        let b = content.text().find('b').unwrap();
        let files = [Found {
            matches: Some(crate::search::FileMatches {
                content,
                fragments: vec![Fragment {
                    range: b..b + 1,
                    ids: vec![(1, CondId::Number(1))],
                    escape: None,
                }],
                lines: 1,
            }),
            ..found("f")
        }];
        let legend = [(CondId::Number(1), "text has 'b'".to_string())];
        let response = |highlight: Highlight| {
            Response {
                detail: Detail::Lines,
                blocks: BlockOptions::default(),
                group_dirs: false,
                group_repeated: 3,
                highlight,
                time_zone: jiff::tz::TimeZone::UTC,
                page: Page::default(),
                funnel: None,
                hints: Vec::new(),
                facets: Vec::new(),
                facet_limit: 8,
                budget: None,
                limits: Limits::default(),
                result_id: None,
                legend: &legend,
                files: &files,
                warnings: &Warnings::default(),
                timed_out: false,
            }
            .to_text()
        };
        assert_eq!(
            response(Highlight::Markers("⟦", "⟧")),
            "files: 1  lines: 1  matches: 1\n#1 text has 'b'\n\nf  [#1]  lines: 1  matches: 1\n1:a ⟧ ⟦#1:b⟧\n\nwarning: marker_in_content (1): f\n"
        );
        assert!(!response(Highlight::Markers("<", ">")).contains("warning"));
        assert!(!response(Highlight::Columns).contains("warning"));
    }

    #[test]
    #[cfg(unix)]
    fn absolute_paths() {
        assert_eq!(display_path(Path::new("/tmp/x/a.rs"), false), "/tmp/x/a.rs");
        assert_eq!(
            split(Path::new("/tmp/a.rs")),
            ("/tmp/".to_string(), "a.rs".to_string())
        );
        assert_eq!(
            split(Path::new("/a.rs")),
            ("/".to_string(), "a.rs".to_string())
        );
    }

    #[test]
    fn only_the_first_files_are_listed() {
        let files = [found("a"), found("b"), found("c")];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let response = |max_files: usize, timed_out: bool| {
            Response {
                detail: Detail::Files,
                blocks: BlockOptions::default(),
                group_dirs: false,
                group_repeated: 3,
                highlight: Highlight::Markers("⟦", "⟧"),
                time_zone: jiff::tz::TimeZone::UTC,
                page: Page::default(),
                funnel: None,
                hints: Vec::new(),
                facets: Vec::new(),
                facet_limit: 8,
                budget: None,
                limits: Limits {
                    max_files,
                    ..Limits::default()
                },
                result_id: None,
                legend: &legend,
                files: &files,
                warnings: &Warnings::default(),
                timed_out,
            }
            .to_text()
        };
        assert_eq!(
            response(2, false),
            "files: 3  lines: 0  matches: 0  truncated: yes (max_files)\n#1 name ~ '*'\n\na  [#1]\nb  [#1]\n"
        );
        assert!(response(3, false).starts_with("files: 3  lines: 0  matches: 0\n"));
        assert!(
            response(1, true).starts_with(
                "files: 3  lines: 0  matches: 0  truncated: yes (max_files, timeout)\n"
            )
        );
    }

    #[test]
    fn an_empty_result_has_only_the_header() {
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &[(CondId::Number(1), "x".into())],
            files: &[],
            warnings: &Warnings::default(),
            timed_out: false,
        }
        .to_text();
        assert_eq!(text, "files: 0  lines: 0  matches: 0  result: r1\n");
    }
}
