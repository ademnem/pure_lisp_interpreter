use crate::lexer::*;

// clone makes a deep copy of Sexpr
#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr {
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<Sexpr>),
    Lambda(String, Vec<Sexpr>),
    T,
    Nil,
}
pub fn sexpr_to_string(v: &Sexpr) -> String {
    match v {
        Sexpr::Integer(i) => i.to_string(),
        Sexpr::Float(f) => f.to_string(),
        Sexpr::String(s) => String::from(s),
        Sexpr::Symbol(s) => String::from(s),
        Sexpr::T => String::from("T"),
        Sexpr::Nil => String::from("NIL"),
        Sexpr::List(l) => {
            let mut str: String = String::from("(");

            if l.len() < 2 {
                for sexpr in &l[0..l.len() - 1] {
                    str += sexpr_to_string(sexpr).as_str();
                    str += " ";
                }
                let last = &l[l.len() - 1];
                if last != &Sexpr::Nil {
                    str += " . ";
                    str += sexpr_to_string(last).as_str();
                }
                str += ")";
            } else {
                // you can't have a list with only a non nil
                for sexpr in &l[0..l.len() - 2] {
                    str += sexpr_to_string(sexpr).as_str();
                    str += " ";
                }
                str += sexpr_to_string(&l[l.len() - 2]).as_str();
                let last = &l[l.len() - 1];
                if last != &Sexpr::Nil {
                    str += " . ";
                    str += sexpr_to_string(last).as_str();
                }
                str += ")";
            }
            str
        }
        Sexpr::Lambda(name, body) => {
            // not solidified yet
            let mut str: String = String::from("(");
            str += name;
            str += " ";
            for sexpr in &body[1..body.len() - 1] {
                str += sexpr_to_string(sexpr).as_str();
                str += " ";
            }
            str += sexpr_to_string(&body[body.len() - 1]).as_str();
            str += ")";
            str
        }
    }
}

fn parse_atom(token: &Token) -> Sexpr {
    match token {
        Token::Integer(i) => Sexpr::Integer(*i),
        Token::Float(f) => Sexpr::Float(*f),
        Token::String(s) => Sexpr::String(s.clone()),
        Token::Symbol(s) => match s.as_str() {
            "T" => Sexpr::T,
            "NIL" => Sexpr::Nil,
            _ => Sexpr::Symbol(s.to_string()),
        },
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
    let _ = tokens.pop(); // remove first LParen

    let mut list: Vec<Sexpr> = Vec::new();
    /*
    assumptions
    - parens are balanced
    - it is bound to be a symbol or an integer at this point
    */

    while !tokens.is_empty() {
        let token = tokens.pop();
        if let Some(token1) = token {
            match token1.clone() {
                Token::RParen => {
                    return Sexpr::List(list);
                }
                Token::LParen => {
                    tokens.push(Token::LParen);
                    list.push(parse_list(tokens));
                }
                t => {
                    list.push(parse_atom(&t));
                }
            }
        }
    }
    Sexpr::List(list)
}

// due to the shell assume all inputs have balanced parens or only one input
pub fn parse(tokens: &mut Vec<Token>) -> Sexpr {
    if is_list(tokens) {
        tokens.reverse();
        return parse_list(tokens);
    }
    parse_atom(tokens.first().unwrap())
}

// eval_defun

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use std::ptr;

    #[test]
    fn test_sexpr_to_string() {
        let input: Sexpr = Sexpr::List(vec![Sexpr::Nil]);
        let expected: String = String::from("()");
        assert_eq!(sexpr_to_string(&input), expected);

        let input: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1), Sexpr::Nil]);
        let expected: String = String::from("(1 1)");
        assert_eq!(sexpr_to_string(&input), expected);

        let input: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let expected: String = String::from("(1 . 1)");
        assert_eq!(sexpr_to_string(&input), expected);
    }

    #[test]
    fn test_parse_atom() {
        let mut input: Token = Token::Integer(51);
        let mut expected: Sexpr = Sexpr::Integer(51);
        assert_eq!(parse_atom(&input), expected);

        input = Token::Symbol(String::from("+"));
        expected = Sexpr::Symbol(String::from("+"));
        assert_eq!(parse_atom(&input), expected);

        input = Token::Symbol(String::from("T"));
        expected = Sexpr::T;
        assert_eq!(parse_atom(&input), expected);

        input = Token::Float(1.1);
        expected = Sexpr::Float(1.1);
        assert_eq!(parse_atom(&input), expected);
    }

    #[test]
    fn test_is_list() {
        let mut input: Vec<Token> = vec![Token::LParen, Token::RParen];
        assert!(is_list(&input));

        input = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Integer(1),
            Token::Integer(1),
            Token::RParen,
        ];
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

    #[test]
    fn test_parse_list() {
        let mut input: Vec<Token> = vec![Token::LParen, Token::RParen];
        input.reverse();
        let mut result: Sexpr = parse_list(&mut input);
        let mut output: Vec<Sexpr> = Vec::new();
        let mut expected: Sexpr = Sexpr::List(output);
        assert!(equal_sexprs(&result, &expected));

        input = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Integer(1),
            Token::Integer(1),
            Token::RParen,
        ];
        input.reverse();
        result = parse_list(&mut input);
        output = vec![
            Sexpr::Symbol(String::from("+")),
            Sexpr::Integer(1),
            Sexpr::Integer(1),
        ];
        expected = Sexpr::List(output);
        assert!(equal_sexprs(&result, &expected));

        input = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Integer(1),
            Token::Integer(1),
            Token::RParen,
            Token::Integer(1),
            Token::RParen,
        ];
        input.reverse();
        result = parse_list(&mut input);
        let output2 = vec![
            Sexpr::Symbol(String::from("+")),
            expected,
            Sexpr::Integer(1),
        ];
        let expected2 = Sexpr::List(output2);
        assert!(equal_sexprs(&result, &expected2));
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
        assert!(equal_sexprs(&result, &expected));

        input = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Integer(1),
            Token::Integer(1),
            Token::RParen,
        ];
        result = parse(&mut input);
        output = vec![
            Sexpr::Symbol(String::from("+")),
            Sexpr::Integer(1),
            Sexpr::Integer(1),
        ];
        expected = Sexpr::List(output);
        assert!(equal_sexprs(&result, &expected));

        input = vec![
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::LParen,
            Token::Symbol(String::from("+")),
            Token::Integer(1),
            Token::Integer(1),
            Token::RParen,
            Token::Integer(1),
            Token::RParen,
        ];
        result = parse(&mut input);
        let output2 = vec![
            Sexpr::Symbol(String::from("+")),
            expected,
            Sexpr::Integer(1),
        ];
        let expected2 = Sexpr::List(output2);
        assert!(equal_sexprs(&result, &expected2));
    }

    #[test]
    fn test_sexpr_clone() {
        let mut original = Sexpr::Integer(10);
        let mut clone = original.clone();
        assert_eq!(original, clone);
        assert!(!ptr::eq(&original, &clone));
        assert!(match (original, clone) {
            (Sexpr::Integer(l), Sexpr::Integer(r)) => l == r,
            _ => false,
        });

        original = Sexpr::List(vec![
            Sexpr::Symbol(String::from("+")),
            Sexpr::Integer(1),
            Sexpr::Integer(1),
        ]);
        clone = original.clone();
        assert!(equal_sexprs(&original, &original));
        assert!(!ptr::eq(&original, &clone));
        assert!(!match (&original, &clone) {
            (Sexpr::List(l), Sexpr::List(r)) => ptr::eq(&l, &r),
            _ => false,
        });
        let vec_original = unpack_list(&original).unwrap();
        let vec_clone = unpack_list(&clone).unwrap();
        let comp = vec_original.iter().zip(vec_clone);
        for (o, c) in comp {
            assert_eq!(o, c);
            assert!(!ptr::eq(&o, &c));
        }
    }
}
