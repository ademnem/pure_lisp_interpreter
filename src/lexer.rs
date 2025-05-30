#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Integer(i64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

fn space_inputs(input: &String) -> String {
    input.replace("(", " ( ").replace(")", " ) ")
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
/* idk if i want to keep this still, there is a test i need to re-enable if i decide to keep this
fn remove_quotes(input: String) -> String {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
*/
fn tokenize_inputs(input: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for part in input {
        match part.as_str() {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let atom: Result<i64, _> = part.trim().parse();
                match atom {
                    Ok(i) => tokens.push(Token::Integer(i)),
                    Err(_) => {
                        if part.starts_with("\"") && part.ends_with("\"") {
                            tokens.push(Token::String(part));
                        } else {
                            tokens.push(Token::Symbol(part));
                        }
                    }
                }
            }
        }
    }

    tokens
}

pub fn tokenize(input: &String) -> Vec<Token> {
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
    fn test_tokenize_inputs() {
        let mut input: Vec<String> = vec![String::from("("), String::from("+"), String::from(")")];
        let mut result = tokenize_inputs(input);
        let mut expected: Vec<Token> = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result, expected));

        input = vec![String::from("("), String::from(")")];
        result = tokenize_inputs(input);
        expected = vec![Token::LParen, Token::RParen];
        assert!(compare_token_vectors(result, expected));

        input = vec![String::from("+")];
        result = tokenize_inputs(input);
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result, expected));

        input = vec![String::from("+")];
        result = tokenize_inputs(input);
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result, expected));

        input = Vec::new();
        result = tokenize_inputs(input);
        expected = Vec::new();
        assert!(compare_token_vectors(result, expected));
    }

    /*
    #[test]
    fn test_remove_quotes() {
        let mut input = String::from("\"hello\"");
        let mut result = remove_quotes(input);
        assert_eq!(result, String::from("hello"));

        input = String::from("\"he\"llo\"");
        result = remove_quotes(input);
        assert_eq!(result, String::from("he\"llo"));
    }
    */

    #[test]
    fn test_tokenize() {
        let mut input = String::from("(+)");
        let mut result = tokenize(&input);
        let mut expected: Vec<Token> = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::RParen,
        ];
        assert!(compare_token_vectors(result, expected));

        input = String::from("()");
        result = tokenize(&input);
        expected = vec![Token::LParen, Token::RParen];
        assert!(compare_token_vectors(result, expected));

        input = String::from("+");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result, expected));

        input = String::from("+");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result, expected));

        input = String::from("t");
        result = tokenize(&input);
        expected = vec![Token::Symbol(String::from("T"))];
        assert!(compare_token_vectors(result, expected));

        input = String::new();
        result = tokenize(&input);
        expected = Vec::new();
        assert!(compare_token_vectors(result, expected));

        input = String::from("\"hello\"");
        result = tokenize(&input);
        expected = vec![Token::String(String::from("\"hello\""))];
        assert!(compare_token_vectors(result, expected));
    }
}
