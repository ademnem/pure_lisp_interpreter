use crate::eval::*;
use crate::parse::*;
use crate::test::*;

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static OBLIST: Lazy<Mutex<Vec<(String, Sexpr)>>> = Lazy::new(|| Mutex::new(Vec::new()));

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
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("setq: args must be a list")),
    };

    let symbol: Sexpr = match args.first() {
        Some(s) => s.clone(),
        None => return Err(String::from("setq: no first arg")),
    };
    let value: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("setq: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match symbol {
        Sexpr::Symbol(s) => {
            if s == String::from("NIL") {
                return Err(String::from("NIL is not a valid symbol name"));
            } else {
                OBLIST.lock().unwrap().push((s, value.clone()))
            }
        }
        _ => return Err(String::from("first arg must be a symbol")),
    }

    Ok(value)
}

pub fn eq(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("eq: args must be a list")),
    };

    let left: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("eq: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let right: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("eq: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    if equal_sexprs(&left, &right) {
        Ok(Sexpr::T)
    } else {
        Ok(Sexpr::Nil)
    }
}

pub fn atom(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("atom: args list is empty")),
        },
        _ => return Err(String::from("atom: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::List(_) => Ok(Sexpr::Nil),
        _ => Ok(Sexpr::T),
    }
}

pub fn listp(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("listp: args list is empty")),
        },
        _ => return Err(String::from("listp: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::List(_) => Ok(Sexpr::T),
        Sexpr::Nil => Ok(Sexpr::T),
        _ => Ok(Sexpr::Nil),
    }
}
// fn eval_cond(clauses alist)
// fn eval_defun(body alist)

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_setq() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Integer(1)));
        let v: Sexpr = Sexpr::Symbol(String::from("X"));
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(evaluate(v, alist.clone()), Ok(Sexpr::Integer(1)));
        // test for OBLIST change through cargo run
        // i can put this in the test_evaluate function in eval.rs later if needed
    }

    #[test]
    fn test_eq() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(eq(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Integer(1)));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(eq(args, alist), Ok(Sexpr::T));
    }

    #[test]
    fn test_atom() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(atom(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Integer(1)));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(atom(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]),
        ])]);
        let alist = Vec::new();
        assert_eq!(atom(args, alist), Ok(Sexpr::Nil));
    }

    #[test]
    fn test_listp() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(listp(args, alist), Ok(Sexpr::Nil));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Integer(1)));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(listp(args, alist), Ok(Sexpr::Nil));

        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]),
        ])]);
        let alist = Vec::new();
        assert_eq!(listp(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Nil]);
        let alist = Vec::new();
        assert_eq!(listp(args, alist), Ok(Sexpr::T));
    }
}
