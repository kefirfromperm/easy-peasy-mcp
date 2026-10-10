pub enum Expr {
    Num(f64),
}

pub fn number(text: &str) -> Expr {
    Expr::Num(text.parse().unwrap())
}
