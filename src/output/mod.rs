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
pub mod omit;
mod repeated;
mod sort;

pub use blocks::BlockOptions;
pub use budget::fit;
pub use funnel::Hint;
use listing::{Listed, Listing};
pub use sort::{Sort, sort_files};

pub use lines::{Fragment, SubMatch, matched_lines, merge_identical};
pub use omit::{Omit, Part};

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

/// The grain of the counts of `detail = count` (OUTPUT.md#counts-per-file-and-per-line).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CountBy {
    /// The totals.
    Total,
    /// The totals and the counts of every file.
    File,
    /// The totals, the counts of every file and of its matched lines.
    Line,
}

impl CountBy {
    /// The value as written in the configuration.
    #[must_use]
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Total => "total",
            Self::File => "file",
            Self::Line => "line",
        }
    }
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
    /// `auto` in text: the markers, except in the lines whose markers would add more characters than they have.
    Auto(&'a str, &'a str),
}

impl<'a> Highlight<'a> {
    /// The opening and the closing marker of `markers` and `auto`.
    pub fn markers(self) -> Option<[&'a str; 2]> {
        match self {
            Self::Markers(open, close) | Self::Auto(open, close) => Some([open, close]),
            Self::None | Self::Columns => None,
        }
    }
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
    /// `max_output` is the cap of a response written to a file, `server.max_output_file_size`.
    pub file_cap: bool,
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
            file_cap: false,
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
    /// The `count_by` lower than the requested one, if it is.
    pub count_by: Option<CountBy>,
}

/// The change of a listed file to the result of `since` (OUTPUT.md#changes-since).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Added,
    Removed,
    Changed,
}

impl Change {
    /// The mark before the path in the text format.
    fn mark(self) -> char {
        match self {
            Self::Added => '+',
            Self::Removed => '-',
            Self::Changed => '~',
        }
    }

    /// The change in JSON.
    fn keyword(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
            Self::Changed => "changed",
        }
    }
}

/// The differences to a kept result: the response lists the changed files only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Since {
    /// The id of the kept result.
    pub id: String,
    /// The files which match as before.
    pub unchanged: usize,
    /// The change of every file of the response, in their order.
    pub changes: Vec<Change>,
    /// The counts of the current result: files, matched lines, fragments.
    pub totals: [usize; 3],
}

/// The response of a `find` call.
pub struct Response<'a> {
    /// How much it returns.
    pub detail: Detail,
    /// The grain of the counts in `count` detail.
    pub count_by: CountBy,
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
    /// With `since`: the files are the differences to a kept result.
    pub since: Option<Since>,
    /// `None` when the result cache is off.
    pub result_id: Option<&'a str>,
    pub legend: &'a [(CondId, String)],
    pub files: &'a [Found],
    /// Printed after the files.
    pub warnings: &'a Warnings,
    /// The search was stopped by the timeout: `truncated: yes (timeout)`.
    pub timed_out: bool,
    /// The parts left out (OUTPUT.md#omit).
    pub omit: Omit,
    /// `path_style = in_dir`: the working directories as given, by their index; the paths of the files are without
    /// them, with one working directory the warnings too.
    pub in_dirs: Option<&'a [std::path::PathBuf]>,
}

/// The separator of output paths on this platform.
const SEP: char = std::path::MAIN_SEPARATOR;

/// The locations printed for a repeated line.
const LOCATIONS: usize = 3;

impl Response<'_> {
    /// The matched lines and the fragments of all files.
    fn totals(&self) -> (usize, usize) {
        if let Some(since) = &self.since {
            return (since.totals[1], since.totals[2]);
        }
        self.files
            .iter()
            .filter_map(|file| file.matches.as_ref())
            .fold((0, 0), |(lines, matches), m| {
                (lines + m.lines, matches + m.fragments.len())
            })
    }

    /// The number of matched files: of the current result with `since`.
    fn file_count(&self) -> usize {
        self.since
            .as_ref()
            .map_or(self.files.len(), |s| s.totals[0])
    }

    /// The change of the listed file `index` to the result of `since`.
    fn change(&self, index: usize) -> Option<Change> {
        let since = self.since.as_ref()?;
        since.changes.get(self.page_start() + index).copied()
    }

    /// The text format.
    #[must_use]
    pub fn to_text(&self) -> String {
        let repeated = self.repeated();
        let listing = self.listing(&repeated);
        let header = self.header(&listing);
        let mut out = if header.is_empty() {
            String::new()
        } else {
            format!("{header}\n")
        };
        if let Some(dir) = self.in_line() {
            let _ = writeln!(out, "in: {dir}");
        }
        if self.detail == Detail::Exists {
            self.push_warnings(&mut out);
            return out;
        }
        if self.file_count() == 0 || !self.lists() && self.detail == Detail::Count {
            self.push_funnel(&mut out);
            self.push_warnings(&mut out);
            return out;
        }
        if !self.omit.has(Part::Legend) && self.detail != Detail::Count {
            for (id, text) in self.legend {
                let _ = writeln!(out, "{id} {text}");
            }
        }
        self.push_funnel(&mut out);
        if self.detail == Detail::Facets {
            for facet in self.facets.iter().filter(|f| !f.is_empty()) {
                let _ = writeln!(out, "{}", facet.line(self.facet_limit));
            }
            self.push_warnings(&mut out);
            return out;
        }
        if self.files.is_empty() {
            // `since` without differences
            self.push_warnings(&mut out);
            return out;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        self.push_repeated(&mut out, &repeated);
        let mut current: Option<String> = None;
        let mut root: Option<usize> = None;
        for (index, file) in self.listed().iter().enumerate().take(listing.shown) {
            if listing.files[index] == Listed::LeftOut {
                continue;
            }
            if let Some(dir) = self.dir_line(file, &mut root) {
                let _ = writeln!(out, "in: {dir}");
                current = None;
            }
            self.push_file(&mut out, file, &listing, index, &mut current);
        }
        self.push_warnings(&mut out);
        out
    }

    /// The header line of the text format: `exists: yes`, or the counts, the result id, the detail lowered by the
    /// budget and the differences of `since`; then the truncation and the next cursor. The parts left out are not
    /// in it; empty when nothing is left.
    #[must_use]
    pub fn header_line(&self) -> String {
        self.header(&self.listing(&self.repeated()))
    }

    /// The header line with the listing of the response.
    fn header(&self, listing: &Listing) -> String {
        let mut parts = Vec::new();
        if self.detail == Detail::Exists {
            let exists = if self.file_count() == 0 { "no" } else { "yes" };
            parts.push(format!("exists: {exists}"));
        } else {
            if !self.omit.has(Part::Counts) {
                let (lines, matches) = self.totals();
                parts.push(format!(
                    "files: {}  lines: {lines}  matches: {matches}",
                    self.file_count()
                ));
            }
            if let Some(id) = self.result_id.filter(|_| !self.omit.has(Part::Result)) {
                parts.push(format!("result: {id}"));
            }
            if let Some(budget) = self.budget.filter(|b| b.lowered) {
                parts.push(format!(
                    "detail: {} (budget {})",
                    self.detail.keyword(),
                    budget.tokens
                ));
            }
            if let Some((count_by, budget)) = self.budget.and_then(|b| Some((b.count_by?, b))) {
                parts.push(format!(
                    "count by: {} (budget {})",
                    count_by.keyword(),
                    budget.tokens
                ));
            }
            if let Some(since) = &self.since {
                parts.push(format!(
                    "since: {}  unchanged: {}",
                    since.id, since.unchanged
                ));
            }
        }
        let truncated_by = self.truncated_by(listing);
        if !truncated_by.is_empty() {
            parts.push(format!("truncated: yes ({})", truncated_by.join(", ")));
        }
        if let Some(cursor) = self.next_cursor(listing) {
            parts.push(format!("next cursor: {cursor}"));
        }
        parts.join("  ")
    }

    /// The working directories written in `in:` lines: with `path_style = in_dir`, unless `dir` is left out.
    fn shown_dirs(&self) -> Option<&[std::path::PathBuf]> {
        self.in_dirs.filter(|_| !self.omit.has(Part::Dir))
    }

    /// The working directory of the `in:` line after the header: the only one, other than `.`, in a response which
    /// writes a path (OUTPUT.md#the-working-directory-once).
    fn in_line(&self) -> Option<String> {
        let [dir] = self.shown_dirs()? else {
            return None;
        };
        let listed = self.file_count() > 0 && (self.lists() || self.detail == Detail::Facets);
        let paths = listed || self.all_warnings().has_paths();
        (paths && !display_path(dir, false).is_empty()).then(|| dir_name(dir))
    }

    /// With several working directories the `in:` line before a file when its working directory differs from the one
    /// of the previous file, `root`.
    fn dir_line(&self, file: &Found, root: &mut Option<usize>) -> Option<String> {
        let dirs = self.shown_dirs().filter(|dirs| dirs.len() > 1)?;
        if *root == Some(file.root) {
            return None;
        }
        *root = Some(file.root);
        dirs.get(file.root).map(|dir| dir_name(dir))
    }

    /// The path of a listed file in the locations of the repeated lines: with several working directories under
    /// `in_dir` its working directory before it again.
    fn location_path(&self, file: &Found) -> String {
        match self.in_dirs.filter(|dirs| dirs.len() > 1) {
            Some(dirs) => {
                let dir = dirs
                    .get(file.root)
                    .map_or_else(Default::default, |d| d.join(&file.rel));
                let mut full = Found::path_only(dir, file.member.clone());
                full.is_dir = file.is_dir;
                display_found(&full)
            }
            None => display_found(file),
        }
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
        let change = self.change(index);
        if let Some(change) = change {
            let _ = write!(out, "{} {}", change.mark(), display_found(file));
            if change == Change::Removed {
                out.push('\n');
                return;
            }
        } else if self.group_dirs {
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
        if !file.conditions.is_empty()
            && !self.omit.has(Part::Conditions)
            && self.detail != Detail::Count
        {
            let ids: Vec<String> = file.conditions.iter().map(ToString::to_string).collect();
            let _ = write!(out, "  [{}]", ids.join(" "));
        }
        if let Some(m) = file
            .matches
            .as_ref()
            .filter(|_| !self.omit.has(Part::FileCounts))
        {
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
            if self.detail == Detail::Count {
                self.push_line_counts(out, &m.content, fragments);
                return;
            }
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
                self.omit,
            ));
        }
    }

    /// The counts of the listed lines of a file in `count_by = line`: `  42: 2`, without the parts left out.
    fn push_line_counts(&self, out: &mut String, content: &Content, fragments: &[Fragment]) {
        let numbers = !self.omit.has(Part::LineNumbers);
        let counts = !self.omit.has(Part::LineCounts);
        if !numbers && !counts {
            return;
        }
        for (line, count) in lines::line_counts(content, fragments) {
            out.push_str("  ");
            if numbers {
                let _ = write!(out, "{}", line + 1);
            }
            if numbers && counts {
                out.push_str(": ");
            }
            if counts {
                let _ = write!(out, "{count}");
            }
            out.push('\n');
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
            // the fragments of a path keep their markers with `auto` too
            Highlight::Markers(open, close) | Highlight::Auto(open, close) => {
                let markers = Highlight::Markers(open, close);
                let _ = writeln!(
                    out,
                    "  path: {}",
                    lines::shown_line(&content, &path.fragments, 0, 0, markers)
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

    /// Whether the files are listed: in `files` and `lines` detail, in `count` detail with `count_by` other than
    /// `total`.
    fn lists(&self) -> bool {
        self.detail >= Detail::Files
            || self.detail == Detail::Count && self.count_by != CountBy::Total
    }

    /// The files which are listed: the first `max_files` from the start of the page.
    fn listed(&self) -> &[Found] {
        if self.lists() {
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
        self.lists() && self.files.len() - self.page_start() > self.files_limit()
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
            let after = (start + self.files_limit(), 0);
            next = Some(next.map_or(after, |n| n.min(after)));
        }
        let (file, fragment) = next.filter(|(file, _)| *file < self.files.len())?;
        let (lines, matches) = self.totals();
        Some(
            crate::cursor::Cursor {
                hash: hash.clone(),
                file,
                fragment,
                totals: [self.file_count(), lines, matches],
            }
            .encode(),
        )
    }

    /// The limits which were hit, in the order of the limits table.
    fn truncated_by(&self, listing: &Listing) -> Vec<&'static str> {
        let mut by = Vec::new();
        if self.lists() && self.files.len() - self.page_start() > self.limits.max_files {
            by.push("max_files");
        }
        by.extend(listing.truncated_by.iter().map(|&limit| {
            if limit == "max_output" && self.limits.file_cap {
                "max_output_file_size"
            } else {
                limit
            }
        }));
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
                Some(&self.blocks),
                self.page.fragment,
            )
        } else if self.detail == Detail::Count && self.count_by == CountBy::Line {
            Listing::new(
                self.listed(),
                repeated,
                &self.limits,
                None,
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
            .map(|&(file, i)| format!("{}:{}", self.location_path(&self.files[file]), i + 1))
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

    /// The warnings at the end: after an empty line when something is written before them.
    fn push_warnings(&self, out: &mut String) {
        let text = self.warnings_text();
        if out.is_empty() {
            out.push_str(text.trim_start_matches('\n'));
        } else {
            out.push_str(&text);
        }
    }

    /// The warnings at the end of the text format: an empty line and a line per warning; empty without warnings.
    #[must_use]
    pub fn warnings_text(&self) -> String {
        let warnings = self.all_warnings();
        if warnings.is_empty() {
            String::new()
        } else {
            format!("\n{}", warnings.to_text())
        }
    }

    /// The warnings of the response: of the search, `marker_in_content` and `output_file_truncated` when the cap of a
    /// file cut the listing.
    fn all_warnings(&self) -> Cow<'_, Warnings> {
        if self.omit.has(Part::Warnings) {
            return Cow::Owned(Warnings::default());
        }
        let mut warnings = self.marker_warnings();
        if self.limits.file_cap
            && self
                .listing(&self.repeated())
                .truncated_by
                .contains(&"max_output")
        {
            warnings
                .to_mut()
                .add_global(WarningCode::OutputFileTruncated);
        }
        warnings
    }

    /// The warnings of the search and, in `lines` detail with markers, `marker_in_content` for the files with a
    /// matched line which contains a marker; with `auto` a line written without markers doesn't count.
    fn marker_warnings(&self) -> Cow<'_, Warnings> {
        let Some([open, close]) = self.highlight.markers() else {
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
            let plain = if let Highlight::Auto(..) = self.highlight {
                let blocks = blocks::blocks(&m.content, &m.fragments, &self.blocks);
                lines::plain_lines_of(
                    &m.content,
                    &m.fragments,
                    &blocks,
                    [open, close],
                    self.limits.max_line_length,
                )
            } else {
                Vec::new()
            };
            let marked = lines::lines_of(&m.content, &m.fragments)
                .into_iter()
                .filter(|i| !plain.contains(i))
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

/// A working directory in an `in:` line: as given with the separator at the end, `./` for `.`.
fn dir_name(dir: &Path) -> String {
    let name = display_path(dir, true);
    if name.len() == SEP.len_utf8() && !dir.has_root() {
        format!(".{SEP}")
    } else {
        name
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
pub(crate) fn split(rel: &Path) -> (String, String) {
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
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: Some("r3"),
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
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
    fn no_line_for_a_facet_without_values() {
        let files = [found("src/a.rs")];
        let facet = |name: &str, values: &[(&str, usize)]| facets::Facet {
            name: name.to_string(),
            values: values.iter().map(|(v, n)| ((*v).to_string(), *n)).collect(),
        };
        let response = Response {
            detail: Detail::Facets,
            count_by: CountBy::Total,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: vec![
                facet("ext", &[("rs", 1)]),
                facet("git", &[]),
                facet("lang", &[("rust", 1)]),
            ],
            facet_limit: 8,
            budget: None,
            since: None,
            limits: Limits::default(),
            result_id: None,
            legend: &[],
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
        };
        assert_eq!(
            response.to_text(),
            "files: 1  lines: 0  matches: 0\next:        rs 1\nlang:       rust 1\n"
        );
        assert!(facet("git", &[]).is_empty());
    }

    #[test]
    fn full_paths_without_groups() {
        let files = [found("Cargo.toml"), found("src/a.rs")];
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let text = Response {
            detail: Detail::Lines,
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
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
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &[],
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
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
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
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
                    groups: Vec::new(),
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
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
        }
        .to_text();
        assert_eq!(
            text,
            "files: 4  lines: 4  matches: 4\n#1 text has 'x'\n\nrepeated:\n  ×4 use ⟦#1:x⟧;    a:1, b:1, c:1, … 1 more\n"
        );
    }

    /// The repeated line `use x;` of four files with `repeated_locations`, `max_output` and the parts left out.
    fn repeated_text(locations: usize, max_output: usize, omit: &[Part]) -> String {
        let files: Vec<Found> = ["a", "b", "c", "d"]
            .iter()
            .map(|name| Found {
                matches: Some(crate::search::FileMatches {
                    content: crate::content::Content::from_bytes(b"use x;\n"),
                    fragments: vec![Fragment {
                        range: 4..5,
                        ids: vec![(1, CondId::Number(1))],
                        escape: None,
                        groups: Vec::new(),
                    }],
                    lines: 1,
                }),
                ..found(name)
            })
            .collect();
        Response {
            detail: Detail::Lines,
            count_by: CountBy::Total,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            repeated_locations: locations,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page::default(),
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            since: None,
            limits: Limits {
                max_output,
                ..Limits::default()
            },
            result_id: None,
            legend: &[],
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: Omit::from_parts(omit),
            in_dirs: None,
        }
        .to_text()
    }

    #[test]
    fn the_locations_of_a_repeated_line() {
        let head = "files: 4  lines: 4  matches: 4\n\nrepeated:\n  ×4 use ⟦#1:x⟧;";
        assert_eq!(repeated_text(2, usize::MAX, &[]), format!("{head}    a:1, b:1, … 2 more\n"));
        assert_eq!(
            repeated_text(usize::MAX, usize::MAX, &[]),
            format!("{head}    a:1, b:1, c:1, d:1\n")
        );
        assert_eq!(repeated_text(usize::MAX, usize::MAX, &[Part::Locations]), format!("{head}\n"));
        // "a:1, b:1" is 8 characters
        assert_eq!(
            repeated_text(usize::MAX, 8, &[]),
            "files: 4  lines: 4  matches: 4  truncated: yes (max_output)\n\nrepeated:\n  ×4 use ⟦#1:x⟧;    a:1, b:1, … 2 more\n"
        );
        assert_eq!(
            repeated_text(usize::MAX, 2, &[]),
            "files: 4  lines: 4  matches: 4  truncated: yes (max_output)\n\nrepeated:\n  ×4 use ⟦#1:x⟧;    … 4 more\n"
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
                    groups: Vec::new(),
                }],
                lines: 1,
            }),
            ..found("f")
        }];
        let legend = [(CondId::Number(1), "text has 'b'".to_string())];
        let response = |highlight: Highlight| {
            Response {
                detail: Detail::Lines,
                count_by: CountBy::Total,
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
                since: None,
                limits: Limits::default(),
                result_id: None,
                legend: &legend,
                files: &files,
                warnings: &Warnings::default(),
                timed_out: false,
                omit: crate::output::Omit::default(),
                in_dirs: None,
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
                count_by: CountBy::Total,
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
                since: None,
                limits: Limits {
                    max_files,
                    ..Limits::default()
                },
                result_id: None,
                legend: &legend,
                files: &files,
                warnings: &Warnings::default(),
                timed_out,
                omit: Omit::default(),
                in_dirs: None,
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
            count_by: CountBy::Total,
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
            since: None,
            limits: Limits::default(),
            result_id: Some("r1"),
            legend: &[(CondId::Number(1), "x".into())],
            files: &[],
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
        }
        .to_text();
        assert_eq!(text, "files: 0  lines: 0  matches: 0  result: r1\n");
    }

    /// The file where the next page starts: a list of 12 files from the third one, cut to `budget_files`.
    fn next_file(max_files: usize, budget_files: usize) -> Option<usize> {
        let files: Vec<Found> = (0..12).map(|i| found(&format!("f{i:02}.rs"))).collect();
        let legend = [(CondId::Number(1), "name ~ '*'".to_string())];
        let response = Response {
            detail: Detail::Files,
            count_by: CountBy::Total,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Markers("⟦", "⟧"),
            time_zone: jiff::tz::TimeZone::UTC,
            page: Page {
                file: 2,
                fragment: 0,
                hash: Some("0123abcd".to_string()),
            },
            funnel: None,
            hints: Vec::new(),
            facets: Vec::new(),
            facet_limit: 8,
            budget: None,
            since: None,
            limits: Limits {
                max_files,
                budget_files,
                ..Limits::default()
            },
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: crate::output::Omit::default(),
            in_dirs: None,
        };
        let listing = Listing::without_lines(response.listed(), &response.limits);
        let cursor = response.next_cursor(&listing)?;
        Some(
            crate::cursor::Cursor::decode(&cursor)
                .expect("a cursor the server made")
                .file,
        )
    }

    /// The text of `count_by = line` for a file with two matches in line 1 and a multiline match from line 3 to 4.
    fn line_counts(omit: &[Part]) -> String {
        let content = crate::content::Content::from_bytes(b"a x x\nb\nx\ny\n");
        let fragment = |start: usize, end: usize| Fragment {
            range: start..end,
            ids: vec![(1, CondId::Number(1))],
            escape: None,
            groups: Vec::new(),
        };
        let fragments = vec![fragment(2, 3), fragment(4, 5), fragment(8, 11)];
        let lines = matched_lines(&content, &fragments);
        let files = [Found {
            matches: Some(crate::search::FileMatches {
                content,
                fragments,
                lines,
            }),
            ..found("f")
        }];
        let legend = [(CondId::Number(1), "text has 'x'".to_string())];
        Response {
            detail: Detail::Count,
            count_by: CountBy::Line,
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
            since: None,
            limits: Limits::default(),
            result_id: None,
            legend: &legend,
            files: &files,
            warnings: &Warnings::default(),
            timed_out: false,
            omit: Omit::from_parts(omit),
            in_dirs: None,
        }
        .to_text()
    }

    #[test]
    fn the_counts_of_the_lines() {
        let head = "files: 1  lines: 3  matches: 3\n\nf  lines: 3  matches: 3\n";
        assert_eq!(line_counts(&[]), format!("{head}  1: 2\n  3: 1\n  4: 0\n"));
        assert_eq!(
            line_counts(&[Part::LineNumbers]),
            format!("{head}  2\n  1\n  0\n")
        );
        assert_eq!(
            line_counts(&[Part::LineCounts]),
            format!("{head}  1\n  3\n  4\n")
        );
        assert_eq!(line_counts(&[Part::LineNumbers, Part::LineCounts]), head);
    }

    #[test]
    fn the_cursor_of_a_budget_list_starts_after_the_files_listed() {
        // max_files reaches past the end, the budget cuts the list
        assert_eq!(next_file(100, 4), Some(6));
        // max_files is inside the list, the budget cuts it shorter
        assert_eq!(next_file(8, 4), Some(6));
        // max_files alone
        assert_eq!(next_file(8, usize::MAX), Some(10));
        // everything fits
        assert_eq!(next_file(100, usize::MAX), None);
    }
}
