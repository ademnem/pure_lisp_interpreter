

use crate::lexer::*;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sexpr {
    Integer(i64),
    Symbol(String),
    List(Vec<Sexpr>),
    Lambda(String, Vec<Sexpr>),
    T,
    Nil,
}



fn parse_atom(tokens: &Vec<Token>) -> Sexpr {    
    match tokens.first().unwrap() {
        Token::Integer(i) => Sexpr::Integer(*i),
        Token::Symbol(s) => Sexpr::Symbol(s.to_string()),
        _ => Sexpr::Nil, // should never be reached
    }
}
fn is_list(tokens: &Vec<Token>) -> bool { 
    match tokens.first().unwrap() {
        Token::LParen => true,
        _ => false,
    }
}
fn parse_list(tokens: &mut Vec<Token>) -> Sexpr {
    
    let _ = tokens.pop(); // remove first RParen

    let mut list: Vec<Sexpr> = Vec::new();
    /*
    assumptions
    - parens are balanced
    - it is bound to be a symbol or an integer at this point
    */

    while !tokens.is_empty() {
        let token = tokens.pop();
        if token.is_some() {        
            match token.unwrap() {
                Token::Integer(i) => { list.push(Sexpr::Integer(i)); },
                Token::Symbol(s) => { list.push(Sexpr::Symbol(s)); },
                Token::RParen => { list.push(parse_list(tokens)); },
                Token::LParen => { return Sexpr::List(list); },   
            }
        }
    }
    Sexpr::List(list)
}


// due to the shell assume all inputs have balanced parens or only one input
pub fn parse(tokens: &mut Vec<Token>) -> Sexpr  {

    if is_list(tokens) {
        tokens.reverse();
        return parse_list(tokens);
    }
    parse_atom(tokens)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {

        let mut input: Vec<Token> = vec![Token::Integer(51)];
        let mut expected: Sexpr = Sexpr::Integer(51); 
        assert_eq!(parse_atom(&input), expected);

        input = vec![Token::Symbol(String::from("+"))];
        expected = Sexpr::Symbol(String::from("+"));
        assert_eq!(parse_atom(&input), expected);
    }

    #[test]
    fn test_is_list() {

        let mut input: Vec<Token> = vec![Token::LParen, Token::RParen]; 
        assert!(is_list(&input));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen];
        assert!(is_list(&input));

        input = vec![Token::Symbol(String::from("+"))];
        assert!(!is_list(&input));
    }


    fn unpack_list(list: &Sexpr) -> Result<&Vec<Sexpr>, &str> {

        match list {
            Sexpr::List(vec) => Ok(&vec),
            _ => Err(""),
        }
    }
    fn equal_object_lists(result: &Sexpr, expected: &Sexpr) -> bool {

        let vec_result = unpack_list(&result).unwrap();

        let vec_expected = unpack_list(&expected).unwrap();
        let comp = vec_result.iter().zip(vec_expected);
        
        for (r, e) in comp {  
            /* // when the test fails, this prints out
            match r {
                Sexpr::Symbol(_) => print!("s "),
                Sexpr::Integer(_) => print!("i "),
                Sexpr::List(_) => print!("l "),
                _ => {},
            }
            match e {
                Sexpr::Symbol(_) => println!("s"),
                Sexpr::Integer(_) => println!("i"),
                Sexpr::List(_) => println!("l"),
                _ => {},
            }
            */ 
            match r {
                Sexpr::List(_) => if !equal_object_lists(r, e) { return false; },
                _ => if r != e { return false; },
            }
        }

        true
    }

    #[test]
    fn test_parse_list() {

        let mut input: Vec<Token> = vec![Token::LParen, Token::RParen];
        input.reverse();
        let mut result: Sexpr = parse_list(&mut input);
        let mut output: Vec<Sexpr> = Vec::new();
        let mut expected: Sexpr = Sexpr::List(output);
        assert!(equal_object_lists(&result, &expected));


        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen];
        input.reverse();
        result = parse_list(&mut input);
        output = vec![Sexpr::Symbol(String::from("+")), Sexpr::Integer(1), Sexpr::Integer(1)];
        expected = Sexpr::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen, Token::Integer(1),  Token::RParen];
        input.reverse();
        result = parse_list(&mut input);
        let output2 = vec![Sexpr::Symbol(String::from("+")), expected, Sexpr::Integer(1)];
        let expected2 = Sexpr::List(output2);
        assert!(equal_object_lists(&result, &expected2));
    }

    #[test]
    fn test_parse() {

        let mut input: Vec<Token> = vec![Token::Integer(51)];
        let mut result: Sexpr = Sexpr::Integer(51); 
        assert_eq!(parse(&mut input), result);

        input = vec![Token::Symbol(String::from("+"))];
        result = Sexpr::Symbol(String::from("+"));
        assert_eq!(parse(&mut input), result);

        input = vec![Token::LParen, Token::RParen];
        result = parse(&mut input);
        let mut output: Vec<Sexpr> = Vec::new();
        let mut expected: Sexpr = Sexpr::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen];
        result = parse(&mut input);
        output = vec![Sexpr::Symbol(String::from("+")), Sexpr::Integer(1), Sexpr::Integer(1)];
        expected = Sexpr::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen, Token::Integer(1),  Token::RParen];
        result = parse(&mut input);
        let output2 = vec![Sexpr::Symbol(String::from("+")), expected, Sexpr::Integer(1)];
        let expected2 = Sexpr::List(output2);
        assert!(equal_object_lists(&result, &expected2));
    }
}
