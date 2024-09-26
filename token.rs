#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i64),
    Operator(String),
    Symbol(char),
    StringLiteral(String),
    Comment,
    Whitespace,
    Eof,
}