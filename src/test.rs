use crate::parse::*;

pub fn equal_sexpr_lists(l: &Vec<Sexpr>, r: &Vec<Sexpr>) -> bool {
    let comp = l.iter().zip(r);
    for (a, b) in comp {
        if !equal_sexprs(a, b) {
            return false;
        }
    }
    true
}

pub fn equal_sexprs(l: &Sexpr, r: &Sexpr) -> bool {
    match (l, r) {
        (Sexpr::List(a), Sexpr::List(b)) => equal_sexpr_lists(&a, &b),
        (a, b) => a == b,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal_sexprs_atom() {
        let mut l = Sexpr::Integer(55);
        let mut r = Sexpr::Integer(55);
        assert!(equal_sexprs(&l, &r));

        l = Sexpr::String(String::from("Hello"));
        assert!(!equal_sexprs(&l, &r));

        r = Sexpr::String(String::from("Hello"));
        assert!(equal_sexprs(&l, &r));
    }

    #[test]
    fn test_equal_sexpr_lists() {
        let mut l = Sexpr::List(Vec::new());
        let mut r = Sexpr::List(Vec::new());
        assert!(equal_sexprs(&l, &r));

        l = Sexpr::List(vec![Sexpr::Integer(51)]);
        r = Sexpr::List(vec![Sexpr::Integer(51)]);
        assert!(equal_sexprs(&l, &r));

        l = Sexpr::List(vec![Sexpr::List(vec![Sexpr::Integer(51)])]);
        r = Sexpr::List(vec![Sexpr::List(vec![Sexpr::Integer(51)])]);
        assert!(equal_sexprs(&l, &r));

        r = Sexpr::List(vec![Sexpr::List(vec![Sexpr::Integer(1)])]);
        assert!(!equal_sexprs(&l, &r));
    }
}
