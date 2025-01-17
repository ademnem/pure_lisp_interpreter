
mod eval;
mod lexer;
mod parse;
mod shell;

use eval::*;
use lexer::*;
use parse::*;
use shell::*;



const EXIT: i8 = -1;
const CONTINUE: i8 = 0;


static OBLIST: Vec<(String, Sexpr)> = Vec::new();

fn match_command(command: String) -> i8 { 

    match command.replace(" ", "").as_str() {
        "EXIT\n" => EXIT,
        "\n" => { print!(""); CONTINUE },
        _ => { 
            // lexer
            // parser
            // eval (clone OBLIST)
            print!("{}", command); // print here
            CONTINUE
        },
    }
}


fn main() {

    pure_lisp_interpreter_message();

    loop {
        match get_command() {
            Ok(command) => if match_command(command) == EXIT { break; },
            Err(error) => print!("Error: {}", error),
        }
    }
}
