use crate::tokenise::{Token, TokenType};

#[derive(Debug)]
pub enum MathValue {
    Integer(Token),
    Identifier(Token),
    Operation(Box<OperationType>),
}

#[derive(Debug)]
pub enum OperationType {
    Add(NodeMathAdd),
    Sub(NodeMathSub),
    Mult(NodeMathMult),
    Div(NodeMathDiv),
}

#[derive(Debug)]
pub struct NodeMathAdd {
    pub value_1: MathValue,
    pub value_2: MathValue,
}

#[derive(Debug)]
pub struct NodeMathSub {
    pub value_1: MathValue,
    pub value_2: MathValue,
}

#[derive(Debug)]
pub struct NodeMathMult {
    pub value_1: MathValue,
    pub value_2: MathValue,
}

#[derive(Debug)]
pub struct NodeMathDiv {
    pub value_1: MathValue,
    pub value_2: MathValue,
}

#[derive(Debug)]
pub struct NodeMathNegate {
    pub value: MathValue,
}

pub const TOKENS_MATH: [TokenType; 8] = [
    TokenType::ParenOpen,
    TokenType::ParenClose,

    TokenType::Plus,
    TokenType::Minus,
    TokenType::Star,
    TokenType::ForwardsSlash,

    TokenType::IntegerLit,

    TokenType::Identifier,
];

pub const TOKENS_OPERANDS: [TokenType; 2] = [
    TokenType::IntegerLit,
    TokenType::Identifier,
];


pub const TOKENS_OPERATORS: [TokenType; 6] = [
    TokenType::ParenOpen,
    TokenType::ParenClose,

    TokenType::Plus,
    TokenType::Minus,
    TokenType::Star,
    TokenType::ForwardsSlash,
];

