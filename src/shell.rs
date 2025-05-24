use std::io::{stdin, stdout, Write};
use std::result::Result;
use std::string::String;

pub fn pure_lisp_interpreter_message() {
    println!("--== Pure Lisp Interpreter ==--");
}

fn paren_balance(command: &str) -> i16 {
    let mut bal: i16 = 0;

    for c in command.chars() {
        match c {
            '(' => bal += 1,
            ')' => bal -= 1,
            _ => {}
        }
    }

    bal
}
fn rparen_is_last(input: &String) -> bool {
    // if there are inputs before the first (, only the first argument will be evaluated
    if input.starts_with('(') {
        match input.chars().last() {
            Some(c) => return c == ')',
            None => return false,
        }
    }
    true
}

fn get_command_new_line() -> String {
    let mut line = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin().read_line(&mut line).expect("unable to read");

    line
}
fn get_command_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("unable to read");

    line
}
pub fn get_command() -> Result<String, String> {
    let mut input = String::new();

    input += &get_command_new_line();
    let _ = input.pop();
    while paren_balance(&input) > 0 {
        input += &get_command_line();
        let _ = input.pop();
    }

    if paren_balance(&input) < 0 {
        return Err(String::from("too many closing parens"));
    } else if !rparen_is_last(&input) {
        println!("{input}");
        println!("{}", input.find("\n").unwrap());
        return Err(String::from("input starting with a '(' must end with ')'"));
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paren_balance() {
        let mut command = String::new();
        assert_eq!(paren_balance(&command), 0);

        command = String::from("(");
        assert_eq!(paren_balance(&command), 1);

        command = String::from(")");
        assert_eq!(paren_balance(&command), -1);

        command = String::from("()");
        assert_eq!(paren_balance(&command), 0);

        command = String::from("(hello world)");
        assert_eq!(paren_balance(&command), 0);
    }

    #[test]
    fn test_rparen_is_last() {
        let mut input = String::new();
        assert!(rparen_is_last(&input));

        input = String::from("()");
        assert!(rparen_is_last(&input));

        input = String::from("(     +     1     2)");
        assert!(rparen_is_last(&input));

        input = String::from("+ () +");
        assert!(rparen_is_last(&input));

        input = String::from("( + ) +");
        assert!(!rparen_is_last(&input));
    }
}
