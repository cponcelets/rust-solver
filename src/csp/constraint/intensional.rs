/**************************************
- Author: Clement Poncelet
- Desc: Intensional Constraints, mathematical operators
***************************************/

/**************************************
              Factory
***************************************/


/**************************************
   Formula (Intensional Constraints)
***************************************/
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use crate::atom;
use crate::csp::ast::eval::Eval;
use crate::csp::ast::formula::{eval_formula, formula_scope, Formula};
use crate::csp::ast::pred::{pred_scope, Pred};
use crate::csp::constraint::constraint::Constraint;
use crate::csp::domain::domain::OrdT;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::VValue;
pub struct Intensional<T:OrdT, E:Eval> {
    scope:      Vec<Rc<ExVar<T>>>,
    formula: Rc<Formula<E>>
}

impl<T:OrdT, E:Eval<Output = T>> Intensional<T, E> {
    pub fn new(scope: Vec<Rc<ExVar<T>>>, constraint: Rc<Formula<E>>) -> Self {
        let mut scp = scope.clone();
        //reorder if not lexicographic order
        scp.sort_by(|a, b| a.label().cmp(b.label()));
        Self { scope: scp, formula: constraint }
    }

    pub fn from_formula(constraint: Rc<Formula<E>>) -> Self {
        let mut scope = HashSet::new();
        formula_scope(&constraint, &mut scope);
        let mut scp: Vec<_> = scope.into_iter().collect();
        //reorder if not lexicographic order
        scp.sort_by(|a, b| a.label().cmp(b.label()));
        Self { scope: scp, formula: constraint }
    }

    pub fn from_pred(pred: Pred<E>) -> Self {
        let mut scope = HashSet::new();
        pred_scope(&pred, &mut scope);
        let mut scp: Vec<_> = scope.into_iter().collect();
        //reorder if not lexicographic order
        scp.sort_by(|a, b| a.label().cmp(b.label()));
        Self { scope: scp, formula: Rc::new(atom!(pred))}
    }
}

impl<T:OrdT, E:Eval<Output = T>> Constraint<T> for Intensional<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formula)
    }

    fn apply(&self, asn: &Vec<VValue<T>>) -> bool {
        eval_formula(&self.formula, asn).to_bool().unwrap()
    }

    fn scp(&self) -> &[Rc<ExVar<T>>] {
        &self.scope
    }
}

impl<T: OrdT, E: Eval<Output = T>> Debug for Intensional<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Intensional")
            .field("scope", &self.scope)
            .field("formula", &self.formula)
            .finish()
    }
}

impl<T:OrdT, E:Eval<Output = T>> Display for Intensional<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}


/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::ast::expr::Expr;
use crate::csp::ast::pred::Pred;
use crate::csp::ast::formula::Formula;
use std::rc::Rc;
    use crate::csp::constraint::constraint::Constraint;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::domain::domain::Domain;
    use crate::csp::truth::Truth;
    use crate::csp::variable::extvar::ExVar;
    use crate::csp::variable::vvalue::{vv, VValue};
    use crate::{atom, cst, eq, lt, neq, var, var_dom, vvals};
    use crate::csp::constraint::intensional::Intensional;

    #[test]
    fn check_invalid_value_is_false() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,3)
        let vv = VValue { label: "x".into(), value: 3 };

        assert_eq!(c.is_valid(&vv), Truth::False);
        assert_eq!(c.is_allowed(&vv), Truth::False);
    }

    #[test]
    fn support_implies_valid() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,1)
        let vv = VValue { label: "x".into(), value: 1 };

        assert_eq!(c.is_support(&vv), Truth::True);
        assert_eq!(c.is_valid(&vv), Truth::True);
    }

    #[test]
    fn allowed_but_not_valid() {
        let dom12 = SetDom::new(vec![1, 2]);
        let dom123 = SetDom::new(vec![1, 2, 3]);
        let x = Rc::new(ExVar::new("x".into(), dom12));
        let y = Rc::new(ExVar::new("y".into(), dom123));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,3)
        let vv = VValue { label: "x".into(), value: 3 };

        // 3 == 3 is allowed by equality
        assert_eq!(c.is_allowed(&vv), Truth::True);

        // but 3 ∉ dom(x)
        assert_eq!(c.is_valid(&vv), Truth::False);

        // therefore no strict support
        assert_eq!(c.strict_support(&vv), Truth::False);
    }

    #[test]
    fn support_is_not_conflict() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,1)
        let vv = VValue { label: "x".into(), value: 1 };

        assert_eq!(c.is_support(&vv), Truth::True);
        assert_eq!(c.is_conflicts(&vv), Truth::False);
    }

    #[test]
    fn valid_but_conflict() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,3)
        let vv = VValue { label: "x".into(), value: 3 };

        assert_eq!(c.is_valid(&vv), Truth::True);
        assert_eq!(c.is_support(&vv), Truth::False);
        assert_eq!(c.is_conflicts(&vv), Truth::True);
    }

    #[test]
    fn allowed_but_conflict() {
        let dom = SetDom::new(vec![1]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x <> y
        let f = Rc::new(atom!(neq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(x,1)
        let vv = VValue { label: "x".into(), value: 1 };

        assert_eq!(c.is_allowed(&vv), Truth::False);
        assert_eq!(c.is_valid(&vv), Truth::True);
        assert_eq!(c.is_support(&vv), Truth::False);
        assert_eq!(c.is_conflicts(&vv), Truth::True);
    }

    #[test]
    fn unknown_variable_truths() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(z,1)
        let vv = VValue { label: "z".into(), value: 1 };

        assert_eq!(c.is_valid(&vv), Truth::Unknown);
        assert_eq!(c.is_allowed(&vv), Truth::Unknown);
        assert_eq!(c.is_support(&vv), Truth::Unknown);
        assert_eq!(c.is_conflicts(&vv), Truth::Unknown);
    }

    #[test]
    fn unknown_propagation() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);
        //(z,1)
        let vv = VValue { label: "z".into(), value: 1 };

        assert_eq!(c.is_valid(&vv), Truth::Unknown);
    }

    #[test]
    fn eq_constraint_assignment() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(
            c.check_assignment(&vvals!("x" => 1, "y" => 1)),
            Truth::True
        );

        assert_eq!(
            c.check_assignment(&vvals!("x" => 1, "y" => 2)),
            Truth::False
        );
    }

    #[test]
    fn eq_constraint_strict_support() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 1 }),
            Truth::True
        );

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 3 }),
            Truth::False
        );
    }

    #[test]
    fn neq_constraint_support() {
        let dom = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x == y
        let f = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 1 }),
            Truth::True
        );
    }

    #[test]
    fn lt_constraint_support() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = Rc::new(ExVar::new("x".into(), dom.snapshot()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 3 }),
            Truth::False
        );

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 1 }),
            Truth::True
        );
    }

    #[test]
    fn lt_looseness() { //Figure1.4 book
        let domx = SetDom::new(vec![1, 2, 3]);
        let domy = SetDom::new(vec![0, 1, 2, 3]);
        let x = Rc::new(ExVar::new("x".into(), domx));
        let y = Rc::new(ExVar::new("y".into(), domy));

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(c.rel().len(), 3);
        assert_eq!(c.is_valid(&vv("x".into(), 3)), Truth::True);
        assert_eq!(c.is_valid(&vv("y".into(), 0)), Truth::True);
        //supports
        assert_eq!(c.is_support(&vv("x".into(), 3)), Truth::False);
        assert_eq!(c.is_support(&vv("x".into(), 1)), Truth::True);
        assert_eq!(c.is_support(&vv("y".into(), 1)), Truth::False);
        assert_eq!(c.is_support(&vv("y".into(), 2)), Truth::True);

        assert_eq!(c.tightness(), 9./12.);
    }

    #[test]
    fn constraint_entail() {
        //val(c) = sup(c)
        let domx = SetDom::new(vec![1, 2]);
        let domy = SetDom::new(vec![3, 4]);
        let x = Rc::new(ExVar::new("x".into(), domx));
        let y = Rc::new(ExVar::new("y".into(), domy));

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(c.is_entailed(), true);
    }

    #[test]
    fn constraint_disentail() {
        //val(c) = sup(c)
        let domx = SetDom::new(vec![3, 4]);
        let domy = SetDom::new(vec![1, 2]);
        let x = Rc::new(ExVar::new("x".into(), domx));
        let y = Rc::new(ExVar::new("y".into(), domy));

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x,y], f);

        assert_eq!(c.is_disentailed(), true);
    }

    #[test]
    fn support_removed_by_trailing() {
        let d1 = SetDom::new(vec![1,2]);
        let d2 = SetDom::new(vec![1,2]);
        let x = var_dom!("x".into(), d1);
        let y = var_dom!("y".into(), d2);

        // x < y
        let f = Rc::new(atom!(lt!(var!(x), var!(y))));
        let c = Intensional::new(vec![x.clone(),y], f);
        //(x,1)
        let vv = vv(x.label().clone(), 1);

        assert_eq!(c.is_support(&vv), Truth::True);

        c.scp()[1].dom_mut().remove_value(&2, 1); // y cannot be 2 anymore

        assert_eq!(c.is_support(&vv), Truth::False);
    }

    #[test]
    fn formula_constraint_check() {
        let dom =   SetDom::new(vec![1, 2, 3, 4]);
        let w =   Rc::new(ExVar::new("w".into(), dom));

        // w == 3
        let f = Rc::new(atom!(eq!(var!(w), cst!(3))));
        let c = Intensional::new(vec![w], f);

        let a = vec![vv(String::from("w"), 3)];
        assert_eq!(c.check_assignment(&a), Truth::True);

        let b = vec![vv(String::from("w"), 2)];
        assert_eq!(c.check_assignment(&b), Truth::False);
    }
}
