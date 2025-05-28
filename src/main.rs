mod eval;
mod lexer;
mod lisp;
mod parse;
mod shell;
mod test;

use crate::eval::*;
use crate::lexer::*;
use crate::parse::*;
use lisp::*;
use shell::*;

const EXIT: i8 = -1;
const CONTINUE: i8 = 0;

fn match_command(command: String) -> i8 {
    match command.replace(" ", "").as_str() {
        "EXIT" => EXIT,
        "" => CONTINUE,
        _ => {
            let mut tokens: Vec<Token> = tokenize(&command);
            let symbols: Sexpr = parse(&mut tokens);
            match evaluate(symbols, OBLIST.clone()) {
                Ok(s) => match sexpr_to_string(s) {
                    Ok(o) => println!("{}", o),
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("Error: {}", e),
            }
            println!("{}", command); // print here
            CONTINUE
        }
    }
}

fn main() {
    pure_lisp_interpreter_message();

    // save declare variables here
    // how does rust borrowing work?
    loop {
        match get_command() {
            Ok(command) => {
                if match_command(command) == EXIT {
                    break;
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
