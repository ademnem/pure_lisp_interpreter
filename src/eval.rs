use crate::lisp::*;
use crate::parse::*;

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
    // must only be called on symbol
    match f {
        Sexpr::Symbol(s) => match s.as_str() {
            "CAR" => car(args, alist.clone()),
            // "CDR" => cdr(),
            // "SETQ" => setq(args, alist.clone()),
            _ => Ok(Sexpr::Nil), // calls apply_lambda later
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
pub fn evaluate(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<String, String> {
    // alist = activation record (or call stack)
    let result = match v {
        Sexpr::List(_) => apply(v, alist.clone()),
        Sexpr::Lambda(_, _) => apply(v, alist.clone()),
        _ => Ok(eval_atom(v, alist.clone())),
    };

    match result {
        Ok(o) => match o {
            Sexpr::Integer(i) => Ok(i.to_string()),
            Sexpr::String(s) => Ok(s),
            Sexpr::Symbol(s) => Ok(s),
            Sexpr::T => Ok(String::from("T")),
            Sexpr::Nil => Ok(String::from("NIL")),
            _ => Err(String::from("should not get here")),
        },
        Err(o) => Err(o),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn equal_object_lists(l1: Vec<Sexpr>, l2: Vec<Sexpr>) -> bool {
        false
    }

    fn equal_objects(o1: Sexpr, o2: Sexpr) -> bool {
        match (o1, o2) {
            (Sexpr::List(l1), Sexpr::List(l2)) => equal_object_lists(l1, l2),
            (a, b) => a == b,
        }
    }

    #[test]
    fn test_assoc() {
        let mut v: String = String::from("A");
        let mut alist: Vec<(String, Sexpr)> = Vec::new();
        let mut result: Sexpr = Sexpr::Nil;
        assert!(equal_objects(assoc(v, alist.clone()), result));

        v = String::from("A");
        alist.push((String::from("A"), Sexpr::Symbol(String::from("hello"))));
        result = Sexpr::Symbol(String::from("hello"));
        assert!(equal_objects(assoc(v, alist.clone()), result));

        // Local scope variable is used before anything else
        v = String::from("A");
        alist.push((String::from("A"), Sexpr::Integer(1)));
        result = Sexpr::Integer(1);
        assert!(equal_objects(assoc(v, alist.clone()), result));
    }

    #[test]
    fn test_eval_atom() {
        let mut v: Sexpr = Sexpr::Symbol(String::from("A"));
        let mut alist: Vec<(String, Sexpr)> = Vec::new();
        let mut result: Sexpr = Sexpr::Nil;
        assert!(equal_objects(eval_atom(v, alist.clone()), result));

        v = Sexpr::Symbol(String::from("A"));
        alist.push((String::from("A"), Sexpr::Symbol(String::from("hello"))));
        result = Sexpr::Symbol(String::from("hello"));
        assert!(equal_objects(eval_atom(v, alist.clone()), result));

        // Local scope variable is used before anything else
        v = Sexpr::Symbol(String::from("A"));
        alist.push((String::from("A"), Sexpr::Integer(1)));
        result = Sexpr::Integer(1);
        assert!(equal_objects(eval_atom(v, alist.clone()), result));
    }
}
