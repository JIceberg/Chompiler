use super::tokens::Token;

#[derive(PartialEq, Eq, Debug)]
pub enum Primitive {
    Int,
    Byte,
}

#[derive(Eq, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub size: Primitive,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub globals: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Variable>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    Int(u32),
    Char(u8),
    FunctionCall(String, Vec<Expression>),
    Variable(String),
    VariableRef(String),
    Assign(String, Box<Expression>),
    AssignPostfix(String, Box<Expression>),
    Ternary(Box<Expression>, Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Declare(Variable, Option<Expression>),
    Return(Expression),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Exp(Expression),
    Compound(Vec<Statement>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum UnaryOp {
    Negation,
    BitComp,
    LogicalNeg,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BinaryOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
    BitwiseLeft,
    BitwiseRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    Comma,
}

impl From<Token> for BinaryOp {
    fn from(token: Token) -> Self {
        match token {
            Token::Multiplication => BinaryOp::Multiplication,
            Token::Division => BinaryOp::Division,
            Token::Addition => BinaryOp::Addition,
            Token::Negation => BinaryOp::Subtraction,
            Token::Modulus => BinaryOp::Modulus,
            Token::LessThan => BinaryOp::LessThan,
            Token::LessThanOrEqual => BinaryOp::LessThanOrEqual,
            Token::GreaterThan => BinaryOp::GreaterThan,
            Token::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
            Token::Equal => BinaryOp::Equal,
            Token::NotEqual => BinaryOp::NotEqual,
            Token::And => BinaryOp::And,
            Token::Or => BinaryOp::Or,
            Token::BitwiseLeft => BinaryOp::BitwiseLeft,
            Token::BitwiseRight => BinaryOp::BitwiseRight,
            Token::BitwiseAnd => BinaryOp::BitwiseAnd,
            Token::BitwiseXor => BinaryOp::BitwiseXor,
            Token::BitwiseOr => BinaryOp::BitwiseOr,
            Token::Comma => BinaryOp::Comma,
            other => panic!("Token {:?} cannot be converted into a binary operation", other)
        }
    }
}

impl From<Token> for UnaryOp {
    fn from(token: Token) -> Self {
        match token {
            Token::Negation => UnaryOp::Negation,
            Token::LogicalNeg => UnaryOp::LogicalNeg,
            Token::BitComp => UnaryOp::BitComp,
            other => panic!("Unsupported token {:?}, can only use: ! ~ -", other)
        }
    }
}