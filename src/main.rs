

mod parse;
mod shell;
use parse::*;
use shell::*;


fn main() {

    pure_lisp_interpreter_message();

    loop {
        match get_command() {
            Ok(command) => {
                if match_command(command) == 0 {
                    break;
                }
            }
            Err(error) => print!("{}", error),
        }
    }


}
