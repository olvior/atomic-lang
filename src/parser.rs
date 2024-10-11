use crate::{TokenType, Token, exit_message};

mod expression_parser;

pub mod math;
use math::*;

pub use math::MathValue;

#[derive(Debug)]
pub enum NodeStatements {
    Exit(NodeStmtExit),
    Declare(NodeStmtDeclare),
    Set(NodeStmtSet),
}

#[derive(Debug)]
pub struct NodeProgram {
    pub statements: Vec<NodeStatements>
}

// statements
#[derive(Debug)]
pub struct NodeStmtExit {
    pub expression: MathValue,
}

#[derive(Debug)]
pub struct NodeStmtDeclare {
    pub identifier: Token,
    pub expression: MathValue,
}

#[derive(Debug)]
pub struct NodeStmtSet {
    pub identifier: Token,
    pub expression: MathValue,
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl Parser {
    pub fn parse(&mut self) -> NodeProgram {
        let mut program = NodeProgram { statements: vec!() };

        while self.index < self.tokens.len() {
            let token = &self.tokens[self.index];

            let statement = match token.token {
                TokenType::Exit => NodeStatements::Exit(self.parse_exit()),
                TokenType::IntType => NodeStatements::Declare(self.parse_int_assign()),
                TokenType::Identifier => NodeStatements::Set(self.parse_set_var()),
                _ => { dbg!(token); exit_message("Invalid expression"); println!("happy"); return program; }
            };

            program.statements.push(statement);
        }

        return program;
    }

    fn parse_exit(&mut self) -> NodeStmtExit {
        if self.require_token(1, "Could not parse exit").token != TokenType::ParenOpen {
            exit_message("Expected parenthesis");
        }

        // account for exit(
        self.index += 2;

        let expr = self.parse_expr();
        
        if self.require_token(0, "Could not parse exit").token != TokenType::ParenClose {
            exit_message("Expected closing parenthesis");
        }
        if self.require_token(1, "Could not parse exit").token != TokenType::Semicolon {
            exit_message("Expected semicolon");
        }

        // account for );
        self.index += 2;

        return NodeStmtExit { expression: expr };
    }
    
    fn parse_int_assign(&mut self) -> NodeStmtDeclare {
        if self.require_token(1, "Could not parse declaration").token != TokenType::Identifier {
            exit_message("Expected identifier");
        }
        if self.require_token(2, "Could not parse declaration").token != TokenType::AssignEq {
            exit_message("Expected Equal sign");
        }

        let identifier = self.require_token(1, "Could not parse declaration");

        // account for int name =
        self.index += 3;

        let expr = self.parse_expr();


        if self.require_token(0, "Could not parse declaration").token != TokenType::Semicolon {
            exit_message("Expected semicolon");
        }

        // account for ;
        self.index += 1;

        return NodeStmtDeclare { identifier, expression: expr };
    }

    fn parse_set_var(&mut self) -> NodeStmtSet {
        if self.require_token(1, "Could not parse set").token != TokenType::AssignEq {
            exit_message("Expected equal sign");
        }

        let identifier = self.require_token(0, "Could not parse set");

        // account for name =
        self.index += 2;

        let expr = self.parse_expr();
        
        if self.require_token(0, "Could not parse set").token != TokenType::Semicolon {
            exit_message("Expected semicolon");
        }

        // account for ;
        self.index += 1;

        return NodeStmtSet { identifier, expression: expr };
    }


    fn parse_expr(&mut self) -> MathValue {
        let min_index = self.index;

        let mut parens = 0;
        while self.index < self.tokens.len() 
          && TOKENS_MATH.contains(&self.tokens[self.index].token) {

            if self.tokens[self.index].token == TokenType::ParenOpen { parens += 1; }
            if self.tokens[self.index].token == TokenType::ParenClose { parens -= 1; }

            if parens < 0 { break; }

            self.index += 1;
        }


        let max_index = self.index;

        let expression_slice = &self.tokens[min_index..max_index];
        
        let math_value = expression_parser::parse_expression(expression_slice);
        
        return math_value;
    }

    fn require_token(&self, offset: usize, message: &str) -> Token {
        return self.tokens.get(self.index + offset).expect(message).clone();
    }
}

