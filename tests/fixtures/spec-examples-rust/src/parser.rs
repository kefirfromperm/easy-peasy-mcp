//! A recursive descent parser: tokens to the syntax tree of a script.
//!
//! Every statement starts with a keyword, so the parser looks at one token to decide what comes next.
//! Expressions are parsed by precedence climbing, see `Parser::binary`.
//!
//! The syntax tree keeps the line and the column of every statement, so an error of the
//! evaluator can point to the place in the script. Statements which don't start with a keyword
//! are expressions followed by a semicolon. The parser never backtracks: the next token decides
//! which rule comes next, and an unexpected token is an error right away. Blocks and
//! expressions count their nesting, so a script nested too deeply is an error instead of a
//! stack overflow of the interpreter. Operators are left associative; a comparison of a
//! comparison is allowed, but rarely what the author meant. Numbers are always floating point,
//! strings have no escapes yet.

use crate::error::{Error, Result};
use crate::lexer::{tokenize, Token, TokenKind};

/// A whole script: its statements in the order of the source.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

/// A binary operator, from the lowest precedence to the highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

/// A unary operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

/// An expression with the line it starts on.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Str(String),
    Bool(bool),
    Var(String),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Function(Vec<String>, Vec<Stmt>),
}

/// A statement.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    Fn(String, Vec<String>, Vec<Stmt>),
    /// A variable binding: `let name = value;`
    Let(LetStmt),
    /// An assignment to an existing variable: `set name = value;`
    Assign(AssignStmt),
    /// Prints its arguments, separated by spaces: `print a, b;`
    Print(PrintStmt),
    /// Returns from the current function: `return value;`
    Return(ReturnStmt),
    /// Leaves the innermost loop: `break;`
    Break(BreakStmt),
    /// Starts the next iteration of the innermost loop: `continue;`
    Continue(ContinueStmt),
    /// Fails the script when the condition is false: `assert cond;`
    Assert(AssertStmt),
    /// Evaluates another script once: `import "path";`
    Import(ImportStmt),
    /// Makes a name visible to the importing script: `export name;`
    Export(ExportStmt),
    /// Runs a statement when the function returns: `defer stmt;`
    Defer(DeferStmt),
    /// Produces the next value of a generator: `yield value;`
    Yield(YieldStmt),
    /// Raises an error with a message: `throw message;`
    Throw(ThrowStmt),
    /// Pauses the script for some milliseconds: `sleep ms;`
    Sleep(SleepStmt),
    /// Prints the value of an expression with its source: `trace expr;`
    Trace(TraceStmt),
    /// Declares a variable of the outermost scope: `global name;`
    Global(GlobalStmt),
    /// A binding which can't be assigned again: `const name = value;`
    Const(ConstStmt),
    /// Removes a variable from its scope: `delete name;`
    Delete(DeleteStmt),
    /// Exchanges the values of two expressions: `swap a, b;`
    Swap(SwapStmt),
    /// Adds one to a variable: `incr name;`
    Incr(IncrStmt),
    /// Subtracts one from a variable: `decr name;`
    Decr(DecrStmt),
    /// Stops the interpreter with a status code: `exit code;`
    Exit(ExitStmt),
    /// Writes a message to the log of the debug server: `log message;`
    Log(LogStmt),
    /// Reports every change of a variable to the debug server: `watch name;`
    Watch(WatchStmt),
    /// Stops reporting the changes of a variable: `unwatch name;`
    Unwatch(UnwatchStmt),
    /// Stops in the debugger when the condition is true: `breakpoint cond;`
    Breakpoint(BreakpointStmt),
    /// Prints how long a statement took: `time stmt;`
    Time(TimeStmt),
}

/// The `let` statement: a variable binding: `let name = value;`
#[derive(Debug, Clone, PartialEq)]
pub struct LetStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `set` statement: an assignment to an existing variable: `set name = value;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssignStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `print` statement: prints its arguments, separated by spaces: `print a, b;`
#[derive(Debug, Clone, PartialEq)]
pub struct PrintStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `return` statement: returns from the current function: `return value;`
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `break` statement: leaves the innermost loop: `break;`
#[derive(Debug, Clone, PartialEq)]
pub struct BreakStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `continue` statement: starts the next iteration of the innermost loop: `continue;`
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `assert` statement: fails the script when the condition is false: `assert cond;`
#[derive(Debug, Clone, PartialEq)]
pub struct AssertStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `import` statement: evaluates another script once: `import "path";`
#[derive(Debug, Clone, PartialEq)]
pub struct ImportStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `export` statement: makes a name visible to the importing script: `export name;`
#[derive(Debug, Clone, PartialEq)]
pub struct ExportStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `defer` statement: runs a statement when the function returns: `defer stmt;`
#[derive(Debug, Clone, PartialEq)]
pub struct DeferStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `yield` statement: produces the next value of a generator: `yield value;`
#[derive(Debug, Clone, PartialEq)]
pub struct YieldStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `throw` statement: raises an error with a message: `throw message;`
#[derive(Debug, Clone, PartialEq)]
pub struct ThrowStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `sleep` statement: pauses the script for some milliseconds: `sleep ms;`
#[derive(Debug, Clone, PartialEq)]
pub struct SleepStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `trace` statement: prints the value of an expression with its source: `trace expr;`
#[derive(Debug, Clone, PartialEq)]
pub struct TraceStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `global` statement: declares a variable of the outermost scope: `global name;`
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `const` statement: a binding which can't be assigned again: `const name = value;`
#[derive(Debug, Clone, PartialEq)]
pub struct ConstStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `delete` statement: removes a variable from its scope: `delete name;`
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `swap` statement: exchanges the values of two expressions: `swap a, b;`
#[derive(Debug, Clone, PartialEq)]
pub struct SwapStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `incr` statement: adds one to a variable: `incr name;`
#[derive(Debug, Clone, PartialEq)]
pub struct IncrStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `decr` statement: subtracts one from a variable: `decr name;`
#[derive(Debug, Clone, PartialEq)]
pub struct DecrStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `exit` statement: stops the interpreter with a status code: `exit code;`
#[derive(Debug, Clone, PartialEq)]
pub struct ExitStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `log` statement: writes a message to the log of the debug server: `log message;`
#[derive(Debug, Clone, PartialEq)]
pub struct LogStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `watch` statement: reports every change of a variable to the debug server: `watch name;`
#[derive(Debug, Clone, PartialEq)]
pub struct WatchStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `unwatch` statement: stops reporting the changes of a variable: `unwatch name;`
#[derive(Debug, Clone, PartialEq)]
pub struct UnwatchStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `breakpoint` statement: stops in the debugger when the condition is true: `breakpoint cond;`
#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// The `time` statement: prints how long a statement took: `time stmt;`
#[derive(Debug, Clone, PartialEq)]
pub struct TimeStmt {
    /// The line of the keyword.
    pub line: usize,
    /// The column of the keyword.
    pub col: usize,
    /// The name after the keyword, if the statement has one.
    pub name: Option<String>,
    /// The expressions of the statement, in the order of the source.
    pub args: Vec<Expr>,
}

/// Parses a whole script.
pub fn parse(source: &str) -> Result<Program> {
    let tokens = tokenize(source)?;
    let mut parser = Parser { tokens, pos: 0, depth: 0 };
    let mut statements = Vec::new();
    while !parser.at_end() {
        statements.push(parser.statement()?);
    }
    Ok(Program { statements })
}

/// The state of the parser: the tokens and the position of the next one.
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    /// The nesting of blocks and expressions, to stop a runaway recursion.
    depth: usize,
}

/// The max nesting of blocks and expressions.
const MAX_DEPTH: usize = 100000;

impl Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if !self.at_end() {
            self.pos += 1;
        }
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        &self.peek().kind == kind
    }

    fn consume(&mut self, kind: TokenKind, what: &str) -> Result<Token> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(self.error(&format!("expected {what}")))
        }
    }

    fn error(&self, message: &str) -> Error {
        let token = self.peek();
        Error::Syntax { line: token.line, col: token.col, message: message.to_string() }
    }

    fn ident(&mut self) -> Result<String> {
        match self.advance().kind {
            TokenKind::Ident(name) => Ok(name),
            _ => Err(self.error("expected a name")),
        }
    }

    fn enter(&mut self) -> Result<()> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(self.error("the script is nested too deeply"));
        }
        Ok(())
    }

    fn leave(&mut self) {
        self.depth -= 1;
    }

    /// One statement, chosen by its first token.
    fn statement(&mut self) -> Result<Stmt> {
    // TODO: recover after a syntax error and report the next ones too
        match self.peek().kind.clone() {
            TokenKind::Keyword("if") => self.if_statement(),
            TokenKind::Keyword("while") => self.while_statement(),
            TokenKind::Keyword("fn") => self.fn_statement(),
            TokenKind::Ident(word) if word == "let" => self.let_statement().map(Stmt::Let),
            TokenKind::Ident(word) if word == "set" => self.set_statement().map(Stmt::Assign),
            TokenKind::Ident(word) if word == "print" => self.print_statement().map(Stmt::Print),
            TokenKind::Ident(word) if word == "return" => self.return_statement().map(Stmt::Return),
            TokenKind::Ident(word) if word == "break" => self.break_statement().map(Stmt::Break),
            TokenKind::Ident(word) if word == "continue" => self.continue_statement().map(Stmt::Continue),
            TokenKind::Ident(word) if word == "assert" => self.assert_statement().map(Stmt::Assert),
            TokenKind::Ident(word) if word == "import" => self.import_statement().map(Stmt::Import),
            TokenKind::Ident(word) if word == "export" => self.export_statement().map(Stmt::Export),
            TokenKind::Ident(word) if word == "defer" => self.defer_statement().map(Stmt::Defer),
            TokenKind::Ident(word) if word == "yield" => self.yield_statement().map(Stmt::Yield),
            TokenKind::Ident(word) if word == "throw" => self.throw_statement().map(Stmt::Throw),
            TokenKind::Ident(word) if word == "sleep" => self.sleep_statement().map(Stmt::Sleep),
            TokenKind::Ident(word) if word == "trace" => self.trace_statement().map(Stmt::Trace),
            TokenKind::Ident(word) if word == "global" => self.global_statement().map(Stmt::Global),
            TokenKind::Ident(word) if word == "const" => self.const_statement().map(Stmt::Const),
            TokenKind::Ident(word) if word == "delete" => self.delete_statement().map(Stmt::Delete),
            TokenKind::Ident(word) if word == "swap" => self.swap_statement().map(Stmt::Swap),
            TokenKind::Ident(word) if word == "incr" => self.incr_statement().map(Stmt::Incr),
            TokenKind::Ident(word) if word == "decr" => self.decr_statement().map(Stmt::Decr),
            TokenKind::Ident(word) if word == "exit" => self.exit_statement().map(Stmt::Exit),
            TokenKind::Ident(word) if word == "log" => self.log_statement().map(Stmt::Log),
            TokenKind::Ident(word) if word == "watch" => self.watch_statement().map(Stmt::Watch),
            TokenKind::Ident(word) if word == "unwatch" => self.unwatch_statement().map(Stmt::Unwatch),
            TokenKind::Ident(word) if word == "breakpoint" => self.breakpoint_statement().map(Stmt::Breakpoint),
            TokenKind::Ident(word) if word == "time" => self.time_statement().map(Stmt::Time),
            _ => {
                let expr = self.expression()?;
                self.consume(TokenKind::Semicolon, "';' after an expression")?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    /// `{ statements }`
    fn block(&mut self) -> Result<Vec<Stmt>> {
        self.enter()?;
        self.consume(TokenKind::LBrace, "'{'")?;
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.at_end() {
            statements.push(self.statement()?);
        }
        self.consume(TokenKind::RBrace, "'}'")?;
        self.leave();
        Ok(statements)
    }

    /// `if cond { ... } else { ... }`, `else if` is an `if` in the `else` block.
    fn if_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let cond = self.expression()?;
        let then = self.block()?;
        let otherwise = if self.check(&TokenKind::Keyword("else")) {
            self.advance();
            if self.check(&TokenKind::Keyword("if")) {
                vec![self.if_statement()?]
            } else {
                self.block()?
            }
        } else {
            Vec::new()
        };
        Ok(Stmt::If(cond, then, otherwise))
    }

    /// `while cond { ... }`
    fn while_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let cond = self.expression()?;
        let body = self.block()?;
        Ok(Stmt::While(cond, body))
    }

    /// `fn name(a, b) { ... }`
    fn fn_statement(&mut self) -> Result<Stmt> {
        self.advance();
        let name = self.ident()?;
        let params = self.params()?;
        let body = self.block()?;
        Ok(Stmt::Fn(name, params, body))
    }

    /// `(a, b, c)`
    fn params(&mut self) -> Result<Vec<String>> {
        self.consume(TokenKind::LParen, "'('")?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) {
            params.push(self.ident()?);
            if !self.check(&TokenKind::RParen) {
                self.consume(TokenKind::Comma, "',' between parameters")?;
            }
        }
        self.consume(TokenKind::RParen, "')'")?;
        Ok(params)
    }

    /// The `let` statement: a variable binding: `let name = value;`
    fn let_statement(&mut self) -> Result<LetStmt> {
        let keyword = self.advance();
        let mut stmt = LetStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        self.consume(TokenKind::Op("="), "'=' after the name")?;
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the let statement")?;
        Ok(stmt)
    }

    /// The `set` statement: an assignment to an existing variable: `set name = value;`
    fn set_statement(&mut self) -> Result<AssignStmt> {
        let keyword = self.advance();
        let mut stmt = AssignStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        self.consume(TokenKind::Op("="), "'=' after the name")?;
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the set statement")?;
        Ok(stmt)
    }

    /// The `print` statement: prints its arguments, separated by spaces: `print a, b;`
    fn print_statement(&mut self) -> Result<PrintStmt> {
        let keyword = self.advance();
        let mut stmt = PrintStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the print statement")?;
        Ok(stmt)
    }

    /// The `return` statement: returns from the current function: `return value;`
    fn return_statement(&mut self) -> Result<ReturnStmt> {
        let keyword = self.advance();
        let mut stmt = ReturnStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the return statement")?;
        Ok(stmt)
    }

    /// The `break` statement: leaves the innermost loop: `break;`
    fn break_statement(&mut self) -> Result<BreakStmt> {
        let keyword = self.advance();
        let mut stmt = BreakStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        self.consume(TokenKind::Semicolon, "';' after the break statement")?;
        Ok(stmt)
    }

    /// The `continue` statement: starts the next iteration of the innermost loop: `continue;`
    fn continue_statement(&mut self) -> Result<ContinueStmt> {
        let keyword = self.advance();
        let mut stmt = ContinueStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        self.consume(TokenKind::Semicolon, "';' after the continue statement")?;
        Ok(stmt)
    }

    /// The `assert` statement: fails the script when the condition is false: `assert cond;`
    fn assert_statement(&mut self) -> Result<AssertStmt> {
        let keyword = self.advance();
        let mut stmt = AssertStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the assert statement")?;
        Ok(stmt)
    }

    /// The `import` statement: evaluates another script once: `import "path";`
    fn import_statement(&mut self) -> Result<ImportStmt> {
        let keyword = self.advance();
        let mut stmt = ImportStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
    // TODO: keep the comments for the formatter
        self.consume(TokenKind::Semicolon, "';' after the import statement")?;
        Ok(stmt)
    }

    /// The `export` statement: makes a name visible to the importing script: `export name;`
    fn export_statement(&mut self) -> Result<ExportStmt> {
        let keyword = self.advance();
        let mut stmt = ExportStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        self.consume(TokenKind::Semicolon, "';' after the export statement")?;
        Ok(stmt)
    }

    /// The `defer` statement: runs a statement when the function returns: `defer stmt;`
    fn defer_statement(&mut self) -> Result<DeferStmt> {
        let keyword = self.advance();
        let mut stmt = DeferStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the defer statement")?;
        Ok(stmt)
    }

    /// The `yield` statement: produces the next value of a generator: `yield value;`
    fn yield_statement(&mut self) -> Result<YieldStmt> {
        let keyword = self.advance();
        let mut stmt = YieldStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the yield statement")?;
        Ok(stmt)
    }

    /// The `throw` statement: raises an error with a message: `throw message;`
    fn throw_statement(&mut self) -> Result<ThrowStmt> {
        let keyword = self.advance();
        let mut stmt = ThrowStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the throw statement")?;
        Ok(stmt)
    }

    /// The `sleep` statement: pauses the script for some milliseconds: `sleep ms;`
    fn sleep_statement(&mut self) -> Result<SleepStmt> {
        let keyword = self.advance();
        let mut stmt = SleepStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the sleep statement")?;
        Ok(stmt)
    }

    /// The `trace` statement: prints the value of an expression with its source: `trace expr;`
    fn trace_statement(&mut self) -> Result<TraceStmt> {
        let keyword = self.advance();
        let mut stmt = TraceStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the trace statement")?;
        Ok(stmt)
    }

    /// The `global` statement: declares a variable of the outermost scope: `global name;`
    fn global_statement(&mut self) -> Result<GlobalStmt> {
        let keyword = self.advance();
        let mut stmt = GlobalStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        self.consume(TokenKind::Semicolon, "';' after the global statement")?;
        Ok(stmt)
    }

    /// The `const` statement: a binding which can't be assigned again: `const name = value;`
    fn const_statement(&mut self) -> Result<ConstStmt> {
        let keyword = self.advance();
        let mut stmt = ConstStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        self.consume(TokenKind::Op("="), "'=' after the name")?;
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the const statement")?;
        Ok(stmt)
    }

    /// The `delete` statement: removes a variable from its scope: `delete name;`
    fn delete_statement(&mut self) -> Result<DeleteStmt> {
        let keyword = self.advance();
        let mut stmt = DeleteStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the delete statement")?;
        Ok(stmt)
    }

    /// The `swap` statement: exchanges the values of two expressions: `swap a, b;`
    fn swap_statement(&mut self) -> Result<SwapStmt> {
        let keyword = self.advance();
        let mut stmt = SwapStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the swap statement")?;
        Ok(stmt)
    }

    /// The `incr` statement: adds one to a variable: `incr name;`
    fn incr_statement(&mut self) -> Result<IncrStmt> {
        let keyword = self.advance();
        let mut stmt = IncrStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the incr statement")?;
        Ok(stmt)
    }

    /// The `decr` statement: subtracts one from a variable: `decr name;`
    fn decr_statement(&mut self) -> Result<DecrStmt> {
        let keyword = self.advance();
        let mut stmt = DecrStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the decr statement")?;
        Ok(stmt)
    }

    /// The `exit` statement: stops the interpreter with a status code: `exit code;`
    fn exit_statement(&mut self) -> Result<ExitStmt> {
        let keyword = self.advance();
        let mut stmt = ExitStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the exit statement")?;
        Ok(stmt)
    }

    /// The `log` statement: writes a message to the log of the debug server: `log message;`
    fn log_statement(&mut self) -> Result<LogStmt> {
        let keyword = self.advance();
        let mut stmt = LogStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the log statement")?;
        Ok(stmt)
    }

    /// The `watch` statement: reports every change of a variable to the debug server: `watch name;`
    fn watch_statement(&mut self) -> Result<WatchStmt> {
        let keyword = self.advance();
        let mut stmt = WatchStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the watch statement")?;
        Ok(stmt)
    }

    /// The `unwatch` statement: stops reporting the changes of a variable: `unwatch name;`
    fn unwatch_statement(&mut self) -> Result<UnwatchStmt> {
        let keyword = self.advance();
        let mut stmt = UnwatchStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        stmt.name = Some(self.ident()?);
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the unwatch statement")?;
        Ok(stmt)
    }

    /// The `breakpoint` statement: stops in the debugger when the condition is true: `breakpoint cond;`
    fn breakpoint_statement(&mut self) -> Result<BreakpointStmt> {
        let keyword = self.advance();
        let mut stmt = BreakpointStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the breakpoint statement")?;
        Ok(stmt)
    }

    /// The `time` statement: prints how long a statement took: `time stmt;`
    fn time_statement(&mut self) -> Result<TimeStmt> {
        let keyword = self.advance();
        let mut stmt = TimeStmt { line: keyword.line, col: keyword.col, name: None, args: Vec::new() };
        if !self.check(&TokenKind::Semicolon) {
            stmt.args.push(self.expression()?);
            while self.check(&TokenKind::Comma) {
                self.advance();
                stmt.args.push(self.expression()?);
            }
        }
        self.consume(TokenKind::Semicolon, "';' after the time statement")?;
        Ok(stmt)
    }

    /// An expression of any precedence.
    fn expression(&mut self) -> Result<Expr> {
        self.enter()?;
        let expr = self.binary(0)?;
        self.leave();
        Ok(expr)
    }

    /// Precedence climbing: the operators of at least `min` precedence.
    fn binary(&mut self, min: u8) -> Result<Expr> {
        let mut left = self.unary()?;
        while let Some((op, prec)) = self.binary_op() {
            if prec < min {
                break;
            }
            self.advance();
            let right = self.binary(prec + 1)?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    /// The binary operator of the next token and its precedence.
    fn binary_op(&self) -> Option<(BinOp, u8)> {
        let TokenKind::Op(op) = self.peek().kind else {
            return None;
        };
        Some(match op {
            "||" => (BinOp::Or, 1),
            "&&" => (BinOp::And, 2),
            "==" => (BinOp::Eq, 3),
            "!=" => (BinOp::Ne, 3),
            "<" => (BinOp::Lt, 4),
            "<=" => (BinOp::Le, 4),
            ">" => (BinOp::Gt, 4),
            ">=" => (BinOp::Ge, 4),
            "+" => (BinOp::Add, 5),
            "-" => (BinOp::Sub, 5),
            "*" => (BinOp::Mul, 6),
            "/" => (BinOp::Div, 6),
            "%" => (BinOp::Rem, 6),
            _ => return None,
        })
    }

    /// `-x`, `!x` or a call.
    fn unary(&mut self) -> Result<Expr> {
        match self.peek().kind {
            TokenKind::Op("-") => {
                self.advance();
                Ok(Expr::Unary(UnOp::Neg, Box::new(self.unary()?)))
            }
            TokenKind::Op("!") => {
                self.advance();
                Ok(Expr::Unary(UnOp::Not, Box::new(self.unary()?)))
            }
            _ => self.call(),
        }
    }

    /// A primary expression followed by any number of argument lists.
    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        while self.check(&TokenKind::LParen) {
            self.advance();
            let mut args = Vec::new();
            while !self.check(&TokenKind::RParen) {
                args.push(self.expression()?);
                if !self.check(&TokenKind::RParen) {
                    self.consume(TokenKind::Comma, "',' between arguments")?;
                }
            }
            self.consume(TokenKind::RParen, "')' after the arguments")?;
            expr = Expr::Call(Box::new(expr), args);
        }
        Ok(expr)
    }

    /// A literal, a name, a function or an expression in parentheses.
    fn primary(&mut self) -> Result<Expr> {
    // TODO: a better message for a missing semicolon
        let token = self.advance();
        match token.kind {
            TokenKind::Number(n) => Ok(Expr::Number(n)),
            TokenKind::Str(s) => Ok(Expr::Str(s)),
            TokenKind::Keyword("true") => Ok(Expr::Bool(true)),
            TokenKind::Keyword("false") => Ok(Expr::Bool(false)),
            TokenKind::Ident(name) => Ok(Expr::Var(name)),
            TokenKind::Keyword("fn") => {
                let params = self.params()?;
                let body = self.block()?;
                Ok(Expr::Function(params, body))
            }
            TokenKind::LParen => {
                let expr = self.expression()?;
                self.consume(TokenKind::RParen, "')'")?;
                Ok(expr)
            }
            _ => Err(Error::Syntax { line: token.line, col: token.col, message: "expected an expression".to_string() }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(source: &str) -> Stmt {
        let mut program = parse(source).ok().into_iter().flat_map(|p| p.statements);
        program.next().unwrap_or(Stmt::Expr(Expr::Bool(false)))
    }

    #[test]
    fn an_expression_statement() {
        assert_eq!(one("1 + 2 * 3;"), Stmt::Expr(Expr::Binary(
            BinOp::Add,
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Binary(BinOp::Mul, Box::new(Expr::Number(2.0)), Box::new(Expr::Number(3.0)))),
        )));
    }

    #[test]
    fn the_let_statement() {
        let source = "let name = value;";
        assert!(matches!(one(source), Stmt::Let(_)), "{source}");
        assert!(parse("let").is_err(), "let without a semicolon");
    }

    #[test]
    fn the_set_statement() {
        let source = "set name = value;";
        assert!(matches!(one(source), Stmt::Assign(_)), "{source}");
        assert!(parse("set").is_err(), "set without a semicolon");
    }

    #[test]
    fn the_print_statement() {
        let source = "print a, b;";
        assert!(matches!(one(source), Stmt::Print(_)), "{source}");
        assert!(parse("print").is_err(), "print without a semicolon");
    }

    #[test]
    fn the_return_statement() {
        let source = "return value;";
        assert!(matches!(one(source), Stmt::Return(_)), "{source}");
        assert!(parse("return").is_err(), "return without a semicolon");
    }

    #[test]
    fn the_break_statement() {
        let source = "break;";
        assert!(matches!(one(source), Stmt::Break(_)), "{source}");
        assert!(parse("break").is_err(), "break without a semicolon");
    }

    #[test]
    fn the_continue_statement() {
        let source = "continue;";
        assert!(matches!(one(source), Stmt::Continue(_)), "{source}");
        assert!(parse("continue").is_err(), "continue without a semicolon");
    }

    #[test]
    fn the_assert_statement() {
        let source = "assert cond;";
        assert!(matches!(one(source), Stmt::Assert(_)), "{source}");
        assert!(parse("assert").is_err(), "assert without a semicolon");
    }

    #[test]
    fn the_import_statement() {
        let source = "import \"path\";";
        assert!(matches!(one(source), Stmt::Import(_)), "{source}");
        assert!(parse("import").is_err(), "import without a semicolon");
    }

    #[test]
    fn the_export_statement() {
        let source = "export name;";
        assert!(matches!(one(source), Stmt::Export(_)), "{source}");
        assert!(parse("export").is_err(), "export without a semicolon");
    }

    #[test]
    fn the_defer_statement() {
        let source = "defer stmt;";
        assert!(matches!(one(source), Stmt::Defer(_)), "{source}");
        assert!(parse("defer").is_err(), "defer without a semicolon");
    }

    #[test]
    fn the_yield_statement() {
        let source = "yield value;";
        assert!(matches!(one(source), Stmt::Yield(_)), "{source}");
        assert!(parse("yield").is_err(), "yield without a semicolon");
    }

    #[test]
    fn the_throw_statement() {
        let source = "throw message;";
        assert!(matches!(one(source), Stmt::Throw(_)), "{source}");
        assert!(parse("throw").is_err(), "throw without a semicolon");
    }

    #[test]
    fn the_sleep_statement() {
        let source = "sleep ms;";
        assert!(matches!(one(source), Stmt::Sleep(_)), "{source}");
        assert!(parse("sleep").is_err(), "sleep without a semicolon");
    }

    #[test]
    fn the_trace_statement() {
        let source = "trace expr;";
        assert!(matches!(one(source), Stmt::Trace(_)), "{source}");
        assert!(parse("trace").is_err(), "trace without a semicolon");
    }

    #[test]
    fn the_global_statement() {
        let source = "global name;";
        assert!(matches!(one(source), Stmt::Global(_)), "{source}");
        assert!(parse("global").is_err(), "global without a semicolon");
    }

    #[test]
    fn the_const_statement() {
        let source = "const name = value;";
        assert!(matches!(one(source), Stmt::Const(_)), "{source}");
        assert!(parse("const").is_err(), "const without a semicolon");
    }

    #[test]
    fn the_delete_statement() {
        let source = "delete name;";
        assert!(matches!(one(source), Stmt::Delete(_)), "{source}");
        assert!(parse("delete").is_err(), "delete without a semicolon");
    }

    #[test]
    fn the_swap_statement() {
        let source = "swap a, b;";
        assert!(matches!(one(source), Stmt::Swap(_)), "{source}");
        assert!(parse("swap").is_err(), "swap without a semicolon");
    }

    #[test]
    fn the_incr_statement() {
        let source = "incr name;";
        assert!(matches!(one(source), Stmt::Incr(_)), "{source}");
        assert!(parse("incr").is_err(), "incr without a semicolon");
    }

    #[test]
    fn the_decr_statement() {
        let source = "decr name;";
        assert!(matches!(one(source), Stmt::Decr(_)), "{source}");
        assert!(parse("decr").is_err(), "decr without a semicolon");
    }

    #[test]
    fn the_exit_statement() {
        let source = "exit code;";
        assert!(matches!(one(source), Stmt::Exit(_)), "{source}");
        assert!(parse("exit").is_err(), "exit without a semicolon");
    }

    #[test]
    fn the_log_statement() {
        let source = "log message;";
        assert!(matches!(one(source), Stmt::Log(_)), "{source}");
        assert!(parse("log").is_err(), "log without a semicolon");
    }

    #[test]
    fn the_watch_statement() {
        let source = "watch name;";
        assert!(matches!(one(source), Stmt::Watch(_)), "{source}");
        assert!(parse("watch").is_err(), "watch without a semicolon");
    }

    #[test]
    fn the_unwatch_statement() {
        let source = "unwatch name;";
        assert!(matches!(one(source), Stmt::Unwatch(_)), "{source}");
        assert!(parse("unwatch").is_err(), "unwatch without a semicolon");
    }

    #[test]
    fn the_breakpoint_statement() {
        let source = "breakpoint cond;";
        assert!(matches!(one(source), Stmt::Breakpoint(_)), "{source}");
        assert!(parse("breakpoint").is_err(), "breakpoint without a semicolon");
    }

    #[test]
    fn the_time_statement() {
        let source = "time stmt;";
        assert!(matches!(one(source), Stmt::Time(_)), "{source}");
        assert!(parse("time").is_err(), "time without a semicolon");
    }

    #[test]
    fn errors_have_a_position() {
        match parse("let x = ;") {
            Err(Error::Syntax { line, col, .. }) => assert_eq!((line, col), (1, 9)),
            other => panic!("a syntax error is expected, got {other:?}"),
        }
    }
}
