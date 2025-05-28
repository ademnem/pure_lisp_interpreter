use crate::eval::*;
use crate::parse::*;

pub static OBLIST: Vec<(String, Sexpr)> = Vec::new();

pub fn quote(args: Sexpr) -> Result<Sexpr, String> {
    Ok(args)
}
pub fn car(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    match args {
        Sexpr::List(l) => match l.first() {
            Some(s) => evaluate(s.clone(), alist.clone()),
            None => Err(String::from("car: list len must be >=1")),
        },
        _ => Err(String::from("car: arg must be list")),
    }
}
pub fn cdr(args: Sexpr, alist: Vec<(String, Sexpr)>) -> Result<Sexpr, String> {
    /*
        match args {
            Sexpr::List(l) => Ok(evaluate(l[1..].to_vec())),
            _ => Err(String::from("car: arg must be list")),
        }
    */
    Ok(Sexpr::Nil)
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
        let mut args: Sexpr = Sexpr::List(vec![Sexpr::String(String::from("a"))]);
        assert!(equal_sexprs(&quote(args.clone()).unwrap(), &args));

        // adding to the alist should have no effect
        args = Sexpr::List(vec![Sexpr::Symbol(String::from("a"))]);
        assert!(equal_sexprs(&quote(args.clone()).unwrap(), &args));

        args = Sexpr::List(Vec::new());
        assert!(equal_sexprs(&quote(args.clone()).unwrap(), &args));
    }

    #[test]
    fn test_car() {
        let mut args: Sexpr = Sexpr::List(vec![Sexpr::String(String::from("a"))]);
        let mut alist: Vec<(String, Sexpr)> = vec![];
        assert_eq!(
            car(args, alist.clone()).unwrap(),
            Sexpr::String(String::from("a"))
        );

        args = Sexpr::List(vec![Sexpr::Symbol(String::from("a"))]);
        alist.push((String::from("a"), Sexpr::Integer(51)));
        assert_eq!(car(args, alist.clone()).unwrap(), Sexpr::Integer(51));

        args = Sexpr::List(Vec::new());
        assert_eq!(
            car(args, alist.clone()),
            Err(String::from("car: list len must be >=1"))
        );

        args = Sexpr::Integer(51);
        assert_eq!(
            car(args, alist.clone()),
            Err(String::from("car: arg must be list")),
        );
    }
}
