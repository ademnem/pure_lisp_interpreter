use crate::eval::*;
use crate::parse::*;

pub static OBLIST: Vec<(String, Sexpr)> = Vec::new();

pub fn quote(args: Sexpr) -> Result<Sexpr, String> {
    match args {
        // just return the first argument as is
        Sexpr::List(l) => match l.first() {
            Some(s) => Ok(s.clone()),
            None => Err(String::from("quote: list is empty")),
        },
        _ => Err(String::from("quote: something went wrong")), // lambda should also return itself right?
    }
}
pub fn car(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("car: args list is empty")),
        },
        _ => return Err(String::from("car: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::List(l) => match l.first() {
            Some(s) => Ok(s.clone()),
            None => Err(String::from("car: list len must be >=1")),
        },
        _ => Err(String::from("car: arg must be list")),
    }
}
pub fn cdr(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("cdr: args list is empty")),
        },
        _ => return Err(String::from("cdr: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::List(l) => {
            if l.len() >= 1 {
                Ok(Sexpr::List(l[1..].to_vec()))
            } else {
                return Err(String::from("cdr: arg must be length >= 1"));
            }
        }
        _ => Err(String::from("cdr: arg must be list")),
    }
}
pub fn setq(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let something = match args {
        // needs at least two args
        // only get the first two args
        _ => return Err(String::from("setq requires 2 args")),
    };
    // var has to be a Symbol
    // val has to be the evaluated value of
    Ok(Sexpr::Nil)
}

// fn eval_cond(clauses alist)
// fn eval_defun(body alist)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn test_quote() {
        let mut arg: Sexpr = Sexpr::String(String::from("a"));
        let mut args: Sexpr = Sexpr::List(vec![arg.clone()]);
        assert!(equal_sexprs(&quote(args.clone()).unwrap(), &arg));

        arg = Sexpr::List(Vec::new());
        args = Sexpr::List(vec![arg.clone()]);
        assert!(equal_sexprs(&quote(args.clone()).unwrap(), &arg));
    }

    #[test]
    fn test_car() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Integer(1)]),
        ])]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(car(args, alist.clone()), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]),
        ])]);
        assert_eq!(
            car(args, alist.clone()),
            Ok(Sexpr::Symbol(String::from("X")))
        );
    }

    #[test]
    fn test_cdr() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Integer(1)]),
        ])]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(cdr(args, alist.clone()), Ok(Sexpr::List(Vec::new())));

        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]),
        ])]);
        assert_eq!(
            cdr(args, alist.clone()),
            Ok(Sexpr::List(vec![Sexpr::Integer(1)]))
        );
    }
}
