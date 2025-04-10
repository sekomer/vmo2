use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "oxyde.pest"]
pub struct OxydeParser;

#[derive(Debug)]
pub struct AstProgram {
    pub statements: Vec<AstStatement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstStatement {
    Assignment(String, AstExpression),
    FunctionDefinition(String, Vec<AstStatement>),
    While(AstExpression, Vec<AstStatement>),
    Expression(AstExpression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstLiteral {
    UInt(u32),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AstExpression {
    BinaryOperation(String, Box<AstExpression>, Box<AstExpression>),
    Literal(AstLiteral),
    FunctionCall(String, Vec<AstExpression>),
    Variable(String),
}
