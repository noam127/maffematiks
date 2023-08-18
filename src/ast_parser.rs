use std::error::Error;
use std::fmt;
use crate::ast_parser::Expression::*;
use crate::values::Value;
use crate::lexer::Token;
use crate::values::Value::*;

#[derive(Debug)]
pub struct ParsingError(String);
impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for ParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
    fn description(&self) -> &str { "Parsing Error" }
    fn cause(&self) -> Option<&dyn Error> { None }
}

#[derive(Debug)]
pub enum Expression {
    Binop {
        operation: char,
        operands: Box<(Expression, Expression)>,
    },
    Signed {
        sign: char,
        expression: Box<Expression>,
    },
    Parenthesized(Box<Expression>),
    SingleValue(Value),
}
impl Expression {
    fn skip_whitespace(token_stream: &[Token], index: &mut usize) {
        if token_stream[*index] == Token::WhiteSpace {
            *index += 1;
        }
    }
    fn parse_single_value(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        if let Token::Number(n) = token_stream[*index] {
            *index += 1;
            Ok(SingleValue(Natural(n as i32)))
        } else {
            let repr = token_stream[*index].to_string();
            Err(ParsingError(format!("expected a number, found '{repr}'")))
        }
    }
    fn parse_parenthesized(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        if token_stream[*index] == Token::Paren('(') {
            let mut count = 1;
            *index += 1;
            let old_index = *index;
            while count != 0 {
                if token_stream[*index] == Token::EndOfInput {
                    return Err(ParsingError("input is missing a closing paren".into()));
                }
                if let Token::Paren(p) = token_stream[*index] {
                    if p == '(' { count += 1; }
                    else if p == ')' { count -= 1; }
                }
                *index += 1;
            }
            let mut inner_expression= token_stream[old_index..*index-1].iter().cloned().collect::<Vec<Token>>();
            inner_expression.push(Token::EndOfInput);
            Ok(Parenthesized(Box::new(Self::parse_additive(&inner_expression[..], &mut 0)?)))
        } else {
            Self::parse_single_value(token_stream, index)
        }
    }
    fn parse_signed(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        if let &Token::Operator(op) = &token_stream[*index]{
            if op == '+' || op == '-' {
                *index += 1;
                return Ok(Signed {
                    sign: op,
                    expression: Box::new(Self::parse_parenthesized(token_stream, index)?),
                });
            }
        }
        Self::parse_parenthesized(token_stream, index)
    }
    fn parse_power(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        let mut left = Self::parse_signed(token_stream, index)?;
        let mut right: Self;
        loop {
            Self::skip_whitespace(token_stream, index);
            match &token_stream[*index] {
                &Token::Operator('^') => {
                    *index += 1;
                    Self::skip_whitespace(token_stream, index);
                    right = Self::parse_signed(token_stream, index)?;
                    left = Binop {
                        operation: '^',
                        operands: Box::new((left, right)),
                    };
                }
                _ => return Ok(left)
            }
        }
    }
    fn parse_multiplicative(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        let mut left = Self::parse_power(token_stream, index)?;
        let mut right: Self;
        loop {
            Self::skip_whitespace(token_stream, index);
            match &token_stream[*index] {
                &Token::Operator(op) if op == '*' || op == '/' => {
                    *index += 1;
                    Self::skip_whitespace(token_stream, index);
                    right = Self::parse_power(token_stream, index)?;
                    left = Binop {
                        operation: op,
                        operands: Box::new((left, right)),
                    };
                }
                _ => return Ok(left)
            }
        }
    }
    fn parse_additive(token_stream: &[Token], index: &mut usize) -> Result<Self, ParsingError> {
        Self::skip_whitespace(token_stream, index);
        let mut left = Self::parse_multiplicative(token_stream, index)?;
        let mut right: Self;
        loop {
            Self::skip_whitespace(token_stream, index);
            match &token_stream[*index] {
                &Token::EndOfInput => return Ok(left),
                &Token::Operator(op) if op == '+' || op == '-' => {
                    *index += 1;
                    Self::skip_whitespace(token_stream, index);
                    right = Self::parse_multiplicative(token_stream, index)?;
                    left = Binop {
                        operation: op,
                        operands: Box::new((left, right)),
                    };
                }
                other => {
                    let repr = other.to_string();
                    return Err(ParsingError(format!("expected operator, found '{repr}' instead.")))
                }
            }
        }
    }
    pub fn parse(token_stream: &Vec<Token>) -> Result<Self, ParsingError> {
        Self::parse_additive(token_stream, &mut 0)
    }

    pub fn simplify(self) -> Self {
        match self {
            SingleValue(_) => self,
            Signed { sign, expression } => {
                let child = expression.simplify();
                if let SingleValue(n) = child {
                    if sign == '-' {
                        SingleValue(-n)
                    } else {
                        SingleValue(n)
                    }
                } else {
                    Signed { sign, expression: Box::new(child) }
                }
            }
            Parenthesized(expression) => expression.simplify(),
            Binop { operation, operands } => {
                let left = (*operands).0.simplify();
                let right = (*operands).1.simplify();
                if let (&SingleValue(left_val), &SingleValue(right_val)) = (&left, &right) {
                    return SingleValue(match operation {
                        '+' => left_val + right_val,
                        '-' => left_val - right_val,
                        '*' => left_val * right_val,
                        '/' => left_val / right_val,
                        _ => Undefined,
                    })
                } else {
                    Binop { operation, operands: Box::new((left, right))}
                }
            }
        }
    }
}