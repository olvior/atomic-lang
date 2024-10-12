use crate::exit_message;
use crate::tokenise::{Token, TokenType};

use super::{MathValue, NodeMathAdd, NodeMathSub, NodeMathMult, NodeMathDiv, OperationType};


struct ExpressionParser {
    index: usize,
}

impl ExpressionParser {
    fn parse_sum(&mut self, tokens: &[Token]) -> MathValue {
        let mut value_1 = self.parse_product(tokens);

        while self.index < tokens.len() 
        && (tokens[self.index].token == TokenType::Plus || tokens[self.index].token == TokenType::Minus) {

            let first_token_type = &tokens[self.index].token;

            self.index += 1;

            let value_2 = self.parse_product(tokens);


            let operation = match first_token_type { 
                TokenType::Plus => { 
                    let add_node = NodeMathAdd { value_1, value_2 };
                    Box::new(OperationType::Add(add_node))
                },
                TokenType::Minus => { 
                    let sub_node = NodeMathSub { value_1, value_2 };
                    Box::new(OperationType::Sub(sub_node))
                },
                
                _ => panic!("Error while parsing expression"),
            };

            let math_operation = MathValue::Operation(operation);

            value_1 = math_operation;
        }

        return value_1;
    }

    fn parse_product(&mut self, tokens: &[Token]) -> MathValue {
        let mut value_1 = self.parse_factor(tokens);

        while self.index < tokens.len()
        && (tokens[self.index].token == TokenType::Star || tokens[self.index].token == TokenType::ForwardsSlash) {

            let first_token_type = &tokens[self.index].token;

            self.index += 1;

            let value_2 = self.parse_factor(tokens);

            let operation = match first_token_type {
                TokenType::Star => {
                    let add_node = NodeMathMult { value_1, value_2 };

                    Box::new(OperationType::Mult(add_node))
                }

                TokenType::ForwardsSlash => {
                    let add_node = NodeMathDiv { value_1, value_2 };

                    Box::new(OperationType::Div(add_node))
                }

                _ => panic!("Error while parsing"),
            };
            let math_operation = MathValue::Operation(operation);

            value_1 = math_operation;
        }

        return value_1;
    }

    fn parse_factor(&mut self, tokens: &[Token]) -> MathValue {
        if self.index >= tokens.len() {
            panic!("Expected factor");
        }
        let token = &tokens[self.index];
        self.index += 1;

        if token.token == TokenType::IntegerLit {
            return MathValue::Integer(token.clone());
        }
        else if token.token == TokenType::Identifier {
            return MathValue::Identifier(token.clone());
        }
        else if token.token == TokenType::ParenOpen {
            let math_value = self.parse_sum(tokens);

            if self.index < tokens.len() && tokens[self.index].token == TokenType::ParenClose {
                self.index += 1;
                return math_value;
            } else {
                panic!("Expected a closing paren");
            }
        } else {
            panic!("Unknown factor");
        }
    }
}


pub fn parse_expression(tokens: &[Token]) -> MathValue {
    let mut expr_parser = ExpressionParser { index: 0 };
    let math_value = expr_parser.parse_sum(tokens);
    
    if expr_parser.index != tokens.len() {
        dbg!(&tokens);
        dbg!(&tokens[expr_parser.index..]);
        panic!("Possible error, not all of the tokens for the expression were used!");
    }

    return math_value;
}

