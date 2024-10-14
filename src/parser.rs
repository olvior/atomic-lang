use core::panic;
use std::process::exit;

use crate::{TokenType, Token, exit_message};

mod expression_parser;

pub mod math;
use math::*;

pub use math::MathValue;

#[derive(Debug)]
pub enum NodeStatements {
    Exit(NodeStmtExit),
    PutChar(NodeStmtPutChar),

    Declare(NodeStmtDeclare),
    Set(NodeStmtSet),
    
    Function(NodeStmtFunction),
    FunctionCall(NodeStmtFunctionCall),
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
pub struct NodeStmtPutChar {
    pub expression: MathValue,
}

#[derive(Debug)]
pub struct NodeStmtDeclare {
    pub identifier: Token,
    pub expression: Option<MathValue>,
}

#[derive(Debug)]
pub struct NodeStmtSet {
    pub identifier: Token,
    pub expression: MathValue,
}

#[derive(Debug)]
pub struct NodeStmtFunction {
    pub identifier: Token,
    pub args: Vec<NodeStmtDeclare>,
    pub scope: NodeProgram,
}

#[derive(Debug)]
pub struct NodeStmtFunctionCall {
    pub identifier: Token,
    pub args: Vec<NodeStmtDeclare>,
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
                TokenType::PutChar => NodeStatements::PutChar(self.parse_putchar()),
                TokenType::IntType => NodeStatements::Declare(self.parse_int_assign()),
                TokenType::Identifier => {
                    if self.index + 1 >= self.tokens.len() {
                        panic!("Expected something after identifier");
                    }

                    if self.tokens[self.index+1].token == TokenType::ParenOpen {
                        NodeStatements::FunctionCall(self.parse_func_call())
                    } else {
                        NodeStatements::Set(self.parse_set_var())
                    }
                },
                TokenType::Function => NodeStatements::Function(self.parse_function()),
                _ => { dbg!(token); exit_message("Invalid expression"); println!("happy"); return program; }
            };

            program.statements.push(statement);
        }

        return program;
    }

    fn parse_scope(&mut self) -> NodeProgram {
        if self.require_token(0, "Expected `{` in scope").token != TokenType::BraceOpen {
            exit_message("Expected `{` in scope");
        }
        self.index += 1;

        let start_index = self.index;
        // the first thing we do is to look for the end of the scope
        let mut end_index = start_index;
        let mut brace_count = 1;
        while end_index < self.tokens.len() {
            let token = &self.tokens[end_index].token;
            end_index += 1;
            if *token == TokenType::BraceOpen {
                brace_count += 1;
            } else if *token == TokenType::BraceClose {
                brace_count -= 1;
            }

            if brace_count == 0 {
                break;
            }
        }

        if brace_count != 0 {
            exit_message("Expected closing brace in scope");
        }

        let new_tokens = self.tokens[start_index..end_index - 1].to_vec();

        let mut new_parser = Parser { tokens: new_tokens, index: 0 };

        let program = new_parser.parse();

        self.index = end_index;

        return program;
    }

    fn parse_function(&mut self) -> NodeStmtFunction {
        if self.require_token(1, "Could not parse function").token != TokenType::Identifier {
            exit_message("Expected identifier");
        }
        if self.require_token(2, "Could not parse function").token != TokenType::ParenOpen {
            exit_message("Expected `(`");
        }
        
        let identifier = self.tokens[self.index + 1].clone();

        // account for: fn test(
        self.index += 3;

        let mut args: Vec<NodeStmtDeclare> = vec!();
        while self.require_token(0, "Expected cloing paren").token != TokenType::ParenClose {
            args.push(self.parse_int_assign());
        }

        // now we finished all the args
        // so we call parse scope
        self.index += 1;
        let scope = self.parse_scope();

        let function_stmt = NodeStmtFunction { identifier, args, scope };

        return function_stmt;
    }

    fn parse_func_call(&mut self) -> NodeStmtFunctionCall {
        if self.require_token(1, "Could not parse function").token != TokenType::ParenOpen {
            exit_message("Expected `(`");
        }
        if self.require_token(2, "Could not parse function").token != TokenType::ParenClose {
            exit_message("Expected `_`");
        }
        if self.require_token(3, "Could not parse function").token != TokenType::Semicolon {
            exit_message("Expected `;`");
        }

        let identifier = self.tokens[self.index].clone();
        let args = vec!();

        // account for: test();
        self.index += 4;

        let function_call_stmt = NodeStmtFunctionCall { identifier, args };

        return function_call_stmt;
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
    
    fn parse_putchar(&mut self) -> NodeStmtPutChar {
        if self.require_token(1, "Could not parse putchar").token != TokenType::ParenOpen {
            exit_message("Expected parenthesis");
        }

        // account for putchar(
        self.index += 2;

        let expr = self.parse_expr();
        
        if self.require_token(0, "Could not parse putchar").token != TokenType::ParenClose {
            exit_message("Expected closing parenthesis");
        }
        if self.require_token(1, "Could not parse putchar").token != TokenType::Semicolon {
            exit_message("Expected semicolon");
        }

        // account for );
        self.index += 2;

        return NodeStmtPutChar { expression: expr };
    }
    
    
    fn parse_int_assign(&mut self) -> NodeStmtDeclare {
        if self.require_token(1, "Could not parse declaration").token != TokenType::Identifier {
            exit_message("Expected identifier");
        }
        let identifier = self.require_token(1, "Could not parse declaration");

        // initial value is optional
        if self.require_token(2, "Could not parse declaration").token != TokenType::AssignEq {
            // it's ok if it doesn't exist
            // we just mark the expression as None
            
            // account for int name
            self.index += 2;
            if self.require_token(0, "Could not parse declaration").token != TokenType::Semicolon {
                exit_message("Expected semicolon");
            }
            self.index += 1;
            return NodeStmtDeclare { identifier, expression: None }
        }


        // account for int name =
        self.index += 3;

        let expr = self.parse_expr();


        if self.require_token(0, "Could not parse declaration").token != TokenType::Semicolon {
            exit_message("Expected semicolon");
        }

        // account for ;
        self.index += 1;

        return NodeStmtDeclare { identifier, expression: Some(expr) };
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

