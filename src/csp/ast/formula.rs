/**************************************
- Author: Clement Poncelet
- Desc: Formula, root of intentional constraint equation.
    - atom, not, and, or
***************************************/

/**************************************
            Formula
***************************************/
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use crate::csp::ast::eval::Eval;
use crate::csp::ast::pred::{pred_scope, print_predicate, Pred};
use crate::csp::domain::domain::OrdT;
use crate::csp::truth::Truth;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::VValue;

#[derive(Clone, Debug)]
pub enum Formula<E:Eval> {
    Atom(Pred<E>),
    Not(Box<Formula<E>>),
    And(Vec<Formula<E>>),
    Or(Vec<Formula<E>>)
}

impl<E:Eval> Formula<E> {
    pub fn atom(p: Pred<E>) -> Self { Formula::Atom(p) }
    pub fn not(f: Formula<E>) -> Self { Formula::Not(Box::new(f)) }
    pub fn and(fs: Vec<Formula<E>>) -> Self { Formula::And(fs) }
    pub fn or(fs: Vec<Formula<E>>) -> Self { Formula::Or(fs) }
}

impl<E:Eval> Display for Formula<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_formula(self))
    }
}

pub fn eval_formula<E:Eval<Output = T>, T:OrdT>(f: &Formula<E>, asn: &Vec<VValue<T>>) -> Truth {
    match f {
        Formula::Atom(p)           => p.eval(asn),
        Formula::Not(f)     => !eval_formula(f, asn),
        Formula::Or(fs)
        => fs.iter().map(|x| eval_formula(x, asn)).reduce(Truth::or).unwrap(),
        Formula::And(fs)
        => fs.iter().map(|x| eval_formula(x, asn)).reduce(Truth::and).unwrap(),
    }
}

pub fn print_formula<E:Eval>(f: &Formula<E>)-> String {
    match f {
        Formula::Atom(p)           => print_predicate(p),
        Formula::Not(f)     => "! ".to_owned() + &*print_formula(f),
        Formula::Or(fs)
        => fs.iter().map(|op| "|| ".to_owned() + &*print_formula(op)).collect(),
        Formula::And(fs)
        => fs.iter().map(|op| "&& ".to_owned() + &*print_formula(op)).collect(),
    }
}

/**************************************
            Utilities
***************************************/

pub fn formula_scope<E: Eval<Output = T>, T: OrdT>(f: &Formula<E>, acc: &mut HashSet<Rc<ExVar<T>>>) {
    match f {
        Formula::Atom(p)           => pred_scope(p, acc),
        Formula::Not(f)     => formula_scope(f, acc),
        Formula::Or(fs)
        => fs.iter().map(|op| formula_scope(op, acc)).collect(),
        Formula::And(fs)
        => fs.iter().map(|op| formula_scope(op, acc)).collect(),
    }
}

/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::ast::expr::{Expr,AExpr};
    use crate::csp::ast::pred::Pred;
    use std::rc::Rc;
    use crate::csp::ast::formula::{eval_formula, Formula};
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::prelude::extvar::ExVar;
    use crate::{add, atom, base, cst, dom, eq, or, var, var_dom};
    use crate::csp::truth::Truth;
    use crate::csp::variable::vvalue::vv;

    fn setup_vars() -> (Rc<ExVar<i32>>, Rc<ExVar<i32>>) {
        let dom = SetDom::new(vec![1, 2, 3, 4]);
        let w = Rc::new(ExVar::new("w".into(), dom.clone()));
        let z = Rc::new(ExVar::new("z".into(), dom));
        (w, z)
    }

    #[test]
    fn formula_building() {
        let (w, z) = setup_vars();
        //inconvenience should be AExpr everywhere
        let f = or!(
                atom!(eq!(base!(var!(w)), base!(cst!(3)))),
                atom!(eq!(base!(var!(w)), add!(base!(var!(z)), base!(cst!(1)))))
            );
        match f {
            Formula::Or(fs) => assert_eq!(fs.len(), 2),
            _ => panic!("Expected Or formula"),
        }
    }

    #[test]
    fn formula_eval() {
        let (w, z) = setup_vars();
        //(w == z + 1) or (w == 3)
        let f = or!(
                atom!(eq!(base!(var!(w)), add!(base!(var!(z)), base!(cst!(1))))),
                atom!(eq!(base!(var!(w)), base!(cst!(3))))
            );

        // w = 2, z = 1 → true (2 == 1 + 1)
        let a1 = vec![vv(String::from("w"), 2), vv(String::from("z"), 1)];
        assert_eq!(eval_formula(&f, &a1), Truth::True);

        // w = 3, z = 1 → true (w == 3)
        let a2 = vec![vv(String::from("w"), 3), vv(String::from("z"), 1)];
        assert_eq!(eval_formula(&f, &a2), Truth::True);

        // w = 1, z = 1 → false
        let a3 = vec![vv(String::from("w"), 1), vv(String::from("z"), 1)];
        assert_eq!(eval_formula(&f, &a3), Truth::False);
    }

    #[test]
    fn formula_macro() {
        let dom = dom![1, 2, 3, 4];
        let w = var_dom!(String::from("w"), dom.snapshot());
        let z = var_dom!(String::from("z"), dom);
        let f = or!(
            atom!(eq!(
                base!(var!(w)),
                add!(base!(var!(z)), base!(cst!(1)))
            )),
            atom!(eq!(
                base!(var!(w)),
                base!(cst!(3))
            ))
        );
        assert_eq!(eval_formula(&f, &vec![vv(String::from("w"), 3)]), Truth::True);
    }
}