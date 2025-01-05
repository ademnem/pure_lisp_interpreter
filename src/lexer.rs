


#[derive(PartialEq, Eq)]
enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}



fn space_inputs(input: String) -> String {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
}
fn space_separate_inputs(input: &str) -> Vec<&str> {
    input
        .split_whitespace()
        .collect()
}
fn tokenize_inputs(input: Vec<&str>) -> Vec<Token> {
    
    let mut tokens: Vec<Token> = Vec::new();

    for part in input.clone() {
        match part {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let atom: Result<i64, _> = part.trim().parse();
                match atom {
                    Ok(i) => tokens.push(Token::Integer(i)),
                    Err(_) => tokens.push(Token::Symbol(part.to_string())),
                }
            },
        }
    }

    tokens
}



pub fn tokenize(input: String) -> Vec<Token> {

    let spaced_input = space_inputs(input);
    let space_separated_input: Vec<&str>  = space_separate_inputs(&spaced_input);
    tokenize_inputs(space_separated_input)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_inputs() {
        let mut input = String::from("(+)"); 
        let mut expected = String::from(" ( + ) ");
        assert_eq!(space_inputs(input), expected);

        input = String::from("()");
        expected = String::from(" (  ) ");
        assert_eq!(space_inputs(input), expected);

        input = String::from("+");
        expected = String::from("+");
        assert_eq!(space_inputs(input), expected);

        input = String::from("");
        expected = String::from("");
        assert_eq!(space_inputs(input), expected);
    }

    #[test]
    fn test_space_separate_inputs() {
        let mut input = " ( + ) ";
        let mut expected: Vec<&str> = vec!["(", "+", ")"];
        assert_eq!(space_separate_inputs(input), expected);

        input = " (  ) ";
        expected = vec!["(", ")"];
        assert_eq!(space_separate_inputs(input), expected);

        input = "+";
        expected = vec!["+"];
        assert_eq!(space_separate_inputs(input), expected);

        input = "";
        expected = Vec::new();
        assert_eq!(space_separate_inputs(input), expected);
    }

    
    fn compare_token_vectors(expected: Vec<Token>, result: Vec<Token>) -> bool {
        let comp = expected.iter().zip(&result);
        for (e, r) in comp {
            if e != r {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_tokenize_inputs() {

        let mut input: Vec<&str> = vec!["(", "+", ")"];
        let mut expected: Vec<Token> = vec![Token::LParen, Token::Symbol(String::from("+")), Token::RParen]; 
        let mut result = tokenize_inputs(input); 
        assert!(compare_token_vectors(expected, result));

        input = vec!["(", ")"];
        expected = vec![Token::LParen, Token::RParen];
        result = tokenize_inputs(input); 
        assert!(compare_token_vectors(expected, result));

        input = vec!["+"];
        expected = vec![Token::Symbol(String::from("+"))];
        result = tokenize_inputs(input); 
        assert!(compare_token_vectors(expected, result));

        input = vec!["+"];
        expected = vec![Token::Symbol(String::from("-"))];
        result = tokenize_inputs(input); 
        assert_eq!(compare_token_vectors(expected, result), false);

        input = Vec::new();
        expected = Vec::new();
        result = tokenize_inputs(input); 
        assert!(compare_token_vectors(expected, result));
    }
}
