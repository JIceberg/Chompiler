use super::tokens::Token;
use super::tokens::Keyword;
use super::tokens::Value;
use super::oper::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens.into_iter().collect() }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn next_token(&mut self) -> Token {
        self.next().expect("Failed to parse token")
    }

    fn tokens_left(&mut self) -> bool {
        !self.tokens.is_empty()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.front()
    }

    fn peek_token(&mut self, token: Token) -> Result<Token, String> {
        match self.peek() {
            Some(t) if t == &token => Ok(token),
            other => Err(format!("{:?} not found, wsa instead {:?}", token, other))
        }
    }

    fn match_token(&mut self, token: Token) -> Result<Token, String> {
        match self.next_token() {
            ref t if t == &token => Ok(token),
            other => Err(format!("{:?} not found, was instead {:?}", token, other))
        }
    }

    fn match_keyword(&mut self, keyword: &Keyword) -> Result<(), String> {
        match self.next_token() {
            Token::Keyword(ref k) if k == keyword => Ok(()),
            other => Err(format!("Unexpected token {:?}", other))
        }
    }

    fn match_identifier(&mut self) -> Result<String, String> {
        match self.next_token() {
            Token::Identifier(n) => Ok(n),
            other => Err(format!("Unexpected token {:?}", other))
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut funcs = Vec::new();

        while self.tokens_left() {
            funcs.push(self.parse_function().expect("Failed to parse function"))
        }

        Program { functions: funcs, globals: Vec::new() }
    }

    pub fn parse_function(&mut self) -> Result<Function, String> {
        self.match_keyword(&Keyword::Int)?;
        let name = self.match_identifier()?;
        self.match_token(Token::OpenParen)?;

        let arguments: Vec<Variable> = match self.peek() {
            Some(Token::CloseParen) => Vec::new(),
            _ => self.parse_arguments().expect("Failed to parse arguments")
        };

        self.match_token(Token::CloseParen)?;
        self.match_token(Token::OpenBrace)?;

        let mut statements = Vec::new();

        while let Err(_) = self.peek_token(Token::CloseBrace) {
            statements.push(self.parse_statement().expect("Failed to parse statement"));
        }

        self.match_token(Token::CloseBrace)?;

        Ok(Function { name, args: arguments, statements })
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.next() {
            Some(Token::Keyword(Keyword::Return)) => {
                let state = Ok(Statement::Return(self.parse_expression()
                    .expect("Failed to parse expression")));
                self.match_token(Token::SemiColon).expect("Missing ;");
                state
            },
            other => Err(format!("Unexpected token {:?}", other))
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        match self.next() {
            Some(Token::Literal(Value::Int(num))) => Ok(Expression::Int(num)),
            other => Err(format!("Expected a statement, found {:?}", other))
        }
    }

    pub fn parse_arguments(&mut self) -> Result<Vec<Variable>, String> {
        let mut args = Vec::new();
        while let Err(_) = self.peek_token(Token::CloseParen) {
            let typ = match self.next() {
                Some(Token::Keyword(Keyword::Int)) => Primitive::Int,
                other => panic!("Unknown type {:?}", other)
            };
            let name = self.match_identifier()?;
            args.push(Variable { name, size: typ });
            if let Some(Token::Comma) = self.peek() {
                self.next();
            }
        }
        Ok(args)
    }
}