mod eval;
mod lexer;
mod lisp;
mod parse;
mod shell;
mod test;

use eval::*;
use lexer::*;
use lisp::*;
use parse::*;
use shell::*;

const EXIT: i8 = -1;
const CONTINUE: i8 = 0;

fn match_command(command: String) -> i8 {
    match command.replace(" ", "").to_ascii_uppercase().as_str() {
        "EXIT" => EXIT,
        "" => CONTINUE,
        _ => {
            let mut tokens: Vec<Token> = tokenize(&command);
            let symbols: Sexpr = parse(&mut tokens);
            match evaluate(symbols, OBLIST.clone()) {
                Ok(s) => println!("{}", sexpr_to_string(&s)),
                Err(e) => println!("Error: {}", e),
            }
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
