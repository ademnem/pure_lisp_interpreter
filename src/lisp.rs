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
                return Err(String::from("setq: NIL is not a valid symbol name"));
            } else {
                OBLIST.lock().unwrap().push((s, value.clone()))
            }
        }
        _ => return Err(String::from("setq: first arg must be a symbol")),
    }

    Ok(value)
}

pub fn equal(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("equal: args must be a list")),
    };

    let left: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("equal: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let right: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("equal: no second arg")),
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

pub fn null(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("null: args list is empty")),
        },
        _ => return Err(String::from("null: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::Nil => Ok(Sexpr::T),
        _ => Ok(Sexpr::Nil),
    }
}

pub fn floor(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let mut arg = match &args {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("floor: args list is empty")),
        },
        _ => return Err(String::from("floor: args must be a list")),
    };
    arg = match evaluate(arg, alist.clone()) {
        Ok(o) => o,
        Err(e) => return Err(e),
    };

    match arg {
        Sexpr::Integer(i) => Ok(Sexpr::Integer(i)),
        Sexpr::Float(f) => Ok(Sexpr::Integer(f as i64)),
        _ => Err(String::from("floor: arg must be a int or float")),
    }
}

pub fn add(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("add: args must be a list")),
    };

    let num1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("add: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let num2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("add: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (num1, num2) {
        (Sexpr::Integer(i1), Sexpr::Integer(i2)) => Ok(Sexpr::Integer(i1 + i2)),
        (Sexpr::Float(f), Sexpr::Integer(i)) => Ok(Sexpr::Float(f + i as f64)),
        (Sexpr::Integer(i), Sexpr::Float(f)) => Ok(Sexpr::Float(f + i as f64)),
        (Sexpr::Float(f1), Sexpr::Float(f2)) => Ok(Sexpr::Float(f1 + f2)),
        (_, _) => Err(String::from("add: both args must be nums")),
    }
}

pub fn subtract(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("subtract: args must be a list")),
    };

    let num1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("subtract: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let num2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("subtract: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (num1, num2) {
        (Sexpr::Integer(i1), Sexpr::Integer(i2)) => Ok(Sexpr::Integer(i1 - i2)),
        (Sexpr::Float(f), Sexpr::Integer(i)) => Ok(Sexpr::Float(f - i as f64)),
        (Sexpr::Integer(i), Sexpr::Float(f)) => Ok(Sexpr::Float(f - i as f64)),
        (Sexpr::Float(f1), Sexpr::Float(f2)) => Ok(Sexpr::Float(f1 - f2)),
        (_, _) => Err(String::from("subtract: both args must be nums")),
    }
}

pub fn multiply(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("multiply: args must be a list")),
    };

    let num1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("multiply: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let num2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("multiply: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (num1, num2) {
        (Sexpr::Integer(i1), Sexpr::Integer(i2)) => Ok(Sexpr::Integer(i1 * i2)),
        (Sexpr::Float(f), Sexpr::Integer(i)) => Ok(Sexpr::Float(f * i as f64)),
        (Sexpr::Integer(i), Sexpr::Float(f)) => Ok(Sexpr::Float(f * i as f64)),
        (Sexpr::Float(f1), Sexpr::Float(f2)) => Ok(Sexpr::Float(f1 * f2)),
        (_, _) => Err(String::from("multiply: both args must be nums")),
    }
}

pub fn divide(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("divide: args must be a list")),
    };

    let num1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("divide: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let num2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("divide: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (num1, num2) {
        (Sexpr::Integer(i1), Sexpr::Integer(i2)) => Ok(Sexpr::Integer(i1 / i2)),
        (Sexpr::Float(f), Sexpr::Integer(i)) => Ok(Sexpr::Float(f / i as f64)),
        (Sexpr::Integer(i), Sexpr::Float(f)) => Ok(Sexpr::Float(f / i as f64)),
        (Sexpr::Float(f1), Sexpr::Float(f2)) => Ok(Sexpr::Float(f1 / f2)),
        (_, _) => Err(String::from("divide: both args must be nums")),
    }
}

pub fn modulo(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("modulo: args must be a list")),
    };

    let num1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("modulo: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let num2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("modulo: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (num1, num2) {
        (Sexpr::Integer(i1), Sexpr::Integer(i2)) => Ok(Sexpr::Integer(i1 % i2)),
        (Sexpr::Float(f), Sexpr::Integer(i)) => Ok(Sexpr::Float(f % i as f64)),
        (Sexpr::Integer(i), Sexpr::Float(f)) => Ok(Sexpr::Float(f % i as f64)),
        (Sexpr::Float(f1), Sexpr::Float(f2)) => Ok(Sexpr::Float(f1 % f2)),
        (_, _) => Err(String::from("modulo: both args must be nums")),
    }
}

pub fn print(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("print: args must be a list")),
    };

    let arg: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("print: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    print!("{}", sexpr_to_string(&arg));
    Ok(Sexpr::String(String::new()))
}

pub fn eval(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("eval: args must be a list")),
    };

    let arg: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("eval: no arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    evaluate(arg, alist.clone())
}

pub fn cons(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    let args: Vec<Sexpr> = match &args {
        Sexpr::List(l) => l.clone(),
        _ => return Err(String::from("cons: args must be a list")),
    };

    let arg1: Sexpr = match evaluate(
        match args.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("cons: no first arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let arg2: Sexpr = match evaluate(
        match args.get(1) {
            Some(s) => s.clone(),
            None => return Err(String::from("cons: no second arg")),
        },
        alist.clone(),
    ) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match (arg1, arg2) {
        (l, Sexpr::List(mut r)) => {
            r.insert(0, l);
            Ok(Sexpr::List(r))
        }
        (l, r) => Ok(Sexpr::List(vec![l, r])),
    }
}

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
    fn test_equal() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(equal(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Integer(1)));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(equal(args, alist), Ok(Sexpr::T));
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

    #[test]
    fn test_null() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(null(args, alist), Ok(Sexpr::Nil));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Nil]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(null(args, alist), Ok(Sexpr::T));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Nil]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Nil));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(null(args, alist), Ok(Sexpr::T));
    }

    #[test]
    fn test_floor() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(floor(args, alist), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(1.1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(floor(args, alist), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X")), Sexpr::Float(6.7)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(setq(args, alist.clone()), Ok(Sexpr::Float(6.7)));
        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]);
        let alist = OBLIST.lock().unwrap().clone();
        assert_eq!(floor(args, alist), Ok(Sexpr::Integer(6)));
    }

    #[test]
    fn test_add() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(add(args, alist), Ok(Sexpr::Integer(2)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(1.1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(add(args, alist), Ok(Sexpr::Float(2.1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(1.1), Sexpr::Float(1.1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(add(args, alist), Ok(Sexpr::Float(2.2)));
    }

    #[test]
    fn test_subtract() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(subtract(args, alist), Ok(Sexpr::Integer(0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(100.0), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(subtract(args, alist), Ok(Sexpr::Float(99.0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(1.1), Sexpr::Float(1.1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(subtract(args, alist), Ok(Sexpr::Float(0.0)));
    }

    #[test]
    fn test_multiply() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(multiply(args, alist), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(100.0), Sexpr::Integer(2)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(multiply(args, alist), Ok(Sexpr::Float(200.0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(10.0), Sexpr::Float(1.1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(multiply(args, alist), Ok(Sexpr::Float(11.0)));
    }

    #[test]
    fn test_divide() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(divide(args, alist), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(100.0), Sexpr::Integer(2)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(divide(args, alist), Ok(Sexpr::Float(50.0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(10.0), Sexpr::Float(2.0)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(divide(args, alist), Ok(Sexpr::Float(5.0)));
    }

    #[test]
    fn test_modulo() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(modulo(args, alist), Ok(Sexpr::Integer(0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(51.0), Sexpr::Integer(50)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(modulo(args, alist), Ok(Sexpr::Float(1.0)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Float(10.0), Sexpr::Float(2.0)]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(modulo(args, alist), Ok(Sexpr::Float(0.0)));
    }

    #[test]
    fn test_eval() {
        let args: Sexpr = Sexpr::List(vec![Sexpr::Integer(1)]);
        let mut alist: Vec<(String, Sexpr)> = Vec::new();
        assert_eq!(eval(args, alist.clone()), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Nil]);
        assert_eq!(eval(args, alist.clone()), Ok(Sexpr::Nil));

        let args: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("X"))]);
        alist.push((String::from("X"), Sexpr::Integer(1)));
        assert_eq!(eval(args, alist.clone()), Ok(Sexpr::Integer(1)));

        let args: Sexpr = Sexpr::List(vec![Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::Symbol(String::from("X")),
        ])]);
        assert_eq!(eval(args, alist.clone()), Ok(Sexpr::Integer(1)));
    }

    #[test]
    fn test_cons() {
        let mut arg1: Sexpr = Sexpr::Integer(1);
        let mut arg2: Sexpr = Sexpr::Integer(1);
        let mut args: Sexpr = Sexpr::List(vec![arg1, arg2]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        let mut result: Sexpr = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1)]);
        assert_eq!(cons(args, alist.clone()), Ok(result));

        arg1 = Sexpr::Integer(1);
        arg2 = Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Nil]),
        ]);
        args = Sexpr::List(vec![arg1, arg2]);
        result = Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Integer(1), Sexpr::Nil]);
        assert_eq!(cons(args, alist.clone()), Ok(result));

        arg1 = Sexpr::List(vec![
            Sexpr::Symbol(String::from("QUOTE")),
            Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Nil]),
        ]);
        arg2 = Sexpr::Integer(1);
        args = Sexpr::List(vec![arg1, arg2]);
        result = Sexpr::List(vec![
            Sexpr::List(vec![Sexpr::Integer(1), Sexpr::Nil]),
            Sexpr::Integer(1),
        ]);
        assert_eq!(cons(args, alist.clone()), Ok(result));
    }
}
