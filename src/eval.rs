use crate::lisp::*;
use crate::parse::*;
use crate::test::*;

fn assoc(v: String, alist: Vec<(String, Sexpr)>) -> Sexpr {
    for (s, o) in alist.iter().rev() {
        if *s == v {
            return o.clone();
        }
    }

    Sexpr::Nil
}

/*
fn eval_list(l: Sexpr, alist: Vec<String, Sexpr>) -> Sexpr {
    Sexpr::Nil
}

// fn bind_formals(formals: Sexpr, actuals: Sexpr, alist: Vec<String, Sexpr>) {}
// pass a mutable reference of the formal and replace it with the actual
*/
fn apply_lambda(f: Sexpr, args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    // not sure how to dynamically create and store a lambda in rust
    // maybe i just store the list for the function and evaluate it when the function is called
    // yeah i'll do that, later tho...
    Ok(Sexpr::Nil)
}
fn apply_atom(f: Sexpr, args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    // args is a list containing the args of the function
    // (arg1 arg2 ... argN)
    match f {
        Sexpr::Symbol(s) => match s.as_str() {
            "QUOTE" => quote(args),
            "CAR" => car(args, alist.clone()),
            "CDR" => cdr(args, alist.clone()),
            "SETQ" => setq(args, alist.clone()),
            "EQUAL" => equal(args, alist.clone()),
            "ATOM" => atom(args, alist.clone()),
            "LISTP" => listp(args, alist.clone()),
            "NULL" => null(args, alist.clone()),
            "FLOOR" => floor(args, alist.clone()),
            "+" => add(args, alist.clone()),
            "-" => subtract(args, alist.clone()),
            "*" => multiply(args, alist.clone()),
            "/" => divide(args, alist.clone()),
            "MOD" => modulo(args, alist.clone()),
            "PRINT" => print(args, alist.clone()),
            "EVAL" => eval(args, alist.clone()),
            "CONS" => cons(args, alist.clone()),
            _ => Ok(Sexpr::Nil),
        },
        _ => Err(String::from("apply_atom: requires symbol as first arg")),
    }
}

fn apply(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    // create a copy of first sexpr
    let f = match &v {
        Sexpr::List(l) => match l.first() {
            Some(s) => s.clone(),
            None => return Err(String::from("apply: list is empty")),
        },
        _ => return Err(String::from("apply: val passed in was not a list")),
    };

    // create copy of args
    let args = match &v {
        Sexpr::List(l) => {
            if l.len() >= 2 {
                Sexpr::List(l[1..].to_vec())
            } else {
                return Err(String::from("apply: v len must be >= 2"));
            }
        }
        _ => return Err(String::from("apply: v must be a list")),
    };

    match f {
        Sexpr::Symbol(_) => apply_atom(f, args, alist.clone()), // do i need to clone the alist here? it's safe to do so...
        Sexpr::Lambda(_, _) => apply_lambda(f, args, alist.clone()),
        _ => Ok(Sexpr::Nil),
    }
}

// idk what this is supposed to do really
fn eval_list(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Sexpr {
    // calls apply
    Sexpr::Nil
}
fn eval_atom(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Sexpr {
    match v {
        Sexpr::Symbol(s) => assoc(s, alist),
        _ => v, // integers, T and NIL return themselves
    }
}
pub fn evaluate(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    match &v {
        Sexpr::List(l) => {
            if l.is_empty() {
                Ok(Sexpr::Nil)
            } else if l.first() == Some(&Sexpr::Nil) {
                Ok(Sexpr::Nil)
            } else {
                apply(v, alist.clone())
            }
        }
        Sexpr::Lambda(_, _) => apply(v, alist.clone()),
        _ => Ok(eval_atom(v, alist.clone())),
    }
}

#[cfg(test)]
mod tests {
    use crate::test;

    use super::*;
    use test::*;

    #[test]
    fn test_assoc() {
        let mut v: String = String::from("A");
        let mut alist: Vec<(String, Sexpr)> = Vec::new();
        let mut result: Sexpr = Sexpr::Nil;
        assert!(equal_sexprs(&assoc(v, alist.clone()), &result));

        v = String::from("A");
        alist.push((String::from("A"), Sexpr::Symbol(String::from("hello"))));
        result = Sexpr::Symbol(String::from("hello"));
        assert!(equal_sexprs(&assoc(v, alist.clone()), &result));

        // Local scope variable is used before anything else
        v = String::from("A");
        alist.push((String::from("A"), Sexpr::Integer(1)));
        result = Sexpr::Integer(1);
        assert!(equal_sexprs(&assoc(v, alist.clone()), &result));
    }

    #[test]
    fn test_eval_atom() {
        let mut v: Sexpr = Sexpr::Symbol(String::from("A"));
        let mut alist: Vec<(String, Sexpr)> = Vec::new();
        let mut result: Sexpr = Sexpr::Nil;
        assert!(equal_sexprs(&eval_atom(v, alist.clone()), &result));

        v = Sexpr::Symbol(String::from("A"));
        alist.push((String::from("A"), Sexpr::Symbol(String::from("hello"))));
        result = Sexpr::Symbol(String::from("hello"));
        assert!(equal_sexprs(&eval_atom(v, alist.clone()), &result));

        // Local scope variable is used before anything else
        v = Sexpr::Symbol(String::from("A"));
        alist.push((String::from("A"), Sexpr::Integer(1)));
        result = Sexpr::Integer(1);
        assert!(equal_sexprs(&eval_atom(v, alist.clone()), &result));
    }

    #[test]
    fn test_evaluate() {
        let args: Sexpr = Sexpr::List(vec![
            Sexpr::Integer(1),
            Sexpr::Integer(2),
            Sexpr::Integer(3),
        ]);
        let v: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("QUOTE")), args.clone()]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert!(equal_sexprs(
            &evaluate(v.clone(), alist.clone()).unwrap(),
            &args
        ));

        // () == NIL tests
        let v: Sexpr = Sexpr::List(Vec::new());
        assert!(equal_sexprs(
            &evaluate(v.clone(), alist.clone()).unwrap(),
            &Sexpr::Nil
        ));
        let v: Sexpr = Sexpr::List(vec![Sexpr::Nil]);
        assert!(equal_sexprs(
            &evaluate(v.clone(), alist.clone()).unwrap(),
            &Sexpr::Nil
        ));
    }

    #[test]
    fn test_apply() {
        let args: Sexpr = Sexpr::List(vec![
            Sexpr::Integer(1),
            Sexpr::Integer(2),
            Sexpr::Integer(3),
        ]);
        let v: Sexpr = Sexpr::List(vec![Sexpr::Symbol(String::from("QUOTE")), args.clone()]);
        let alist: Vec<(String, Sexpr)> = Vec::new();
        assert!(equal_sexprs(
            &apply(v.clone(), alist.clone()).unwrap(),
            &args
        ));
    }

    #[test]
    fn test_apply_atom() {
        let mut f = Sexpr::Symbol(String::from("QUOTE"));
        let mut quoted = Sexpr::List(vec![
            Sexpr::Integer(1),
            Sexpr::Integer(2),
            Sexpr::Integer(3),
        ]);
        let mut args = Sexpr::List(vec![quoted.clone()]);
        let alist = Vec::new();
        assert!(equal_sexprs(
            &apply_atom(f.clone(), args.clone(), alist.clone()).unwrap(),
            &quoted
        ));

        /*
        f = Sexpr::Symbol(String::from("CAR"));
        let quoted = Sexpr::List(vec![
            Sexpr::Integer(1),
            Sexpr::Integer(2),
            Sexpr::Integer(3),
        ]);
        args = Sexpr::List(vec![Sexpr::Symbol(String::from("QUOTE")), quoted.clone()]);
        match apply_atom(f.clone(), args.clone(), alist.clone()) {
            Ok(s) => match s {
                Sexpr::Integer(i) => println!("{}", i),
                _ => println!("no"),
            },
            Err(e) => println!("{}", e),
        }
        assert!(equal_sexprs(
            &apply_atom(f.clone(), args.clone(), alist.clone()).unwrap(),
            &Sexpr::Integer(1)
        ));
        */
    }
}
