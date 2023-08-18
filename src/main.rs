use std::io::Write;
use crate::ast_parser::Expression;
use crate::lexer::{Token, TokenStreamExt};

mod lexer;
mod ast_parser;
mod values;

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!(">> ");
        let _  = std::io::stdout().flush();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("stdin error");
            continue;
        }
        if let Some('\n') = input.chars().next_back() { input.pop(); }
        if let Some('\r') = input.chars().next_back() { input.pop(); }

        if &input == "exit" { return }
        
        let tokenization_result = Vec::<Token>::lex(&input);
        if let Err(lexing_error) = tokenization_result {
            eprintln!("{lexing_error}");
            continue;
        }
        let tokens = tokenization_result.unwrap();
        println!("Tokens:\n{:?}", tokens);


        let parsing_result = Expression::parse(&tokens);
        if let Err(parsing_error) = parsing_result {
            eprintln!("{parsing_error}");
            continue;
        }
        let ast = parsing_result.unwrap();
        println!("Tree:\n{:#?}", ast);
        println!("Simplified:\n{:#?}", ast.simplify());
    }
}
