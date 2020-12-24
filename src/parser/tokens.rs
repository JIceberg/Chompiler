extern crate itertools;

use self::itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

pub struct TokenParser<'a> {
    pub tokens: Vec<Token>,
    iter: Peekable<Chars<'a>>,
}

impl<'a> TokenParser<'a> {
    pub fn new(source: &str) -> TokenParser {
        TokenParser { tokens: vec![], iter: source.chars().peekable() }
    }

    pub fn push(&mut self, token: Token) {
        self.iter.next();
        self.tokens.push(token);
    }

    pub fn push_back(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn next(&mut self) -> Option<char> {
        self.iter.next()
    }

    pub fn drop(&mut self) {
        self.iter.next();
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn get_string<F>(&mut self, func: F) -> String
        where F : Fn(&char) -> bool {
        self.iter.peeking_take_while(|c| func(c)).collect()
    }
}

// here we create an enum for our keywords in C
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Keyword {
    Int,
    Char,
    Return,
    If,
    Else,
    While,
}

// here we create values for our data types
// ints are 32 bits and chars are a byte
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Int(u32),
    Char(u8),
}

// list of tokens
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    // punctuation
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    SemiColon,
    Comma,
    Colon,
    Question,

    // numbers and words we type into the environment/editor
    Keyword(Keyword),
    Identifier(String),
    Literal(Value),

    // bitwise
    BitComp,
    BitwiseLeft,
    BitwiseRight,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,

    // logical
    LogicalNeg,
    And,
    Or,

    // arithmetic
    Negation,
    Addition,
    Multiplication,
    Division,
    Modulus,

    // comparison
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // assignment
    Assign,
    AssignAdd,
    AssignSub,
    AssignDiv,
    AssignMul,
    AssignMod,
    AssignBitLeft,
    AssignBitRight,
    AssignAnd,
    AssignOr,
    AssignXor,

    // increment/decrement unary ops
    Increment,
    Decrement,
}