/**************************************
- Author: Clement Poncelet
- Desc: V-Values, pair of variable name and value.
        Mainly used as assignment into the solver.
***************************************/

/**************************************
            Factories
***************************************/

pub fn vv<T: OrdT>(label: String, value: T) -> VValue<T> {
    VValue {
        label: label.to_string(),
        value,
    }
}

pub fn make_assignment<T:OrdT>(scope: &[Rc<ExVar<T>>], values: Vec<T>)
                           -> Vec<VValue<T>> {
    scope.iter()
        .zip(values)
        .map(|(v, val)| VValue {
            label: v.label().clone(),
            value: val,
        })
        .collect()
}

/**************************************
            V-Values
***************************************/
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::csp::domain::domain::OrdT;
use crate::csp::variable::extvar::ExVar;

#[derive(Debug, Clone)]
pub struct VValue<T:OrdT> {
    pub label: String,
    pub value: T
}

// Implement Eq + PartialEq
impl<T:OrdT> PartialEq for VValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.value == other.value
    }
}

impl<T:OrdT> Hash for VValue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
        self.value.hash(state);
    }
}

impl<T:OrdT> Eq for VValue<T> {}

impl<T:OrdT> fmt::Display for VValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.label, self.value)
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::csp::variable::vvalue::{vv, VValue};
    use crate::vvals;

    #[test]
    fn vvalue_equality() {
        let a = VValue { label: "x".into(), value: 1 };
        let b = VValue { label: "x".into(), value: 1 };
        let c = VValue { label: "x".into(), value: 2 };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn vvalue_hashing() {
        let mut set = HashSet::new();
        set.insert(VValue { label: "x".into(), value: 1 });

        assert!(set.contains(&VValue { label: "x".into(), value: 1 }));
        assert!(!set.contains(&VValue { label: "x".into(), value: 2 }));
    }

    #[test]
    fn vvalue_clone() {
        let v = vv(String::from("x"), 1);
        let c = v.clone();

        assert_eq!(v, c);
    }

    #[test]
    fn vvalue_macro() {
        let a = vvals! {
                "x" => 1,
                "y" => 2,
                "x" => 1,
            };

        assert_eq!(a[0],a[2]);
    }
}