//! Several queries in one call (OUTPUT.md#batch).

use std::fmt::Write as _;

use serde_json::{Map, Value};

use crate::error::{ErrorCode, Report};
use crate::output_file::{self, Head, Written};
use crate::query::did_you_mean;

/// The fields of an item of `queries`.
const ITEM_FIELDS: [&str; 6] = ["label", "query", "dirs", "timeout", "traversal", "output"];

/// The items of a batch request; `query` and `queries` together or a `queries` which is not a non-empty list of
/// objects is `invalid_option`.
///
/// # Errors
///
/// `invalid_option` as above.
pub fn items(arguments: &Value) -> Result<&[Value], Report> {
    if arguments.get("query").is_some() {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'query' and 'queries' can't be used together",
        ));
    }
    match arguments.get("queries") {
        Some(Value::Array(items)) if !items.is_empty() && items.iter().all(Value::is_object) => {
            Ok(items)
        }
        _ => Err(Report::new(
            ErrorCode::InvalidOption,
            "'queries' must be a non-empty list of objects",
        )),
    }
}

/// The fields of a batch request which all queries share.
#[must_use]
pub fn shared(arguments: &Value) -> Value {
    let mut shared = arguments.clone();
    if let Some(object) = shared.as_object_mut() {
        object.remove("queries");
    }
    shared
}

/// The label of the item `index`: its `label`, or its number from 1.
#[must_use]
pub fn label(item: &Value, index: usize) -> String {
    item.get("label")
        .and_then(Value::as_str)
        .map_or_else(|| (index + 1).to_string(), str::to_string)
}

/// The arguments of the query of an item: the shared fields with its `query`, its `dirs` and `timeout` instead of
/// the shared ones and its `traversal` and `output` merged into the shared ones key by key.
///
/// # Errors
///
/// `invalid_option`: an unknown field, a `label` or `query` which is not a string.
pub fn item_arguments(shared: &Value, item: &Value, spec: u32) -> Result<Value, Report> {
    let empty = Map::new();
    let fields = item.as_object().unwrap_or(&empty);
    if let Some(unknown) = fields.keys().find(|k| !ITEM_FIELDS.contains(&k.as_str())) {
        let expected = format!("expected one of: {}", ITEM_FIELDS.join(", "));
        let hint = match did_you_mean(unknown, &ITEM_FIELDS) {
            Some(close) => format!("did you mean '{close}'? {expected}"),
            None => expected,
        };
        return Err(Report::new(
            ErrorCode::InvalidOption,
            format!("unknown field '{unknown}'"),
        )
        .hint(hint));
    }
    if fields.get("label").is_some_and(|l| !l.is_string()) {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'label' must be a string",
        ));
    }
    if !fields.get("query").is_some_and(Value::is_string) {
        return Err(Report::new(
            ErrorCode::InvalidOption,
            "'query' must be a string",
        ));
    }
    let mut arguments = shared.as_object().cloned().unwrap_or_default();
    for (name, value) in fields {
        match (name.as_str(), arguments.get_mut(name), value) {
            ("label", _, _) => {}
            ("traversal" | "output", Some(Value::Object(shared)), Value::Object(own)) => {
                for (key, value) in own {
                    // spec 2: the `omit` of an item is added to the shared one (OUTPUT.md#defaults-of-spec-2)
                    let merged = match (key.as_str(), shared.get(key)) {
                        ("omit", Some(before)) if spec >= 2 && name == "output" => {
                            omit_union(before, value)
                        }
                        _ => value.clone(),
                    };
                    shared.insert(key.clone(), merged);
                }
            }
            _ => {
                arguments.insert(name.clone(), value.clone());
            }
        }
    }
    Ok(Value::Object(arguments))
}

/// Two values of `omit`: `"all"` when one of them is, the parts of both lists otherwise; a wrong one as it is, for
/// the check of the request.
fn omit_union(before: &Value, own: &Value) -> Value {
    match (before, own) {
        (Value::Array(a), Value::Array(b)) => {
            let mut parts = a.clone();
            parts.extend(b.iter().filter(|p| !a.contains(p)).cloned());
            Value::Array(parts)
        }
        (Value::String(all), Value::Array(_)) if all == "all" => before.clone(),
        _ => own.clone(),
    }
}

/// A JSON document without its `"version"`, the first field: the rest after the opening brace.
fn without_version(document: &str) -> &str {
    document
        .strip_prefix("{\"version\":")
        .and_then(|rest| rest.split_once(','))
        .map_or(document, |(_, rest)| rest)
}

/// The response of a batch: in JSON `{"version":1,"queries":[...]}` with every query's response (without `version`)
/// or its error after its `label`; in the text format the sections `== label ==  <response>`.
#[must_use]
pub fn response(sections: &[(String, Result<String, Report>)], json: bool, version: u32) -> String {
    if json {
        let items: Vec<String> = sections
            .iter()
            .map(|(label, result)| {
                let label = serde_json::to_string(label).expect("a string serializes");
                let body = match result {
                    Ok(text) => without_version(text).to_string(),
                    Err(report) => report
                        .to_json()
                        .strip_prefix('{')
                        .unwrap_or_default()
                        .to_string(),
                };
                format!("{{\"label\":{label},{body}")
            })
            .collect();
        return format!(
            "{{\"version\":{version},\"queries\":[{}]}}",
            items.join(",")
        );
    }
    sections
        .iter()
        .fold(String::new(), |mut out, (label, result)| {
            let body = match result {
                Ok(text) => text.clone(),
                Err(report) => report.to_text(),
            };
            let _ = write!(out, "== {label} ==  {body}");
            out
        })
}

/// The summary of a batch response written to a file (OUTPUT.md#in-a-batch): the first line of every section and the
/// line of the file; in JSON the summary fields of every query after its label, then the fields of the file.
#[must_use]
pub fn summary(
    sections: &[(String, Result<Head, Report>)],
    json: bool,
    written: &Written,
    version: u32,
) -> String {
    if json {
        let items: Vec<String> = sections
            .iter()
            .map(|(label, result)| {
                let label = serde_json::to_string(label).expect("a string serializes");
                let body = match result {
                    Ok(Head::Json(fields)) => without_version(fields).to_string(),
                    // a JSON batch has JSON heads
                    Ok(Head::Text { .. }) => "}".to_string(),
                    Err(report) => report
                        .to_json()
                        .strip_prefix('{')
                        .unwrap_or_default()
                        .to_string(),
                };
                format!("{{\"label\":{label},{body}")
            })
            .collect();
        let document = format!(
            "{{\"version\":{version},\"queries\":[{}]}}",
            items.join(",")
        );
        return output_file::with_file(&document, written);
    }
    let mut out = String::new();
    for (label, result) in sections {
        let first = match result {
            Ok(Head::Text { first, .. }) => first.clone(),
            Ok(Head::Json(fields)) => fields.clone(),
            Err(report) => report
                .to_text()
                .lines()
                .next()
                .unwrap_or_default()
                .to_string(),
        };
        let _ = writeln!(out, "== {label} ==  {first}");
    }
    out.push_str(&written.line());
    out.push('\n');
    out
}

/// How a budget of the whole response is split between the queries: an item with its own value keeps it, the rest is
/// divided equally among the others in their order, and what one of them doesn't use goes to the next ones.
#[derive(Debug, Clone)]
pub struct Split {
    /// What is left for the items without their own value.
    pool: usize,
    own: Vec<Option<usize>>,
    /// The items without their own value which haven't got their part yet.
    waiting: usize,
    /// Whether the last part was an item's own value.
    last_own: bool,
}

impl Split {
    /// The split of `total` between items with their own values `own`.
    #[must_use]
    pub fn new(total: usize, own: &[Option<usize>]) -> Self {
        Self {
            pool: total.saturating_sub(own.iter().flatten().sum()),
            own: own.to_vec(),
            waiting: own.iter().filter(|o| o.is_none()).count(),
            last_own: false,
        }
    }

    /// The part of the item `index`.
    pub fn next(&mut self, index: usize) -> usize {
        if let Some(own) = self.own.get(index).copied().flatten() {
            self.last_own = true;
            return own;
        }
        self.last_own = false;
        let part = self.pool / self.waiting.max(1);
        self.waiting = self.waiting.saturating_sub(1);
        part
    }

    /// What the last item used of its part.
    pub fn used(&mut self, used: usize) {
        if !self.last_own {
            self.pool = self.pool.saturating_sub(used);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn the_arguments_of_an_item() {
        let shared = json!({
            "dirs": ["."],
            "output": { "detail": "files", "context": 1 },
            "traversal": { "hidden": true }
        });
        let item = json!({
            "label": "x",
            "query": "lang = rust",
            "dirs": ["src"],
            "output": { "detail": "count" }
        });
        assert_eq!(
            item_arguments(&shared, &item, 1).unwrap(),
            json!({
                "query": "lang = rust",
                "dirs": ["src"],
                "output": { "detail": "count", "context": 1 },
                "traversal": { "hidden": true }
            })
        );
        assert_eq!(
            item_arguments(&shared, &json!({ "label": 1, "query": "x" }), 1)
                .unwrap_err()
                .to_text(),
            "error[invalid_option]: 'label' must be a string\n"
        );
        assert_eq!(
            item_arguments(&shared, &json!({ "label": "x" }), 1)
                .unwrap_err()
                .to_text(),
            "error[invalid_option]: 'query' must be a string\n"
        );
    }

    #[test]
    fn the_budget_is_split() {
        // 300 tokens for three queries, the second has its own 50
        let mut split = Split::new(300, &[None, Some(50), None]);
        assert_eq!(split.next(0), 125);
        split.used(100);
        assert_eq!(split.next(1), 50);
        split.used(50);
        // the first left 25 of its 125 for the last one
        assert_eq!(split.next(2), 150);
    }
}
