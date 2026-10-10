//! The JSON format of the response (OUTPUT.md#json-format): the fields in the order of the fields table, empty and
//! default values left out, written without whitespace.

use serde::Serialize;
use serde::ser::SerializeMap;

use super::blocks::BlockLine;
use super::lines::{columns, cut_line, starts_in};
use super::listing::Listed;
use super::{BlockOptions, Detail, Highlight, LOCATIONS, Response, display_found};
use crate::content::Content;
use crate::query::Kind;
use crate::query::ast::CondId;
use crate::search::{Found, PathFragments};

/// The version of the JSON response (TOOLS.md#versioning).
const VERSION: u32 = 1;

#[derive(Serialize)]
struct Document<'a> {
    version: u32,
    exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    repeated: Vec<Repeated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts: Option<Counts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncated: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    truncated_by: Vec<&'static str>,
    #[serde(skip_serializing_if = "Legend::is_empty")]
    legend: Legend<'a>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    warnings: Vec<Warning>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    files: Vec<File>,
}

/// A matched line repeated in many files (OUTPUT.md#repeated-lines).
#[derive(Serialize)]
struct Repeated {
    count: usize,
    text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fragments: Vec<Fragment>,
    locations: Vec<String>,
}

#[derive(Serialize)]
struct Counts {
    files: usize,
    lines: usize,
    matches: usize,
}

/// The legend: a map in the order of the query.
struct Legend<'a>(&'a [(CondId, String)]);

impl Legend<'_> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for Legend<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (id, text) in self.0 {
            map.serialize_entry(&id.to_string(), text)?;
        }
        map.end()
    }
}

#[derive(Serialize)]
struct Warning {
    code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    paths: Vec<String>,
}

#[derive(Serialize)]
struct File {
    path: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    entry_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    truncated: Option<bool>,
    #[serde(skip_serializing_if = "is_zero")]
    lines: usize,
    #[serde(skip_serializing_if = "is_zero")]
    matches: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    path_fragments: Vec<Fragment>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // the signature of `skip_serializing_if`
fn is_zero(n: &usize) -> bool {
    *n == 0
}

#[derive(Serialize)]
struct Block {
    #[serde(skip_serializing_if = "is_zero")]
    indent: usize,
    lines: Vec<Line>,
}

#[derive(Serialize)]
struct Line {
    n: usize,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    cut: bool,
    text: String,
    /// `highlight = none`: a matched line.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    matched: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fragments: Vec<Fragment>,
}

#[derive(Serialize)]
struct Fragment {
    col: [usize; 2],
    text: String,
    ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    escape: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<To>,
}

#[derive(Serialize)]
struct To {
    n: usize,
    col: usize,
}

impl Response<'_> {
    /// The JSON format.
    #[must_use]
    pub fn to_json(&self) -> String {
        let (lines, matches) = self.totals();
        let counted = self.detail >= Detail::Count;
        let listed = self.detail >= Detail::Files;
        let repeated = self.repeated();
        let listing = self.listing(&repeated);
        let truncated_by = self.truncated_by(&listing);
        let document = Document {
            version: VERSION,
            exists: !self.files.is_empty(),
            result: self.result_id,
            repeated: repeated
                .lines
                .iter()
                .map(|line| {
                    let shown = json_line(
                        &line.content,
                        &line.fragments,
                        BlockLine {
                            index: 0,
                            matched: true,
                        },
                        0,
                        self.highlight,
                        usize::MAX,
                    );
                    Repeated {
                        count: line.files.len(),
                        text: shown.text,
                        fragments: shown.fragments,
                        locations: self.locations(line).into_iter().take(LOCATIONS).collect(),
                    }
                })
                .collect(),
            counts: counted.then_some(Counts {
                files: self.files.len(),
                lines,
                matches,
            }),
            counts_exact: (counted && self.timed_out).then_some(false),
            truncated: (!truncated_by.is_empty()).then_some(true),
            truncated_by,
            legend: Legend(if listed && !self.files.is_empty() {
                self.legend
            } else {
                &[]
            }),
            warnings: self
                .all_warnings()
                .groups()
                .into_iter()
                .map(|(code, count, paths)| Warning { code, count, paths })
                .collect(),
            files: if listed {
                self.listed()
                    .iter()
                    .zip(&listing.files)
                    .zip(&listing.cut)
                    .filter_map(|((file, listed), cut)| match listed {
                        Listed::LeftOut => None,
                        Listed::NoLines => Some(self.file(file, None, *cut)),
                        Listed::Lines(fragments) => Some(self.file(file, Some(fragments), *cut)),
                    })
                    .collect()
            } else {
                Vec::new()
            },
        };
        serde_json::to_string(&document).expect("the response serializes")
    }
}

impl Response<'_> {
    /// A matched file, with the blocks of the fragments `lines` or without lines; `cut` by a per file limit.
    fn file(&self, found: &Found, lines: Option<&[super::Fragment]>, cut: bool) -> File {
        let meta = found.meta.as_ref();
        File {
            path: display_found(found),
            entry_type: meta.map(|m| m.entry_type.keyword()),
            size: meta.and_then(|m| m.size),
            lang: meta.and_then(|m| m.lang.clone()),
            kind: meta.and_then(|m| m.kind.map(Kind::keyword)),
            #[allow(clippy::cast_precision_loss)] // whole seconds of dates are exact
            modified: meta
                .and_then(|m| m.modified)
                .map(|secs| super::meta::date_json(secs as f64, &self.time_zone)),
            encoding: found.encoding.map(|e| e.name().into_owned()),
            score: meta
                .and(found.score)
                .map(crate::relevance::shown_score),
            truncated: cut.then_some(true),
            lines: found.matches.as_ref().map_or(0, |m| m.lines),
            matches: found.matches.as_ref().map_or(0, |m| m.fragments.len()),
            conditions: found.conditions.iter().map(ToString::to_string).collect(),
            path_fragments: found
                .path_fragments
                .as_ref()
                .map(path_fragments)
                .unwrap_or_default(),
            blocks: match (&found.matches, lines) {
                (Some(m), Some(fragments)) => blocks(
                    &m.content,
                    fragments,
                    &self.blocks,
                    self.highlight,
                    self.limits.max_line_length,
                ),
                _ => Vec::new(),
            },
        }
    }
}

/// The path fragments with their columns in the path (OUTPUT.md#path-fragments).
fn path_fragments(path: &PathFragments) -> Vec<Fragment> {
    let content = Content::from_text(path.path.clone());
    path.fragments
        .iter()
        .filter(|f| starts_in(&content, f, 0))
        .map(|f| fragment(&content, f, 0))
        .collect()
}

/// A fragment which starts in the line `i`.
fn fragment(content: &Content, f: &super::Fragment, i: usize) -> Fragment {
    let at = columns(content, f, i);
    Fragment {
        col: [at.start, at.end],
        text: content.text()[f.range.clone()].to_string(),
        ids: f.ids.iter().map(|(_, id)| id.to_string()).collect(),
        escape: f.escape,
        to: at.to.map(|(n, col)| To { n, col }),
    }
}

/// The blocks of the matched lines with their context, every fragment in the line where it starts.
fn blocks(
    content: &Content,
    fragments: &[super::Fragment],
    options: &BlockOptions,
    highlight: Highlight<'_>,
    max_line_length: usize,
) -> Vec<Block> {
    super::blocks::blocks(content, fragments, options)
        .into_iter()
        .map(|block| Block {
            indent: block.indent,
            lines: block
                .lines
                .into_iter()
                .map(|line| {
                    json_line(
                        content,
                        fragments,
                        line,
                        block.indent,
                        highlight,
                        max_line_length,
                    )
                })
                .collect(),
        })
        .collect()
}

/// A line of a block without its first `indent` bytes in the highlight style: with the fragments which start in it
/// (their columns of the original line), with the markers in its text or with `match`; cut when it is longer than
/// `max_line_length` characters.
fn json_line(
    content: &Content,
    fragments: &[super::Fragment],
    line: BlockLine,
    indent: usize,
    highlight: Highlight<'_>,
    max_line_length: usize,
) -> Line {
    let i = line.index;
    let (text, cut) = cut_line(content, fragments, i, indent, highlight, max_line_length);
    Line {
        n: i + 1,
        cut,
        text,
        matched: (highlight == Highlight::None && line.matched).then_some(true),
        fragments: if highlight == Highlight::Columns {
            fragments
                .iter()
                .filter(|f| starts_in(content, f, i))
                .map(|f| fragment(content, f, i))
                .collect()
        } else {
            Vec::new()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::Content;
    use crate::query::ast::CondId;

    fn frag(range: std::ops::Range<usize>, id: u32, escape: Option<u8>) -> super::super::Fragment {
        super::super::Fragment {
            range,
            ids: vec![(id as usize, CondId::Number(id))],
            escape,
        }
    }

    #[test]
    fn fragments_in_columns_of_characters() {
        let c = Content::from_bytes("é = x\nab\ncd\n".as_bytes());
        let x = c.text().find('x').unwrap();
        let json = serde_json::to_value(blocks(
            &c,
            &[frag(x..x + 1, 1, Some(0))],
            &BlockOptions::default(),
            Highlight::Columns,
            usize::MAX,
        ))
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!([{ "lines": [ { "n": 1, "text": "é = x",
                "fragments": [ { "col": [5, 6], "text": "x", "ids": ["#1"], "escape": 0 } ] } ] }])
        );
        let a = c.text().find('a').unwrap();
        let d = c.text().find('d').unwrap();
        let json = serde_json::to_value(blocks(
            &c,
            &[frag(a..d + 1, 2, None)],
            &BlockOptions::default(),
            Highlight::Columns,
            usize::MAX,
        ))
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!([{ "lines": [
                { "n": 2, "text": "ab",
                  "fragments": [ { "col": [1, 3], "text": "ab\ncd", "ids": ["#2"], "to": { "n": 3, "col": 3 } } ] },
                { "n": 3, "text": "cd" }
            ] }])
        );
    }

    #[test]
    fn blocks_of_consecutive_lines() {
        let c = Content::from_bytes(b"a\nb\nc\na\n");
        let fragments: Vec<_> = c
            .text()
            .match_indices('a')
            .map(|(i, _)| frag(i..i + 1, 1, None))
            .collect();
        let blocks = blocks(
            &c,
            &fragments,
            &BlockOptions::default(),
            Highlight::Columns,
            usize::MAX,
        );
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[1].lines[0].n, 4);
    }

    #[test]
    fn the_order_of_the_fields() {
        let legend = [(CondId::Number(1), "type = dir".to_string())];
        let warnings = crate::warning::Warnings::default();
        let files = [Found {
            rel: "sub".into(),
            member: None,
            is_dir: true,
            conditions: vec![CondId::Number(1)],
            matches: None,
            encoding: None,
            id: None,
            path_fragments: None,
            meta: None,
            signals: None,
            score: None,
        }];
        let text = Response {
            detail: Detail::Lines,
            blocks: BlockOptions::default(),
            group_dirs: true,
            group_repeated: 3,
            highlight: Highlight::Columns,
            time_zone: jiff::tz::TimeZone::UTC,
            limits: super::super::Limits::default(),
            result_id: Some("r1"),
            legend: &legend,
            files: &files,
            warnings: &warnings,
            timed_out: true,
        }
        .to_json();
        let sep = std::path::MAIN_SEPARATOR;
        let path = serde_json::to_string(&format!("sub{sep}")).unwrap();
        assert_eq!(
            text,
            format!(
                r##"{{"version":1,"exists":true,"result":"r1","counts":{{"files":1,"lines":0,"matches":0}},"counts_exact":false,"truncated":true,"truncated_by":["timeout"],"legend":{{"#1":"type = dir"}},"files":[{{"path":{path},"conditions":["#1"]}}]}}"##
            )
        );
    }
}
