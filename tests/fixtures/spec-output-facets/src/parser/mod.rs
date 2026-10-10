mod ast;
mod lexer;

pub fn parse(text: &str) -> ast::Expr {
    lexer::tokens(text).expect("tokens").into()
}
