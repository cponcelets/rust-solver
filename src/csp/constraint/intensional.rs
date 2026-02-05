/**************************************
- Author: Clement Poncelet
- Desc: Intensional Constraints, mathematical operators
***************************************/

/**************************************
            EqConstraint
***************************************/
use std::fmt;
use std::rc::Rc;
use crate::csp::constraint::traits::Constraint;
use crate::csp::domain::traits::OrdT;
use crate::csp::variable::extvar::ExVar;

pub struct EqConstraint<T:OrdT> {
    operands: [Rc<ExVar<T>>; 2],
}

impl<T:OrdT> EqConstraint<T> {
    pub fn new(left: Rc<ExVar<T>>, right: Rc<ExVar<T>>) -> Self {
        Self { operands: [left, right] }
    }
}

impl<T:OrdT> Constraint<T> for EqConstraint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} == {}", self.operands[0].label(), self.operands[1].label())
    }

    fn apply(&self, x: &T, y: &T) -> bool {
        x == y
    }

    fn scp(&self) -> &[Rc<ExVar<T>>] { &self.operands }
}

/**************************************
            LtConstraint
***************************************/
pub struct LtConstraint<T:OrdT> {
    operands: [Rc<ExVar<T>>; 2],
}

impl<T:OrdT> LtConstraint<T> {
    pub fn new(left: Rc<ExVar<T>>, right: Rc<ExVar<T>>) -> Self {
        Self { operands: [left, right] }
    }
}

impl<T: OrdT> Constraint<T> for LtConstraint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} < {}", self.operands[0].label(), self.operands[1].label())
    }

    //other.domain().min() <= v && v <= other.domain().max()
    fn apply(&self, x: &T, y: &T) -> bool {
        x < y
    }

    fn scp(&self) -> &[Rc<ExVar<T>>] { &self.operands }
}

/**************************************
            NeqConstraint
***************************************/
pub struct NeqConstraint<T: OrdT> {
    operands: [Rc<ExVar<T>>; 2],
}

impl<T: OrdT> NeqConstraint<T> {
    pub fn new(left: Rc<ExVar<T>>, right: Rc<ExVar<T>>) -> Self {
        Self { operands: [left, right] }
    }
}

impl<T: OrdT> Constraint<T> for NeqConstraint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} != {}", self.operands[0].label(), self.operands[1].label())
    }

    fn apply(&self, x: &T, y: &T) -> bool {
        x != y
    }
    fn scp(&self) -> &[Rc<ExVar<T>>] { &self.operands }
}

/**************************************
        Test
***************************************/

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::csp::constraint::intensional::{EqConstraint, LtConstraint, NeqConstraint};
    use crate::csp::constraint::traits::Constraint;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::domain::traits::Domain;
    use crate::csp::truth::Truth;
    use crate::csp::variable::extvar::ExVar;
    use crate::csp::variable::vvalue::{vv, VValue};
    use crate::{var, vvals};

    #[test]
    fn check_invalid_value_is_false() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

        let vv = VValue { label: "x".into(), value: 3 };

        assert_eq!(c.is_valid(&vv), Truth::False);
        assert_eq!(c.check(&vv), Truth::False);
    }

    #[test]
    fn support_implies_valid() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

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

        let c = EqConstraint::new(x.clone(), y.clone());

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

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

        let vv = VValue { label: "x".into(), value: 1 };

        assert_eq!(c.is_support(&vv), Truth::True);
        assert_eq!(c.is_conflicts(&vv), Truth::False);
    }

    #[test]
    fn valid_but_conflict() {
        let dom = SetDom::new(vec![1, 2, 3]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = LtConstraint::new(x.clone(), y.clone());

        let vv = VValue { label: "x".into(), value: 3 };

        assert_eq!(c.is_valid(&vv), Truth::True);
        assert_eq!(c.is_support(&vv), Truth::False);
        assert_eq!(c.is_conflicts(&vv), Truth::True);
    }

    #[test]
    fn allowed_but_conflict() {
        let dom = SetDom::new(vec![1]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = NeqConstraint::new(x.clone(), y.clone());

        let vv = VValue { label: "x".into(), value: 1 };

        assert_eq!(c.is_allowed(&vv), Truth::False);
        assert_eq!(c.is_valid(&vv), Truth::True);
        assert_eq!(c.is_support(&vv), Truth::False);
        assert_eq!(c.is_conflicts(&vv), Truth::True);
    }

    #[test]
    fn unknown_variable_truths() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x, y);

        let vv = VValue { label: "z".into(), value: 1 };

        assert_eq!(c.is_valid(&vv), Truth::Unknown);
        assert_eq!(c.is_allowed(&vv), Truth::Unknown);
        assert_eq!(c.is_support(&vv), Truth::Unknown);
        assert_eq!(c.is_conflicts(&vv), Truth::Unknown);
    }

    #[test]
    fn unknown_propagation() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

        let vv = VValue { label: "z".into(), value: 1 };

        assert_eq!(c.is_valid(&vv), Truth::Unknown);
    }

    #[test]
    fn eq_constraint_assignment() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

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

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = EqConstraint::new(x.clone(), y.clone());

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

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = NeqConstraint::new(x.clone(), y.clone());

        assert_eq!(
            c.strict_support(&VValue { label: "x".into(), value: 1 }),
            Truth::True
        );
    }

    #[test]
    fn lt_constraint_support() {
        let dom = SetDom::new(vec![1, 2, 3]);

        let x = Rc::new(ExVar::new("x".into(), Clone::clone(&dom)));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = LtConstraint::new(x.clone(), y.clone());

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

        let c = LtConstraint::new(x.clone(), y.clone());

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

        let c = LtConstraint::new(x.clone(), y.clone());

        assert_eq!(c.is_entailed(), true);
    }

    #[test]
    fn constraint_disentail() {
        //val(c) = sup(c)
        let domx = SetDom::new(vec![3, 4]);
        let domy = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), domx));
        let y = Rc::new(ExVar::new("y".into(), domy));

        let c = LtConstraint::new(x.clone(), y.clone());

        assert_eq!(c.is_disentailed(), true);
    }

    #[test]
    fn support_removed_by_trailing() {
        let mut d1 = SetDom::new(vec![1,2]);
        let mut d2 = SetDom::new(vec![1,2]);

        let x = var!("x".into(), d1);
        let y = var!("y".into(), d2);

        let c = LtConstraint::new(x.clone(), y.clone());
        let vv = vv(x.label().clone(), 1);

        assert_eq!(c.is_support(&vv), Truth::True);

        c.operands[1].dom_mut().remove_value(&2, 1); // y cannot be 2 anymore

        assert_eq!(c.is_support(&vv), Truth::False);
    }
}
