

use crate::lexer::*;


#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
    Void,
}



fn parse_atom(token: Option<&Token>) -> Object {   
    match token.unwrap() {
        Token::Integer(i) => Object::Integer(*i),
        Token::Symbol(s) => Object::Symbol(s.to_string()),
        _ => Object::Void,
    }
}
fn is_list(tokens: &Vec<Token>) -> bool { 
    match tokens.first().unwrap() {
        Token::RParen => true,
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
                Token::RParen => { parse_list(tokens); },
                Token::LParen => { return Object::List(list); },   
            }
        }
    }
    Object::List(list)
}


// due to the shell assume all inputs have balanced parens or only one input
fn parse(tokens: &mut Vec<Token>) -> Object  {

    if is_list(tokens) {
        return parse_list(tokens);
    }
    parse_atom(tokens.first())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let mut input: Token = Token::Integer(51);
        let mut result: Object = Object::Integer(51); 
        assert_eq!(parse_atom(Some(&input)), result);

        input = Token::Symbol(String::from("+"));
        result = Object::Symbol(String::from("+"));
        assert_eq!(parse_atom(Some(&input)), result);
    }

    #[test]
    fn test_is_list() {
        let mut input: Vec<Token> = vec![Token::RParen, Token::LParen]; 
        assert!(is_list(&input));

        input = vec![Token::Symbol(String::from("+"))];
        assert!(!is_list(&input));
    }
}
