pub const KW_ROOT: &str = ".root";
pub const KW_DEFINE: &str = ".define";
pub const KW_BLOCK: &str = ".block";
pub const KW_ARRAY: &str = ".array";
pub const KW_EXPORT: &str = ".export";

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Symbol(String),
    NumberList(Vec<f64>),
    List(String, Vec<Expr>),
    Comment(String),
}

impl From<&str> for Expr {
    fn from(key: &str) -> Self {
        Expr::Symbol(String::from(key))
    }
}

impl From<f64> for Expr {
    fn from(n: f64) -> Self {
        Expr::NumberList(vec![n])
    }
}

impl Expr {
    pub fn list(keyword: &str, cons: Vec<Expr>) -> Self {
        Self::List(String::from(keyword), cons)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
    name: String,
}
