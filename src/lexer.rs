use std::{iter::Peekable, vec::IntoIter};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}
#[allow(dead_code)] // should i allow this? it's useful for testing
pub fn token_to_string(v: &Token) -> String {
    match v {
        Token::Integer(i) => i.to_string(),
        Token::Float(f) => f.to_string(),
        Token::String(s) => s.to_string(),
        Token::Symbol(s) => s.to_string(),
        Token::LParen => String::from("("),
        Token::RParen => String::from(")"),
    }
}

fn space_inputs(input: &String) -> String {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ")
}
fn space_separate_inputs(input: &str) -> Vec<String> {
    let inputs: Vec<&str> = input.split_whitespace().collect();
    let mut outputs: Vec<String> = Vec::new();
    for i in inputs.iter() {
        if i.starts_with("\"") && i.ends_with("\"") {
            outputs.push(i.to_string());
        } else {
            outputs.push(i.to_uppercase());
        }
    }
    outputs
}

fn get_datatype(input: &String) -> Token {
    match input.as_str() {
        "(" => Token::LParen,
        ")" => Token::RParen,
        _ => match input.trim().parse() {
            Ok(i) => Token::Integer(i),
            Err(_) => match input.trim().parse::<f64>() {
                Ok(f) => Token::Float(f),
                Err(_) => {
                    if input.starts_with("\"") && input.ends_with("\"") {
                        Token::String(input.to_string())
                    } else {
                        Token::Symbol(input.to_string())
                    }
                }
            },
        },
    }
}

// only one that actually moves the pointer for the iterator
fn tokenize_input(
    iter: &mut Peekable<IntoIter<String>>,
    tokens: &mut Vec<Token>,
) -> Result<Vec<Token>, String> {
    if iter.peek() == None {
        return Err(String::from(
            "tokenize_input: input is required to tokenize",
        ));
    }
    let data = iter.next().unwrap();
    match get_datatype(&data) {
        Token::LParen => {
            tokens.push(Token::LParen);
            tokenize_list(iter, tokens);
        }
        Token::Symbol(s) => match s.as_str() {
            /*
            "." => {
                // needs to check prev through tokens
                // needs to check next through iter.peek().unwrap()
                let atom = iter.next();
                let rparen = iter.peek();
                if tokens.last() == None {
                    return Err(String::from(
                        "tokenize_input: . must be preceded by an atom or list",
                    ));
                }
                if atom == None
                    || atom == Some(String::from("("))
                    || atom == Some(String::from(")"))
                    || atom == Some(String::from("."))
                    || atom == Some(String::from("'"))
                {
                    return Err(String::from(
                        "tokenize_input: . must be followed by an atom or list",
                    ));
                }
                if rparen != Some(&String::from(")")) {
                    return Err(String::from("tokenize_input: atom must be followed by )"));
                }

                let atom = atom.unwrap();
                tokens.push(get_datatype(atom));
            }
            */
            "'" => {
                // can't be followed by .
                // just replace it with ( QUOTE input )
                // peek next and see if it is a list or atom
                // then call tokenize_input or tokenize_list
            }
            _ => tokens.push(Token::Symbol(s)),
        },
        t => tokens.push(t),
    }

    Ok(tokens.to_vec())
}
fn tokenize_list(
    iter: &mut Peekable<IntoIter<String>>,
    tokens: &mut Vec<Token>,
) -> Result<Vec<Token>, String> {
    while iter.peek() != None {
        let part = iter.peek().unwrap();
        match get_datatype(part) {
            Token::RParen => {
                iter.next();
                tokens.push(Token::Symbol(String::from("NIL")));
                tokens.push(Token::RParen);
            }
            _ => match tokenize_input(iter, tokens) {
                Err(e) => return Err(e),
                Ok(_) => {} // returned Vec<Token> should be the same as tokens
            },
        }
    }
    Ok(tokens.to_vec())
}
fn tokenize_inputs(input: Vec<String>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter: Peekable<IntoIter<String>> = input.into_iter().peekable();

    tokenize_input(&mut iter, &mut tokens)
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {
    let spaced_input = space_inputs(&input);
    let space_separated_input: Vec<String> = space_separate_inputs(&spaced_input);

    tokenize_inputs(space_separated_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_inputs() {
        let mut input = String::from("(+)");
        let mut expected = String::from(" ( + ) ");
        assert_eq!(space_inputs(&input), expected);

        input = String::from("()");
        expected = String::from(" (  ) ");
        assert_eq!(space_inputs(&input), expected);

        input = String::from("+");
        expected = String::from("+");
        assert_eq!(space_inputs(&input), expected);

        input = String::from("");
        expected = String::from("");
        assert_eq!(space_inputs(&input), expected);

        input = String::from("'(1)");
        expected = String::from(" '  ( 1 ) ");
        assert_eq!(space_inputs(&input), expected);
    }

    #[test]
    fn test_space_separate_inputs() {
        let mut input = " ( + ) ";
        let mut expected: Vec<String> =
            vec![String::from("("), String::from("+"), String::from(")")];
        assert_eq!(space_separate_inputs(input), expected);

        input = " (  ) ";
        expected = vec![String::from("("), String::from(")")];
        assert_eq!(space_separate_inputs(input), expected);

        input = "+";
        expected = vec![String::from("+")];
        assert_eq!(space_separate_inputs(input), expected);

        input = "";
        expected = Vec::new();
        assert_eq!(space_separate_inputs(input), expected);
    }

    fn compare_token_vectors(result: Vec<Token>, expected: Vec<Token>) -> bool {
        let comp = result.iter().zip(&expected);
        for (r, e) in comp {
            if r != e {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_get_datatype() {
        let mut input = String::from("(");
        assert_eq!(get_datatype(&input), Token::LParen);

        input = String::from(")");
        assert_eq!(get_datatype(&input), Token::RParen);

        input = String::from("1");
        assert_eq!(get_datatype(&input), Token::Integer(1));

        input = String::from("234.1");
        assert_eq!(get_datatype(&input), Token::Float(234.1));

        input = String::from("\"hello\"");
        assert_eq!(
            get_datatype(&input),
            Token::String(String::from("\"hello\""))
        );
    }

    #[test]
    fn test_tokenize_inputs() {
        let mut input: Vec<String> = vec![String::from("("), String::from("+"), String::from(")")];
        let mut result = tokenize_inputs(input);
        let mut expected: Vec<Token> = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Symbol(String::from("NIL")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = vec![String::from("("), String::from(")")];
        result = tokenize_inputs(input);
        expected = vec![
            Token::LParen,
            Token::Symbol(String::from("NIL")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = vec![String::from("+")];
        result = tokenize_inputs(input);
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = vec![String::from("+")];
        result = tokenize_inputs(input);
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result.unwrap(), expected));

        input = Vec::new();
        result = tokenize_inputs(input);
        assert_eq!(
            result,
            Err(String::from(
                "tokenize_input: input is required to tokenize",
            ))
        );

        input = vec![String::from("1")];
        result = tokenize_inputs(input);
        expected = vec![Token::Integer(1)];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = vec![String::from("1.1")];
        result = tokenize_inputs(input);
        expected = vec![Token::Float(1.1)];
        assert!(compare_token_vectors(result.unwrap(), expected));

        /*
        input = vec![
            String::from("("),
            String::from("1.1"),
            String::from("."),
            String::from("1"),
            String::from(")"),
        ];
        result = tokenize_inputs(input);
        expected = vec![
            Token::LParen,
            Token::Float(1.1),
            Token::Integer(1),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = vec![
            String::from("("),
            String::from("1.1"),
            String::from("."),
            String::from(")"),
        ];
        result = tokenize_inputs(input);
        assert_eq!(
            result,
            Err(String::from(
                "tokenize_inputs: . must be followed by an atom"
            ))
        );

        input = vec![
            String::from("("),
            String::from("1.1"),
            String::from("."),
            String::from("1.1"),
            String::from("1.1"),
            String::from(")"),
        ];
        result = tokenize_inputs(input);
        assert_eq!(
            result,
            Err(String::from("tokenize_inputs: atom must be followed by )"))
        );
        */
    }

    #[test]
    fn test_tokenize() {
        let mut input = String::from("(+)");
        let mut result = tokenize(&input);
        let mut expected: Vec<Token> = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Symbol(String::from("NIL")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = String::from("()");
        result = tokenize(&input);
        expected = vec![
            Token::LParen,
            Token::Symbol(String::from("NIL")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = String::from("+");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = String::from("+");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result.unwrap(), expected));

        input = String::from("t");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("T"))];
        assert!(compare_token_vectors(result.unwrap(), expected));

        input = String::new();
        result = tokenize(&input);
        assert_eq!(
            result,
            Err(String::from(
                "tokenize_input: input is required to tokenize",
            ))
        );

        input = String::from("\"hello\"");
        result = tokenize(&input);
        expected = vec![Token::String(String::from("\"hello\""))];
        assert!(compare_token_vectors(result.unwrap(), expected));
    }
}
