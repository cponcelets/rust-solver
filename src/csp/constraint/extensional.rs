/**************************************
- Author: Clement Poncelet
- Desc: Extensional Constraints, list of allowed tuples
***************************************/

/**************************************
            Extensional Constraint
***************************************/
use std::fmt;
use std::rc::Rc;
use crate::csp::constraint::traits::Constraint;
use crate::csp::domain::traits::OrdT;
use crate::csp::truth::Truth;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::{vv, VValue};

pub struct ExtConstraint<T: OrdT> {
    scope: Vec<Rc<ExVar<T>>>, //HashSet<Vec<VValue<T>>>
    allowed: Vec<Vec<VValue<T>>>,
}

impl<T: OrdT> ExtConstraint<T> {
    pub fn new( scp: Vec<Rc<ExVar<T>>>, rel: Vec<Vec<VValue<T>>>) -> Self {
        ExtConstraint { scope: scp, allowed: rel }
    }
}

impl<T: OrdT> Constraint<T> for ExtConstraint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in &self.allowed {
            write!(f, "  [")?;
            for vv in i {
                write!(f, "{}", vv)?;
            }
            writeln!(f, "],")?;
        }
        write!(f, "}}")
    }

    //---- Overriding -----------
    fn is_entailed(&self) -> bool {
        self.allowed.len() == 0
    }

    fn check_assignment(&self, asn: &Vec<VValue<T>>) -> Truth {
        if self.allowed.iter().any(|t| t == asn) {
            Truth::True
        } else {
            Truth::False
        }
    }

    fn is_support(&self, vvalue: &VValue<T>) -> Truth {
        if self.allowed.iter().any(|tuple|
            tuple.iter().any(|vv| vv == vvalue)
        ) {
            Truth::True
        } else {
            Truth::False
        }
    }
    fn rel(&self) -> Vec<Vec<VValue<T>>> { self.allowed.clone()}
    //---- Overriding -----------

    fn apply(&self, x: &T, y: &T) -> bool {
        if self.check_assignment(&vec![vv(String::from("x"), x.clone()), vv(String::from("y"), y.clone())]) == Truth::True { true }
        else {false}
    }

    fn scp(&self) -> &[Rc<ExVar<T>>] { &self.scope }
}


/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::prelude::extensional::VValue;
use std::rc::Rc;
    use crate::csp::constraint::extensional::ExtConstraint;
    use crate::csp::constraint::traits::Constraint;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::truth::Truth;
    use crate::csp::variable::extvar::ExVar;
    use crate::vvals;

    #[test]
    fn ext_constraint_rel() {
        let dom = SetDom::new(vec![1, 2, 3]);

        let x = Rc::new(ExVar::new("x".into(), dom.clone()));
        let y = Rc::new(ExVar::new("y".into(), dom));
        let c = ExtConstraint::new(
            vec![x.clone(), y.clone()],
            vec![
                vvals!("x" => 1, "y" => 1),
                vvals!("x" => 2, "y" => 3),
            ],
        );

        let rel = c.rel();

        assert!(rel.contains(&vvals!("x" => 1, "y" => 1)));
        assert!(!rel.contains(&vvals!("x" => 1, "y" => 2)));
    }

    #[test]
    fn ext_constraint_support() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), dom.clone()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = ExtConstraint::new(
            vec![x.clone(), y.clone()],
            vec![vvals!("x" => 1, "y" => 2)],
        );

        assert_eq!(
            c.is_support(&VValue { label: "x".into(), value: 1 }),
            Truth::True
        );

        assert_eq!(
            c.is_support(&VValue { label: "y".into(), value: 1 }),
            Truth::False
        );
    }

    #[test]
    fn ext_constraint_tightness() {
        let dom = SetDom::new(vec![1, 2]);

        let x = Rc::new(ExVar::new("x".into(), dom.clone()));
        let y = Rc::new(ExVar::new("y".into(), dom));

        let c = ExtConstraint::new(
            vec![x, y],
            vec![vvals!("x" => 1, "y" => 1)],
        );

        assert_eq!(c.looseness(), 0.25);
        assert_eq!(c.tightness(), 0.75);
    }
}
