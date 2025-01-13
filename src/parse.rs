

use crate::lexer::*;


#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
    Void,
}



fn parse_atom(tokens: &Vec<Token>) -> Object {    
    match tokens.first().unwrap() {
        Token::Integer(i) => Object::Integer(*i),
        Token::Symbol(s) => Object::Symbol(s.to_string()),
        _ => Object::Void, // should never be reached
    }
}
fn is_list(tokens: &Vec<Token>) -> bool { 
    match tokens.first().unwrap() {
        Token::LParen => true,
        _ => false,
    }
}
fn parse_list(tokens: &mut Vec<Token>) -> Object {
    
    let _ = tokens.pop(); // remove first RParen

    let mut list: Vec<Object> = Vec::new();
    /*
    assumptions
    - parens are balanced
    - it is bound to be a symbol or an integer at this point
    */

    while !tokens.is_empty() {
        let token = tokens.pop();
        if token.is_some() {        
            match token.unwrap() {
                Token::Integer(i) => { list.push(Object::Integer(i)); },
                Token::Symbol(s) => { list.push(Object::Symbol(s)); },
                Token::RParen => { list.push(parse_list(tokens)); },
                Token::LParen => { return Object::List(list); },   
            }
        }
    }
    Object::List(list)
}


// due to the shell assume all inputs have balanced parens or only one input
pub fn parse(tokens: &mut Vec<Token>) -> Object  {

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
        let mut expected: Object = Object::Integer(51); 
        assert_eq!(parse_atom(&input), expected);

        input = vec![Token::Symbol(String::from("+"))];
        expected = Object::Symbol(String::from("+"));
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


    fn unpack_list(list: &Object) -> Result<&Vec<Object>, &str> {

        match list {
            Object::List(vec) => Ok(&vec),
            _ => Err(""),
        }
    }
    fn equal_object_lists(result: &Object, expected: &Object) -> bool {

        let vec_result = unpack_list(&result).unwrap();

        let vec_expected = unpack_list(&expected).unwrap();
        let comp = vec_result.iter().zip(vec_expected);
        
        for (r, e) in comp {  
            /* // when the test fails, this prints out
            match r {
                Object::Symbol(_) => print!("s "),
                Object::Integer(_) => print!("i "),
                Object::List(_) => print!("l "),
                _ => {},
            }
            match e {
                Object::Symbol(_) => println!("s"),
                Object::Integer(_) => println!("i"),
                Object::List(_) => println!("l"),
                _ => {},
            }
            */ 
            match r {
                Object::List(_) => if !equal_object_lists(r, e) { return false; },
                _ => if r != e { return false; },
            }
        }

        true
    }

    #[test]
    fn test_parse_list() {

        let mut input: Vec<Token> = vec![Token::LParen, Token::RParen];
        input.reverse();
        let mut result: Object = parse_list(&mut input);
        let mut output: Vec<Object> = Vec::new();
        let mut expected: Object = Object::List(output);
        assert!(equal_object_lists(&result, &expected));


        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen];
        input.reverse();
        result = parse_list(&mut input);
        output = vec![Object::Symbol(String::from("+")), Object::Integer(1), Object::Integer(1)];
        expected = Object::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen, Token::Integer(1),  Token::RParen];
        input.reverse();
        result = parse_list(&mut input);
        let output2 = vec![Object::Symbol(String::from("+")), expected, Object::Integer(1)];
        let expected2 = Object::List(output2);
        assert!(equal_object_lists(&result, &expected2));
    }

    #[test]
    fn test_parse() {

        let mut input: Vec<Token> = vec![Token::Integer(51)];
        let mut result: Object = Object::Integer(51); 
        assert_eq!(parse(&mut input), result);

        input = vec![Token::Symbol(String::from("+"))];
        result = Object::Symbol(String::from("+"));
        assert_eq!(parse(&mut input), result);

        input = vec![Token::LParen, Token::RParen];
        result = parse(&mut input);
        let mut output: Vec<Object> = Vec::new();
        let mut expected: Object = Object::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen];
        result = parse(&mut input);
        output = vec![Object::Symbol(String::from("+")), Object::Integer(1), Object::Integer(1)];
        expected = Object::List(output);
        assert!(equal_object_lists(&result, &expected));

        input = vec![Token::LParen, Token::Symbol(String::from("+")), Token::LParen, Token::Symbol(String::from("+")), Token::Integer(1), Token::Integer(1),  Token::RParen, Token::Integer(1),  Token::RParen];
        result = parse(&mut input);
        let output2 = vec![Object::Symbol(String::from("+")), expected, Object::Integer(1)];
        let expected2 = Object::List(output2);
        assert!(equal_object_lists(&result, &expected2));
    }
}
