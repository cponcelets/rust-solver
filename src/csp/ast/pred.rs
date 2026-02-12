/**************************************
- Author: Clement Poncelet
- Desc: Predicates, boolean expressions.
    - eq, neq, lt, le, gt, ge
***************************************/

/**************************************
            Predicates
***************************************/
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;
use crate::csp::ast::eval::Eval;
use crate::csp::domain::domain::OrdT;
use crate::csp::truth::Truth;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::VValue;

#[derive(Clone, Debug)]
pub enum Pred<E> where E: Eval {
    Eq(E, E),
    Neq(E, E),
    Lt(E, E),
    Le(E, E),
    Gt(E, E),
    Ge(E, E)
}

impl<E> Pred<E>  where E: Eval,
{
    pub fn eq(a: E, b: E) -> Self { Pred::Eq(a, b) }
    pub fn neq(a: E, b: E) -> Self { Pred::Neq(a, b) }
    pub fn lt(a: E, b: E) -> Self { Pred::Lt(a, b) }
    pub fn le(a: E, b: E) -> Self { Pred::Le(a, b) }
    pub fn gt(a: E, b: E) -> Self { Pred::Gt(a, b) }
    pub fn ge(a: E, b: E) -> Self { Pred::Ge(a, b) }

    pub fn eval(&self, asn: &Vec<VValue<E::Output>>) -> Truth {
        use Pred::*;
        let (a, b) = match self {
            Eq(x, y) | Neq(x, y)
            | Lt(x, y) | Le(x, y)
            | Gt(x, y) | Ge(x, y)
            => (x, y),
        };

        let va = a.eval(asn);
        let vb = b.eval(asn);

        match (va, vb) {
            (Some(x), Some(y)) => {
                let ok = match self {
                    Eq(_, _) => x == y,
                    Neq(_, _) => x != y,
                    Lt(_, _) => x < y,
                    Le(_, _) => x <= y,
                    Gt(_, _) => x > y,
                    Ge(_, _) => x >= y
                };
                Truth::from(ok)
            }
            _ => Truth::Unknown
        }
    }
}

impl<E> Display for Pred<E>  where E: Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_predicate(self))
    }
}

pub fn print_predicate<E: Eval>(p: &Pred<E>) -> String {
    match p {
        Pred::Eq(a, b) =>
            a.print() + " == " + &*b.print(),
        Pred::Neq(a, b) =>
            a.print() + " <> " + &*b.print(),
        Pred::Lt(a, b) =>
            a.print() + " < " + &*b.print(),
        Pred::Le(a, b) =>
            a.print() + " <= " + &*b.print(),
        Pred::Gt(a, b) =>
            a.print() + " > " + &*b.print(),
        Pred::Ge(a, b) =>
            a.print() + " >= " + &*b.print(),
    }
}

/**************************************
           Utilities
***************************************/

pub fn pred_scope<E: Eval<Output = T>, T: OrdT>(p: &Pred<E>, acc: &mut HashSet<Rc<ExVar<T>>>) {
    match p {
        Pred::Eq(a,b) | Pred::Neq(a,b) | Pred::Lt(a,b)
        | Pred::Le(a,b) | Pred::Gt(a,b) | Pred::Ge(a,b) => {
            a.collect_vars(acc);
            b.collect_vars(acc);
        }
    }
}

/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{cst, eq, var, var_dom};
    use crate::csp::ast::expr::{AExpr, Expr};
    use crate::csp::ast::expr::Expr::{Const, Var};
    use crate::csp::ast::pred::Pred;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::truth::Truth;
    use crate::csp::variable::extvar::ExVar;
    use crate::csp::variable::vvalue::{vv};

    fn setup_vars() -> (Rc<ExVar<i32>>, Rc<ExVar<i32>>) {
        let dom = SetDom::new(vec![1, 2, 3, 4]);
        let w = Rc::new(ExVar::new("w".into(), dom.clone()));
        let z = Rc::new(ExVar::new("z".into(), dom));
        (w, z)
    }

    #[test]
    fn pred_building() {
        let (w, _) = setup_vars();
        let p = eq!(var!(w), cst!(3));
        match p {
            Pred::Eq(_, _) => {}
            _ => panic!("Expected Eq predicate"),
        }
    }

    #[test]
    fn pred_eval_true() {
        let x = var_dom!(String::from("x"), SetDom::new(vec![1, 2]));
        let asn = vec![vv(String::from("x"), 1)];

        let p = Pred::Eq(
            Expr::Var(x.clone()),
            Expr::Const(1),
        );

        assert_eq!(p.eval(&asn), Truth::True);
    }

    #[test]
    fn pred_eval_false() {
        let x = var_dom!(String::from("x"), SetDom::new(vec![1, 2]));
        let asn = vec![vv(String::from("x"), 2)];

        let p = Pred::Eq(
            Expr::Var(x.clone()),
            Expr::Const(1),
        );

        assert_eq!(p.eval(&asn), Truth::False);
    }

    #[test]
    fn pred_eval_unknown() {
        let x = var_dom!(String::from("x"), SetDom::new(vec![1, 2]));
        let asn = vec![];

        let p = Pred::Eq(
            Expr::Var(x.clone()),
            Expr::Const(1),
        );

        assert_eq!(p.eval(&asn), Truth::Unknown);
    }

    #[test]
    fn pred_arith_true() {
        let dom = SetDom::new(vec![0, 1, 2, 3]);
        let x = var_dom!(String::from("x"), dom.clone());
        let y = var_dom!(String::from("y"), dom);

        // x+1 == y
        let expr = Pred::Eq(
            AExpr::Add(
                Box::new(AExpr::Base(Var(x.clone()))),
                Box::new(AExpr::Base(Const(1))),
            ),
            AExpr::Base(Var(y.clone())),
        );
        //{(x,1),(y,2)}
        let asn = vec![
            vv(String::from("x"), 1),
            vv(String::from("y"), 2),
        ];

        assert_eq!(expr.eval(&asn), Truth::True);
    }

    #[test]
    fn pred_arith_false() {
        let dom = SetDom::new(vec![0, 1, 2, 3]);
        let x = var_dom!(String::from("x"), dom.clone());
        let y = var_dom!(String::from("y"), dom);

        // x+1 == y
        let expr = Pred::Eq(
            AExpr::Add(
                Box::new(AExpr::Base(Var(x.clone()))),
                Box::new(AExpr::Base(Const(1))),
            ),
            AExpr::Base(Var(y.clone())),
        );
        //{(x,1),(y,2)}
        let asn = vec![
            vv(String::from("x"), 1),
            vv(String::from("y"), 3),
        ];
        assert_eq!(expr.eval(&asn), Truth::False);
    }

    #[test]
    fn pred_arith_unknown() {
        let dom = SetDom::new(vec![0, 1, 2, 3]);
        let x = var_dom!(String::from("x"), dom.clone());
        let y = var_dom!(String::from("y"), dom);

        // x+1 == y
        let expr = Pred::Eq(
            AExpr::Add(
                Box::new(AExpr::Base(Var(x.clone()))),
                Box::new(AExpr::Base(Const(1))),
            ),
            AExpr::Base(Var(y.clone())),
        );
        //{(x,1)}
        let asn = vec![
            vv(String::from("x"), 1)
        ];
        assert_eq!(expr.eval(&asn), Truth::Unknown)
    }
}