#[derive(Debug)]
pub enum Statement {
    Function(FunctionDef),
    VariableDeclaration(VariableDeclaration),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: String,
    pub value: Box<Expression>,

}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Box<Expression>,
    pub then_branch: Vec<Statement>,
    pub else_branch: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Box<Expression>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct ForStatement {
    pub iterator: String,
    pub range: Box<Expression>,
    pub increment: Option<i64>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub var_type: String,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    BinaryOperation {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}