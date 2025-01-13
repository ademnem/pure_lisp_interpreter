


#[derive(Debug, PartialEq, Eq)]
pub enum Token {
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

    for part in input { 
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

        let mut input: Vec<&str> = vec!["(", "+", ")"];
        let mut result = tokenize_inputs(input); 
        let mut expected: Vec<Token> = vec![Token::LParen, Token::Symbol(String::from("+")), Token::RParen]; 
        assert!(compare_token_vectors(result, expected));

        input = vec!["(", ")"];
        result = tokenize_inputs(input); 
        expected = vec![Token::LParen, Token::RParen];
        assert!(compare_token_vectors(result, expected));

        input = vec!["+"];
        result = tokenize_inputs(input); 
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result, expected));

        input = vec!["+"];
        result = tokenize_inputs(input); 
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result, expected));

        input = Vec::new();
        result = tokenize_inputs(input); 
        expected = Vec::new();
        assert!(compare_token_vectors(result, expected));
    }

    #[test]
    fn test_tokenize() {

        let mut input = String::from("(+)"); 
        let mut result = tokenize(input); 
        let mut expected: Vec<Token> = vec![Token::LParen, Token::Symbol(String::from("+")), Token::RParen]; 
        assert!(compare_token_vectors(result, expected));

        input = String::from("()");
        result = tokenize(input); 
        expected = vec![Token::LParen, Token::RParen];
        assert!(compare_token_vectors(result, expected));

        input = String::from("+");
        result = tokenize(input); 
        expected = vec![Token::Symbol(String::from("+"))];
        assert!(compare_token_vectors(result, expected));

        input = String::from("+");
        result = tokenize(input); 
        expected = vec![Token::Symbol(String::from("-"))];
        assert!(!compare_token_vectors(result, expected));

        input = String::new();
        result = tokenize(input); 
        expected = Vec::new();
        assert!(compare_token_vectors(result, expected));
    }
}
