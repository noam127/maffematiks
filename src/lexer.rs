
use std::fmt::Debug;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct LexingError(String);
impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for LexingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
    fn description(&self) -> &str { "Lexing Error" }
    fn cause(&self) -> Option<&dyn Error> { None }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(char),
    Number(u32),
    Paren(char),
    WhiteSpace,
    EndOfInput,
}
impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Self::Operator(op) => format!("{op}"),
            Self::Number(n) => format!("{n}"),
            Self::Paren(p) => format!("{p}"),
            Self::WhiteSpace => " ".into(),
            Self::EndOfInput => "EndOfInput".into(),
        }
    }
}

pub trait TokenStreamExt {
    fn lex(source: &str) -> Result<Self, LexingError> where Self: Sized;
}
impl TokenStreamExt for Vec<Token> {
    fn lex(source: &str) -> Result<Self, LexingError> {
        let mut result = Vec::<Token>::new();
        let mut current_num = None::<u32>;
        for c in source.chars() {
            if (c as u8) >= 48 && (c as u8) <= 57 {
                let digit = c as u32 - 48;
                if let Some(num) = &mut current_num {
                    *num *= 10;
                    *num += digit;
                } else {
                    current_num = Some(digit);
                }
                continue;
            }
            if let Some(num) = current_num {
                result.push(Token::Number(num));
                current_num = None;
            }
            result.push(match c {
                '+'|'-'|'*'|'/'|'^' => Token::Operator(c),
                '('|')' => Token::Paren(c),
                ' '|'\t' => {
                    if result.last() == Some(&Token::WhiteSpace) { continue; }
                    Token::WhiteSpace
                },
                _ => return Err(LexingError(format!("invalid character {c}"))),
            });
        }
        if let Some(num) = current_num {
            result.push(Token::Number(num));
        }
        result.push(Token::EndOfInput);
        Ok(result)
    }
}
