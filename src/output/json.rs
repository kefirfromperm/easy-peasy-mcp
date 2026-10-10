//! The JSON format of the response (OUTPUT.md#json-format): the fields in the order of the fields table, empty and
//! default values left out, written without whitespace.

use serde::Serialize;
use serde::ser::SerializeMap;

use super::{Response, display_found, lines::lines_of};
use crate::content::Content;
use crate::query::ast::CondId;
use crate::search::Found;

/// The version of the JSON response (TOOLS.md#versioning).
const VERSION: u32 = 1;

#[derive(Serialize)]
struct Document<'a> {
    version: u32,
    exists: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<&'a str>,
    counts: Counts,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<String>,
    #[serde(skip_serializing_if = "is_zero")]
    lines: usize,
    #[serde(skip_serializing_if = "is_zero")]
    matches: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,
}

#[allow(clippy::trivially_copy_pass_by_ref)] // the signature of `skip_serializing_if`
fn is_zero(n: &usize) -> bool {
    *n == 0
}

#[derive(Serialize)]
struct Block {
    lines: Vec<Line>,
}

#[derive(Serialize)]
struct Line {
    n: usize,
    text: String,
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
        let document = Document {
            version: VERSION,
            exists: !self.files.is_empty(),
            result: self.result_id,
            counts: Counts {
                files: self.files.len(),
                lines,
                matches,
            },
            counts_exact: self.timed_out.then_some(false),
            truncated: self.timed_out.then_some(true),
            truncated_by: if self.timed_out {
                vec!["timeout"]
            } else {
                Vec::new()
            },
            legend: Legend(if self.files.is_empty() {
                &[]
            } else {
                self.legend
            }),
            warnings: self
                .warnings
                .groups()
                .into_iter()
                .map(|(code, count, paths)| Warning { code, count, paths })
                .collect(),
            files: self.files.iter().map(file).collect(),
        };
        serde_json::to_string(&document).expect("the response serializes")
    }
}

/// A matched file.
fn file(found: &Found) -> File {
    File {
        path: display_found(found),
        encoding: found.encoding.map(|e| e.name().into_owned()),
        lines: found.matches.as_ref().map_or(0, |m| m.lines),
        matches: found.matches.as_ref().map_or(0, |m| m.fragments.len()),
        conditions: found.conditions.iter().map(ToString::to_string).collect(),
        blocks: found
            .matches
            .as_ref()
            .map(|m| blocks(&m.content, &m.fragments))
            .unwrap_or_default(),
    }
}

/// The matched lines in blocks of consecutive lines, every fragment in the line where it starts.
fn blocks(content: &Content, fragments: &[super::Fragment]) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut previous: Option<usize> = None;
    for i in lines_of(content, fragments) {
        let (start, end) = content.line_range(i);
        let text = content.line(i);
        let column = |offset: usize| content.text()[start..offset].chars().count() + 1;
        let line = Line {
            n: i + 1,
            text: text.to_string(),
            fragments: fragments
                .iter()
                .filter(|f| content.line_of(f.range.start) == i && f.range.start <= end)
                .map(|f| {
                    let last = content.line_of(f.range.end.saturating_sub(1).max(f.range.start));
                    let multiline = last > i;
                    Fragment {
                        col: [
                            column(f.range.start),
                            if multiline {
                                text.chars().count() + 1
                            } else {
                                column(f.range.end)
                            },
                        ],
                        text: content.text()[f.range.clone()].to_string(),
                        ids: f.ids.iter().map(|(_, id)| id.to_string()).collect(),
                        escape: f.escape,
                        to: multiline.then(|| {
                            let (last_start, _) = content.line_range(last);
                            To {
                                n: last + 1,
                                col: content.text()[last_start..f.range.end].chars().count() + 1,
                            }
                        }),
                    }
                })
                .collect(),
        };
        match blocks.last_mut() {
            Some(block) if previous.is_some_and(|p| p + 1 == i) => block.lines.push(line),
            _ => blocks.push(Block { lines: vec![line] }),
        }
        previous = Some(i);
    }
    blocks
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
        let json = serde_json::to_value(blocks(&c, &[frag(x..x + 1, 1, Some(0))])).unwrap();
        assert_eq!(
            json,
            serde_json::json!([{ "lines": [ { "n": 1, "text": "é = x",
                "fragments": [ { "col": [5, 6], "text": "x", "ids": ["#1"], "escape": 0 } ] } ] }])
        );
        let a = c.text().find('a').unwrap();
        let d = c.text().find('d').unwrap();
        let json = serde_json::to_value(blocks(&c, &[frag(a..d + 1, 2, None)])).unwrap();
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
        let blocks = blocks(&c, &fragments);
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
        }];
        let text = Response {
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
