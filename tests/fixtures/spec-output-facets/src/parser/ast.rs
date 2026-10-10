pub enum Expr {
    Num(f64),
}

impl From<Vec<String>> for Expr {
    fn from(tokens: Vec<String>) -> Self {
        Expr::Num(tokens[0].parse().unwrap())
    }
}
