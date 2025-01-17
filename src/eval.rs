
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

fn apply_lambda(f: Sexpr, args: Sexpr, alist: Vec<String, Sexpr>) -> Sexpr {
    // args is Sexpr::List
    Sexpr::Nil
}

// fn bind_formals(formals: Sexpr, actuals: , alist: Vec<String, Sexpr>) {}


// fn eval_setq(var val)
// fn eval_cond(clauses alist)
// fn eval_defun(body alist)

fn apply_atom(f: Sexpr, args: Sexpr, alist: Vec<String, Sexpr>) {
}

*/

fn eval_atom(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Sexpr {
    match v {
        Sexpr::Symbol(s) => assoc(s, alist),
        _ => v,
    }
}



pub fn eval_command(v: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<String, String> {
    // alist will be treated like an activation record

    let result = match v {
        Sexpr::List(_) => Err(String::from("not implemented yet")),
        _ => Ok(eval_atom(v, alist)),
    }; 

    match result {
        Ok(o) => { 
            match o {
                Sexpr::Integer(i) => Ok(i.to_string()),
                Sexpr::Symbol(s) => Ok(s),
                Sexpr::T => Ok(String::from("T")),
                Sexpr::Nil => Ok(String::from("NIL")),
                _ => Err(String::from("should not get here")),
            }
        },
        Err(s) => Err(s),
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
