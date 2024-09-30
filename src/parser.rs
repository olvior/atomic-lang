#[allow(dead_code)]

use crate::{TokenType, Token};

#[derive(Debug)]
pub struct NodeExit {
    pub expr: NodeExpr,
}

#[derive(Debug)]
pub struct NodeExpr {
    pub int_lit: Token,
}


pub fn parse(source: &Vec<Token>) -> Option<NodeExit> {
    for (i, t) in source.into_iter().enumerate() {
        match t.token {
            TokenType::Exit => {
                if  source.get(i+1)?.token == TokenType::ParenOpen &&
                    source.get(i+2)?.token == TokenType::IntegerLit &&
                    source.get(i+3)?.token == TokenType::ParenClose &&
                    source.get(i+4)?.token == TokenType::Semicolon
                {
                    let literal = source.get(i+2)?;

                    let expr = NodeExpr { int_lit: literal.clone() };
                    let root_exit = NodeExit { expr };

                    return Some(root_exit);
                }
            },
            _ => (),
        }
    }


    return None;
}

fn get_next(source: &Vec<Token>, token_type: TokenType, i: usize) -> Option<usize> {
    let trimmed_source = &source[i..source.len()];

    for (j, c) in trimmed_source.iter().enumerate() {
        if c.token == token_type {
            return Some(j);
        }
    }

    return None;
}
