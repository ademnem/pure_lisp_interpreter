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
    // only (quote (list)) is passed into certain functions
    // and so args needs to be evaluated
    // "all args are recursively evaluated" -prof klefstad

    // only f

    match f {
        Sexpr::Symbol(s) => match s.as_str() {
            // when something that is quoted is passed, evaluating it
            // makes it itself
            // evaluate quoted args before sendings
            "QUOTE" => quote(args),
            _ => {
                let eval_args = match evaluate(args.clone(), alist.clone()) {
                    Ok(o) => o,
                    Err(e) => return Err(e),
                };
                match s.as_str() {
                    "CAR" => car(eval_args, alist.clone()),
                    // "CDR" => cdr(),
                    // "SETQ" => setq(args, alist.clone()),
                    _ => Ok(Sexpr::Nil), // calls apply_lambda later
                }
            }
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
        Sexpr::List(l) => Sexpr::List(l[1..].to_vec()),
        _ => return Err(String::from("apply: issue with args")),
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
    // alist = activation record (or call stack)
    match v {
        Sexpr::List(_) => apply(v, alist.clone()),
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
