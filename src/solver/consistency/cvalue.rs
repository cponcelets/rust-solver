/**************************************
- Author: Clement Poncelet
- Desc: Cvalue <c, x, a>, constraint, variable, value
- Optimization:
    - Improve Eq and hash (now on label())
***************************************/
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::csp::constraint::constraint::Constraint;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::prelude::extvar::ExVar;
use crate::csp::prelude::vvalue::vv_from_hashmap;

pub struct CValue<T:OrdT> {
    pub constraint: Rc<dyn Constraint<T>>,
    pub variable: Rc<ExVar<T>>,
    pub value: T
}

// Implement Eq + PartialEq
impl<T:OrdT> PartialEq for CValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
    }
}

impl<T:OrdT> Hash for CValue<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state);
    }
}

impl<T:OrdT> Eq for CValue<T> {}

impl<T:OrdT> CValue<T> {
    pub fn label(&self) -> String {
        format!("{}", vec![self.constraint.label(), self.variable.label().clone(), self.value.to_string()]
            .join("_"))
    }
    pub fn deep_clone(&self) -> Self {
        let c:Rc<dyn Constraint<T>> = self.constraint.deep_clone();
        let v = c.scp().iter().find(|v| v.label() == self.variable.label())
            .expect(&*format!("{} not found", self.variable.label()));
        Self {
            constraint: c.clone(),
            variable: v.clone(),
            value: self.value.clone()
        }
    }

    pub fn get_first_valid_tuple(&self) -> HashMap<String, T> {
        let c_check = self.deep_clone();
        c_check.variable.dom_mut().reduce_to(&c_check.value, 42); //to check but should not modify dom(cval.variable)
        assert!(!c_check.constraint.rel().is_empty());

        let mut tuple: HashMap<String, T> = HashMap::new();
        tuple.insert(self.variable.label().clone(), self.value.clone());

        for v in self.constraint.scp() {
            if v.label() != self.variable.label() {
                tuple.insert(v.label().clone(), v.dom().head().expect(&format!("Domain wipeout for variable {}", v)));
            }
        }
        tuple
    }

    pub fn get_next_valid_tuple(&self, tuple : &HashMap<String, T>) -> Option<HashMap<String, T>> {
        //tuple in val(c)_x=a
        let c_check = self.deep_clone();
        c_check.variable.dom_mut().reduce_to(&c_check.value, 42); //to check but should not modify dom(cval.variable)
        assert!(c_check.constraint.is_valid_asn(&vv_from_hashmap(tuple)).to_bool().unwrap());

        let mut ret = tuple.clone();
        for y in self.constraint.scp().iter().rev() {
            if y.label() != self.variable.label() {
                if let Some (t) = ret.get_mut(y.label()) {
                    match y.dom().next(&t) {
                        Some(next) => {*t = next; return Some(ret);}, //break at the first next found
                        None => *t = y.dom().head().expect(&format!("Domain wipeout for variable {}", y)),
                    }
                } else {
                    panic!("{}", format!("Domain wipeout for variable {}", y));
                }
            }
        }
        None
    }

    pub fn get_next_valid_tuple_limit(&self, tuple : &HashMap<String, T>, limit : i32) -> Option<HashMap<String, T>> {
        assert_eq!(tuple.get(self.variable.label()), Some(&self.value.clone()));

        //tuple not in val(c)_x=a
        let c_check = self.deep_clone();
        c_check.variable.dom_mut().reduce_to(&c_check.value, 42);
        assert!(!c_check.constraint.rel().contains(&vv_from_hashmap(tuple)));
        assert!(c_check.constraint.rel().len() > 0);

        assert_eq!(limit, self.constraint.get_first_invalid_pos(Some(tuple)));

        let mut ret = tuple.clone();
        for i in (limit+1) as usize..(self.constraint.scp().len() + 1) {
            let y = self.constraint.scp()[i-1].clone();
            if y != self.variable {
                let t = ret.get_mut(y.label()).expect("Error accessing tuple");
                *t = y.dom().head().expect(&format!("Domain wipeout for variable {}", y));
            }
        }

        for i in (1..(limit+1) as usize).rev() {
            let y = self.constraint.scp()[i-1].clone();
            if y != self.variable {
                if let Some (t) = ret.get_mut(y.label()) {
                    if t >= &mut y.dom().tail().expect("Error in domain") {
                        *t = y.dom().head().expect(&format!("Domain wipeout for variable {}", y));
                    } else {
                        *t = y.dom().next(t).expect(&format!("Domain wipeout for variable {}", y));
                        while y.dom().absent(t) != 0 {
                            *t = y.dom().next(t).expect(&format!("Domain wipeout for variable {}", y));
                        }
                        return Some(ret);
                    }
                } else {
                    panic!("{}", format!("Domain wipeout for variable {}", y));
                }
            }
        }
        None
    }
}
