//! The syntax tree of a query (QUERY.md#grammar).

use std::fmt;
use std::ops::Range;

use super::lexer::Quote;

/// A query target (QUERY.md#targets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Path,
    Name,
    Type,
    Size,
    Modified,
    Created,
    Accessed,
    Empty,
    Lang,
    Kind,
    Git,
    Text,
    Len,
}

/// What kind of value a target takes, which decides its operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetClass {
    /// `path`, `name`, `text`: text operators and a string.
    Text,
    /// `type`, `lang`, `kind`, `git`: `=`, `!=`, `in ( ... )` and words.
    Enum,
    /// `size`, dates, `len`: comparisons and `in a..b`.
    Compare,
    /// `empty`: no operator.
    Alone,
}

impl Target {
    /// All targets in the order of the spec.
    pub const ALL: [Target; 13] = [
        Self::Path,
        Self::Name,
        Self::Type,
        Self::Size,
        Self::Modified,
        Self::Created,
        Self::Accessed,
        Self::Empty,
        Self::Lang,
        Self::Kind,
        Self::Git,
        Self::Text,
        Self::Len,
    ];

    /// The keyword of the target.
    pub fn keyword(self) -> &'static str {
        match self {
            Self::Path => "path",
            Self::Name => "name",
            Self::Type => "type",
            Self::Size => "size",
            Self::Modified => "modified",
            Self::Created => "created",
            Self::Accessed => "accessed",
            Self::Empty => "empty",
            Self::Lang => "lang",
            Self::Kind => "kind",
            Self::Git => "git",
            Self::Text => "text",
            Self::Len => "len",
        }
    }

    /// The target with this keyword, in any case.
    pub fn from_keyword(word: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|t| t.keyword().eq_ignore_ascii_case(word))
    }

    /// The value class of the target.
    pub fn class(self) -> TargetClass {
        match self {
            Self::Path | Self::Name | Self::Text => TargetClass::Text,
            Self::Type | Self::Lang | Self::Kind | Self::Git => TargetClass::Enum,
            Self::Size | Self::Modified | Self::Created | Self::Accessed | Self::Len => {
                TargetClass::Compare
            }
            Self::Empty => TargetClass::Alone,
        }
    }
}

/// A condition operator (QUERY.md#operators).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Has,
    Starts,
    Ends,
    Wildcard,
    Fuzzy,
    Regex,
    In,
}

impl Op {
    /// All operators.
    pub const ALL: [Op; 13] = [
        Self::Eq,
        Self::Ne,
        Self::Lt,
        Self::Le,
        Self::Gt,
        Self::Ge,
        Self::Has,
        Self::Starts,
        Self::Ends,
        Self::Wildcard,
        Self::Fuzzy,
        Self::Regex,
        Self::In,
    ];

    /// The operators of the text targets, in the order of the spec.
    pub const TEXT: [Op; 7] = [
        Self::Eq,
        Self::Has,
        Self::Starts,
        Self::Ends,
        Self::Wildcard,
        Self::Fuzzy,
        Self::Regex,
    ];
    /// The operators of the value targets.
    pub const ENUM: [Op; 3] = [Self::Eq, Self::Ne, Self::In];
    /// The operators of the comparison targets.
    pub const COMPARE: [Op; 7] = [
        Self::Eq,
        Self::Ne,
        Self::Lt,
        Self::Le,
        Self::Gt,
        Self::Ge,
        Self::In,
    ];

    /// The operator as written.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Ne => "!=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Has => "has",
            Self::Starts => "starts",
            Self::Ends => "ends",
            Self::Wildcard => "~",
            Self::Fuzzy => "~~",
            Self::Regex => "=~",
            Self::In => "in",
        }
    }

    /// The operator written as `s`; word operators in any case.
    pub fn from_symbol(s: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|o| o.symbol().eq_ignore_ascii_case(s))
    }

    /// The operators of `text`: no wildcards and no fuzzy matching.
    pub const TEXT_CONTENT: [Op; 5] = [Self::Eq, Self::Has, Self::Starts, Self::Ends, Self::Regex];

    /// The operators allowed for a target.
    pub fn for_target(target: Target) -> &'static [Op] {
        if target == Target::Text {
            &Self::TEXT_CONTENT
        } else {
            Self::for_class(target.class())
        }
    }

    /// The operators allowed for a target class.
    pub fn for_class(class: TargetClass) -> &'static [Op] {
        match class {
            TargetClass::Text => &Self::TEXT,
            TargetClass::Enum => &Self::ENUM,
            TargetClass::Compare => &Self::COMPARE,
            TargetClass::Alone => &[],
        }
    }
}

/// A string literal: its value and how it was written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrLit {
    pub value: String,
    pub quote: Quote,
    pub span: Range<usize>,
}

/// A word, number, size or date with its position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned {
    pub text: String,
    pub span: Range<usize>,
}

/// The value of a condition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// `empty` has no value.
    None,
    Str(StrLit),
    /// A word of a value target: `type = dir`.
    Word(Spanned),
    /// `in ( a, b )`.
    Words(Vec<Spanned>),
    /// A number, size or date.
    Scalar(Spanned),
    /// `in a..b`.
    Range(Spanned, Spanned),
}

/// A modifier after a text value (QUERY.md#modifiers): its name without `/` and the arguments of `/e`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifier {
    pub name: String,
    /// `/e(style, levels)`: the style and the levels as written.
    pub args: Option<(Spanned, Spanned)>,
    pub span: Range<usize>,
}

/// A simple condition: `<target> <operator> <value> {modifier}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub target: Target,
    pub target_span: Range<usize>,
    pub op: Option<(Op, Range<usize>)>,
    pub value: Value,
    pub modifiers: Vec<Modifier>,
}

/// `near(A, B, N, options)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Near {
    pub a: Box<Expr>,
    pub b: Box<Expr>,
    pub distance: Spanned,
    pub options: Vec<Spanned>,
}

/// What a node with a condition id holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Condition(Condition),
    Line(Box<Expr>),
    Near(Near),
    /// `( ... ) @name`: a bracket with a label (QUERY.md#groups).
    Group(Box<Expr>),
}

/// The id of a condition: `#1` or `@label` (QUERY.md#labels-and-condition-ids).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CondId {
    Number(u32),
    Label(String),
}

impl fmt::Display for CondId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "#{n}"),
            Self::Label(l) => write!(f, "@{l}"),
        }
    }
}

/// A sub-group of a `text =~` condition: a capture group by its number or its name (QUERY.md#regex-groups).
#[allow(dead_code)] // the fields which refer to groups come with `output.groups` and `replace` (tasks 129, 130)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubGroup {
    Number(usize),
    Name(String),
}

/// A checked reference to a group of the query (QUERY.md#group-references): a whole group, or a sub-group of a
/// `text =~` condition.
#[allow(dead_code)] // the fields which refer to groups come with `output.groups` and `replace` (tasks 129, 130)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupRef {
    pub id: CondId,
    pub sub: Option<SubGroup>,
}

/// A condition, a `line( ... )`, a `near( ... )` or a bracket group: everything which gets a condition id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub label: Option<Spanned>,
    /// The indexes of the first and the last token of the node, label excluded.
    pub tokens: Range<usize>,
    /// The byte range of the first token: the target, `line`, `near` or `(`.
    pub keyword: Range<usize>,
    /// Set after parsing, in the order of appearance.
    pub id: CondId,
}

/// A query expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Or(Vec<Expr>),
    Xor(Vec<Expr>),
    And(Vec<Expr>),
    Not(Box<Expr>),
    /// `( ... )`
    Group(Box<Expr>),
    Node(Box<Node>),
}

impl Expr {
    /// Calls `f` for every node in the order of appearance, a group node before the nodes inside it.
    pub fn visit_nodes_mut(&mut self, f: &mut impl FnMut(&mut Node)) {
        match self {
            Self::Or(items) | Self::Xor(items) | Self::And(items) => {
                for item in items {
                    item.visit_nodes_mut(f);
                }
            }
            Self::Not(inner) | Self::Group(inner) => inner.visit_nodes_mut(f),
            Self::Node(node) => {
                f(node);
                match &mut node.kind {
                    NodeKind::Condition(_) => {}
                    NodeKind::Line(inner) | NodeKind::Group(inner) => inner.visit_nodes_mut(f),
                    NodeKind::Near(near) => {
                        near.a.visit_nodes_mut(f);
                        near.b.visit_nodes_mut(f);
                    }
                }
            }
        }
    }

    /// Calls `f` for every node in the order of appearance.
    pub fn visit_nodes(&self, f: &mut impl FnMut(&Node)) {
        match self {
            Self::Or(items) | Self::Xor(items) | Self::And(items) => {
                for item in items {
                    item.visit_nodes(f);
                }
            }
            Self::Not(inner) | Self::Group(inner) => inner.visit_nodes(f),
            Self::Node(node) => {
                f(node);
                match &node.kind {
                    NodeKind::Condition(_) => {}
                    NodeKind::Line(inner) | NodeKind::Group(inner) => inner.visit_nodes(f),
                    NodeKind::Near(near) => {
                        near.a.visit_nodes(f);
                        near.b.visit_nodes(f);
                    }
                }
            }
        }
    }
}
