use crate::{errors::Error, exit_message, Token, TokenType};

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
    pub fn parse(&mut self) -> Result<NodeProgram, Error> {
        let mut program = NodeProgram { statements: vec!() };

        while self.index < self.tokens.len() {
            let token = &self.tokens[self.index];

            let statement = match token.token {
                TokenType::Exit => NodeStatements::Exit(self.parse_exit()?),
                TokenType::PutChar => NodeStatements::PutChar(self.parse_putchar()?),
                TokenType::IntType => NodeStatements::Declare(self.parse_int_assign()?),
                TokenType::Identifier => {
                    if self.require_token(1, TokenType::ParenOpen).is_ok() {
                        NodeStatements::FunctionCall(self.parse_func_call()?)
                    } else {
                        NodeStatements::Set(self.parse_set_var()?)
                    }
                },
                TokenType::Function => NodeStatements::Function(self.parse_function()?),
                _ => { 
                    return Err ( Error { line: token.line, msg: format!("Expected a valid statement, found {}", token.info) })
                }
            };

            program.statements.push(statement);
        }

        Ok( program )
    }

    fn parse_scope(&mut self) -> Result<NodeProgram, Error> {
        let _brace = self.require_token(0, TokenType::BraceOpen)?;

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
            return Err( Error { line: self.tokens.last().unwrap().line, msg: "Expected closing brace `}`, the issue may potentially be earlier".to_string() })
        }

        let new_tokens = self.tokens[start_index..end_index - 1].to_vec();

        let mut new_parser = Parser { tokens: new_tokens, index: 0 };

        let program = new_parser.parse();

        self.index = end_index;

        return program;
    }

    fn parse_function(&mut self) -> Result<NodeStmtFunction, Error> {
        let identifier = self.require_token(1, TokenType::NoToken)?;
        let _paren = self.require_token(2, TokenType::ParenOpen)?;
        
        // account for: fn test(
        self.index += 3;

        let mut args: Vec<NodeStmtDeclare> = vec!();
        while self.require_token(0, TokenType::ParenClose).is_err() {
            args.push(self.parse_int_assign()?);
        }

        // now we finished all the args
        // so we call parse scope
        self.index += 1;
        let scope = self.parse_scope()?;

        let function_stmt = NodeStmtFunction { identifier, args, scope };

        Ok( function_stmt )
    }

    fn parse_func_call(&mut self) -> Result<NodeStmtFunctionCall, Error> {
        let _paren = self.require_token(1, TokenType::ParenOpen)?;
        let _paren = self.require_token(2, TokenType::ParenClose)?;
        let _semi  = self.require_token(3, TokenType::Semicolon)?;

        let identifier = self.tokens[self.index].clone();
        let args = vec!();

        // account for: test();
        self.index += 4;

        let function_call_stmt = NodeStmtFunctionCall { identifier, args };

        Ok( function_call_stmt )
    }

    fn parse_exit(&mut self) -> Result<NodeStmtExit, Error> {
        let _paren = self.require_token(1, TokenType::ParenOpen)?;

        // account for exit(
        self.index += 2;

        let expr = self.parse_expr()?;
        
        let _paren = self.require_token(0, TokenType::ParenClose)?;
        let _semi = self.require_token(1, TokenType::Semicolon)?;

        // account for );
        self.index += 2;

        Ok( NodeStmtExit { expression: expr } )
    }
    
    fn parse_putchar(&mut self) -> Result<NodeStmtPutChar, Error> {
        let _paren = self.require_token(1, TokenType::ParenOpen)?;

        // account for putchar(
        self.index += 2;

        let expr = self.parse_expr()?;
        
        let _paren = self.require_token(0, TokenType::ParenClose)?;
        let _semi = self.require_token(1, TokenType::Semicolon)?;

        // account for );
        self.index += 2;

        Ok( NodeStmtPutChar { expression: expr } )
    }
    
    
    fn parse_int_assign(&mut self) -> Result<NodeStmtDeclare, Error> {
        let identifier = self.require_token(1, TokenType::Identifier)?;

        // initial value is optional
        if self.require_token(2, TokenType::AssignEq).is_err() {
            // it's ok if it doesn't exist
            // we just mark the expression as None
            
            // account for int name
            self.index += 2;

            let _semi_colon = self.require_token(0, TokenType::Semicolon)?;

            self.index += 1;
            return Ok( NodeStmtDeclare { identifier, expression: None } );
        }


        // account for int name =
        self.index += 3;

        let expr = self.parse_expr()?;


        let _semi_colon = self.require_token(0, TokenType::Semicolon)?;

        // account for ;
        self.index += 1;

        Ok( NodeStmtDeclare { identifier, expression: Some(expr) } )
    }

    fn parse_set_var(&mut self) -> Result<NodeStmtSet, Error> {
        let _equal_sign = self.require_token(0, TokenType::AssignEq)?;
        let identifier = self.require_token(1, TokenType::NoToken)?;

        // account for name =
        self.index += 2;

        let expr = self.parse_expr()?;
        
        let _semi_colon = self.require_token(0, TokenType::Semicolon)?;
        // account for ;
        self.index += 1;

        Ok( NodeStmtSet { identifier, expression: expr } )
    }


    fn parse_expr(&mut self) -> Result<MathValue, Error> {
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

        if expression_slice.len() == 0 {
            return Err ( Error { line: self.tokens[self.index].line, msg: "Expression is empty".to_string() } )
        }
        
        let math_value = expression_parser::parse_expression(expression_slice);
        
        return math_value;
    }

    /// Returns the token at an offset and makes sure it is of a certain type, use
    /// `TokenType::NoToken` to allow for any type
    fn require_token(&self, offset: usize, token_type: TokenType) -> Result<Token, Error> {
        if let Some(token) = self.tokens.get(self.index + offset) {
            if token_type == TokenType::NoToken {
                return Ok(token.clone())
            }
            if token.token != token_type {
                return Err( Error { line: token.line, msg: format!("Expected {:?}, found {:?}", token_type, token.token) });
            }
            
            Ok(token.clone())
        } else {
            Err( Error { line: self.tokens.last().expect("Empty file").line, msg: format!("Expected another token") })
        }
    }
}

