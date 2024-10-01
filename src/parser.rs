use crate::{TokenType, Token};

#[derive(Debug)]
pub enum NodeStatements {
    Exit(NodeStmtExit),
    Declare(NodeStmtDeclare),
}

#[derive(Debug)]
pub struct NodeProgram {
    pub statements: Vec<NodeStatements>
}

// statements
#[derive(Debug)]
pub struct NodeStmtExit {
    pub expression: NodeExpr,
}

#[derive(Debug)]
pub struct NodeStmtDeclare {
    pub identifier: Token,
    pub expression: NodeExpr,
}

// smaller
#[derive(Debug)]
pub struct NodeExpr {
    pub int_lit: Token,
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
                TokenType::Exit => { NodeStatements::Exit(self.parse_exit()) }
                TokenType::IntType => { NodeStatements::Declare(self.parse_int_assign()) }
                _ => { return program; }
            };

            program.statements.push(statement);
        }

        return program;
    }

    fn parse_exit(&mut self) -> NodeStmtExit {
        if self.require_token(1, "Could not parse exit").token != TokenType::ParenOpen {
            panic!("Expected parenthesis");
        }
        if self.require_token(3, "Could not parse exit").token != TokenType::ParenClose {
            panic!("Expected closing parenthesis");
        }
        if self.require_token(4, "Could not parse exit").token != TokenType::Semicolon {
            panic!("Expected semicolon");
        }

        // account for exit(
        self.index += 2;

        let expr = self.parse_expr();

        // account for );
        self.index += 1;

        return NodeStmtExit { expression: expr };
    }
    
    fn parse_int_assign(&mut self) -> NodeStmtDeclare {
        if self.require_token(1, "Could not parse declaration").token != TokenType::Identifier {
            panic!("Expected identifier");
        }
        if self.require_token(2, "Could not parse declaration").token != TokenType::AssignEq {
            panic!("Expected closing parenthesis");
        }
        if self.require_token(4, "Could not parse declaration").token != TokenType::Semicolon {
            panic!("Expected semicolon");
        }

        let identifier = self.require_token(1, "Could not parse declaration");

        // account for name =
        self.index += 3;

        let expr = self.parse_expr();

        // account for ;
        self.index += 1;

        return NodeStmtDeclare { identifier, expression: expr };
    }


    fn parse_expr(&mut self) -> NodeExpr {
        let expr_token = self.require_token(0, "Could not parse expression");
        if expr_token.token != TokenType::IntegerLit && expr_token.token != TokenType::Identifier {
            panic!("Could not parse the expression, expected an identifier or integer literal");
        }

        self.index += 1;

        return NodeExpr { int_lit: expr_token };
    }

    fn require_token(&self, offset: usize, message: &str) -> Token {
        return self.tokens.get(self.index + offset).expect(message).clone();
    }
}

