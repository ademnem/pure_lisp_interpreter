
mod lexer;
mod parse;
mod shell;

use lexer::*;
use parse::*;
use shell::*;



fn main() {
    pure_lisp_interpreter_message();

    loop {
        match get_command() {
            Ok(command) => if match_command(command) == EXIT { break; },
            Err(error) => print!("{}", error),
        }
    }
}
