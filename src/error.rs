//! Errors which make a request fail: code, message, position in the query and hint (OUTPUT.md#errors).

use std::fmt;
use std::ops::Range;

/// The error codes of the spec (QUERY.md#errors, OUTPUT.md#errors).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    UnexpectedToken,
    UnexpectedEnd,
    UnknownTarget,
    UnknownOperator,
    OperatorNotAllowed,
    ModifierNotAllowed,
    UnterminatedString,
    InvalidWildcard,
    DuplicateLabel,
    UnknownGroup,
    UnknownValue,
    InvalidSize,
    InvalidRange,
    InvalidDate,
    InvalidRegex,
    FileTargetInside,
    NestedLine,
    LineTargetOutside,
    InvalidNearOption,
    ConflictingOptions,
    InvalidEscapeLevel,
    InvalidOption,
    DirNotFound,
    DirNotAllowed,
    InvalidToml,
    ReadOnly,
    UnknownKey,
    InvalidValue,
    UpdateDisabled,
    CursorMismatch,
    CursorInvalid,
    UnsupportedVersion,
    ResultExpired,
    OutputFileDisabled,
    OutputFileDenied,
    OutputFileFailed,
    WriteDisabled,
    CountMismatch,
    InvalidTemplate,
    RenameConflict,
    CopyConflict,
    /// A cancelled request: its response is never sent (TOOLS.md#cancellation).
    Cancelled,
    NotImplemented,
}

impl ErrorCode {
    /// The code as written in the response.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnexpectedToken => "unexpected_token",
            Self::UnexpectedEnd => "unexpected_end",
            Self::UnknownTarget => "unknown_target",
            Self::UnknownOperator => "unknown_operator",
            Self::OperatorNotAllowed => "operator_not_allowed",
            Self::ModifierNotAllowed => "modifier_not_allowed",
            Self::UnterminatedString => "unterminated_string",
            Self::InvalidWildcard => "invalid_wildcard",
            Self::DuplicateLabel => "duplicate_label",
            Self::UnknownGroup => "unknown_group",
            Self::UnknownValue => "unknown_value",
            Self::InvalidSize => "invalid_size",
            Self::InvalidRange => "invalid_range",
            Self::InvalidDate => "invalid_date",
            Self::InvalidRegex => "invalid_regex",
            Self::FileTargetInside => "file_target_inside",
            Self::NestedLine => "nested_line",
            Self::LineTargetOutside => "line_target_outside",
            Self::InvalidNearOption => "invalid_near_option",
            Self::ConflictingOptions => "conflicting_options",
            Self::InvalidEscapeLevel => "invalid_escape_level",
            Self::InvalidOption => "invalid_option",
            Self::DirNotFound => "dir_not_found",
            Self::DirNotAllowed => "dir_not_allowed",
            Self::InvalidToml => "invalid_toml",
            Self::ReadOnly => "read_only",
            Self::UnknownKey => "unknown_key",
            Self::InvalidValue => "invalid_value",
            Self::UpdateDisabled => "update_disabled",
            Self::CursorMismatch => "cursor_mismatch",
            Self::CursorInvalid => "cursor_invalid",
            Self::UnsupportedVersion => "unsupported_version",
            Self::ResultExpired => "result_expired",
            Self::OutputFileDisabled => "output_file_disabled",
            Self::OutputFileDenied => "output_file_denied",
            Self::OutputFileFailed => "output_file_failed",
            Self::WriteDisabled => "write_disabled",
            Self::CountMismatch => "count_mismatch",
            Self::InvalidTemplate => "invalid_template",
            Self::RenameConflict => "rename_conflict",
            Self::CopyConflict => "copy_conflict",
            Self::Cancelled => "cancelled",
            Self::NotImplemented => "not_implemented",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An error in the query: the byte range of the marked part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryError {
    pub code: ErrorCode,
    pub message: String,
    /// Byte range in the query; an empty range at the end means "after the last character".
    pub span: Range<usize>,
    pub hint: Option<String>,
}

impl QueryError {
    /// A new query error.
    pub fn new(code: ErrorCode, message: impl Into<String>, span: Range<usize>) -> Self {
        Self {
            code,
            message: message.into(),
            span,
            hint: None,
        }
    }

    /// Adds a hint.
    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The report for this error in `query`.
    #[must_use]
    pub fn into_report(self, query: &str) -> Report {
        self.into_field_report("query", query)
    }

    /// The report for this error in the request field `field` (`query`, `to`, `pattern`) whose text is `text`.
    #[must_use]
    pub fn into_field_report(self, field: &'static str, text: &str) -> Report {
        Report {
            code: self.code,
            message: self.message,
            position: Some((text.to_string(), self.span)),
            field,
            details: None,
            hint: self.hint,
        }
    }
}

/// A failed request, rendered as the text of a tool error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub code: ErrorCode,
    pub message: String,
    /// The text of the field and the byte range of the error in it.
    pub position: Option<(String, Range<usize>)>,
    /// The name of the field of the position in JSON: `query`, `to`, `pattern`.
    pub field: &'static str,
    /// The conflicts of `rename_conflict`.
    pub details: Option<Box<Details>>,
    pub hint: Option<String>,
}

/// More of a report: lines after the message and the position, before the hint, and the same in a JSON field after
/// `message`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Details {
    pub lines: Vec<String>,
    pub name: &'static str,
    pub value: serde_json::Value,
}

impl Report {
    /// A request error without a position.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            position: None,
            field: "query",
            details: None,
            hint: None,
        }
    }

    /// Adds a hint.
    #[must_use]
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Adds lines before the hint and the JSON field `name` with the same information.
    #[must_use]
    pub fn details(
        mut self,
        lines: Vec<String>,
        name: &'static str,
        value: serde_json::Value,
    ) -> Self {
        self.details = Some(Box::new(Details { lines, name, value }));
        self
    }

    /// `not_implemented` for a feature outside of the query.
    pub fn not_implemented(feature: &str) -> Self {
        Self::new(
            ErrorCode::NotImplemented,
            format!("{feature} is not implemented yet"),
        )
    }

    /// The JSON format of OUTPUT.md#errors, compact: `{"error": {code, message, query, position, hint}}`, the field of
    /// the position named after it; `col` of the position is 1-based in characters from the start of the field, `len`
    /// in characters, at least 1.
    #[must_use]
    pub fn to_json(&self) -> String {
        let mut error = serde_json::Map::new();
        error.insert("code".into(), self.code.as_str().into());
        error.insert("message".into(), self.message.clone().into());
        if let Some(details) = &self.details {
            error.insert(details.name.into(), details.value.clone());
        }
        if let Some((query, span)) = &self.position {
            let col = query[..span.start].chars().count() + 1;
            let len = query[span.start..span.end.max(span.start)]
                .chars()
                .count()
                .max(1);
            error.insert(self.field.into(), query.clone().into());
            error.insert(
                "position".into(),
                serde_json::json!({ "col": col, "len": len }),
            );
        }
        if let Some(hint) = &self.hint {
            error.insert("hint".into(), hint.clone().into());
        }
        serde_json::json!({ "error": error }).to_string()
    }

    /// The text format of OUTPUT.md#errors.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut out = format!("error[{}]: {}\n", self.code, self.message);
        if let Some((query, span)) = &self.position {
            let line_start = query[..span.start].rfind('\n').map_or(0, |i| i + 1);
            let line_end = query[span.start..]
                .find('\n')
                .map_or(query.len(), |i| span.start + i);
            let line = &query[line_start..line_end];
            let col = query[line_start..span.start].chars().count();
            let end = span.end.min(line_end).max(span.start);
            let len = query[span.start..end].chars().count().max(1);
            out.push_str("  ");
            out.push_str(line);
            out.push('\n');
            out.push_str("  ");
            out.push_str(&" ".repeat(col));
            out.push_str(&"^".repeat(len));
            out.push('\n');
        }
        for line in self.details.iter().flat_map(|d| &d.lines) {
            out.push_str("  ");
            out.push_str(line);
            out.push('\n');
        }
        if let Some(hint) = &self.hint {
            out.push_str("  hint: ");
            out.push_str(hint);
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_the_span_in_its_line() {
        let q = "name ~ '*.rs'\nand text contains 'x'";
        let start = q.find("contains").unwrap();
        let report = QueryError::new(
            ErrorCode::UnknownOperator,
            "unknown operator 'contains'",
            start..start + 8,
        )
        .hint("h")
        .into_report(q);
        assert_eq!(
            report.to_text(),
            "error[unknown_operator]: unknown operator 'contains'\n  and text contains 'x'\n           ^^^^^^^^\n  hint: h\n"
        );
    }

    #[test]
    fn marks_the_end_of_the_query_with_one_caret() {
        let q = "name ~";
        let report = QueryError::new(
            ErrorCode::UnexpectedEnd,
            "unexpected end of the query",
            6..6,
        )
        .into_report(q);
        assert_eq!(
            report.to_text(),
            "error[unexpected_end]: unexpected end of the query\n  name ~\n        ^\n"
        );
    }

    #[test]
    fn counts_columns_in_characters() {
        let q = "name = 'é' x";
        let start = q.find('x').unwrap();
        let report = QueryError::new(
            ErrorCode::UnexpectedToken,
            "unexpected 'x'",
            start..start + 1,
        )
        .into_report(q);
        assert!(
            report.to_text().contains("\n             ^\n"),
            "{}",
            report.to_text()
        );
    }

    #[test]
    fn a_position_in_another_field_and_lines_before_the_hint() {
        let report = QueryError::new(ErrorCode::InvalidTemplate, "unknown placeholder 'x'", 6..9)
            .into_field_report("to", "{dir}/{x}");
        assert_eq!(
            report.to_text(),
            "error[invalid_template]: unknown placeholder 'x'\n  {dir}/{x}\n        ^^^\n"
        );
        let json: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
        assert_eq!(json["error"]["to"], "{dir}/{x}");
        assert_eq!(
            json["error"]["position"],
            serde_json::json!({"col": 7, "len": 3})
        );
        assert!(json["error"].get("query").is_none());
        let report = Report::new(ErrorCode::RenameConflict, "targets conflict")
            .details(
                vec!["a -> b: exists".into()],
                "conflicts",
                serde_json::json!([{"path": "a"}]),
            )
            .hint("h");
        assert_eq!(
            report.to_text(),
            "error[rename_conflict]: targets conflict\n  a -> b: exists\n  hint: h\n"
        );
        let json: serde_json::Value = serde_json::from_str(&report.to_json()).unwrap();
        assert_eq!(json["error"]["conflicts"][0]["path"], "a");
    }

    #[test]
    fn request_errors_have_no_position() {
        let report = Report::new(ErrorCode::InvalidOption, "unknown field 'x'")
            .hint("expected one of: query");
        assert_eq!(
            report.to_text(),
            "error[invalid_option]: unknown field 'x'\n  hint: expected one of: query\n"
        );
    }
}
