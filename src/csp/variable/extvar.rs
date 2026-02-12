
/**************************************
- Author: Clement Poncelet
- Desc: Variable, a string label associated with a domain<T> (set of possible values)
        ExVar because it is for now implemented using extensional domains
***************************************/

/**************************************
            Factories
***************************************/

pub fn generate_variables<T:OrdT>(base_name: &str, n:usize, dom : &SetDom<T>) -> HashMap<String, Rc<ExVar<T>>> {
    let mut vmap = HashMap::new();
    for i in 1..n+1 {
        vmap.insert(
            String::from(base_name.to_owned() + &*i.to_string()),
            var_dom!(String::from(base_name.to_owned() + &*i.to_string()), Clone::clone(&dom))
        );
    }
    vmap
}

/**************************************
            Variables
***************************************/
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::csp::domain::setdom::SetDom;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::{var_dom};

#[derive(Debug)]
pub struct ExVar<T:OrdT> {
    label: String,
    dom:  Rc<RefCell<SetDom<T>>>
}

impl<T: OrdT> PartialEq for ExVar<T> {
    fn eq(&self, other: &Self) -> bool {
        self.label() == other.label()
    }
}

impl<T: OrdT> Eq for ExVar<T> {}

impl<T: OrdT> Hash for ExVar<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label().hash(state);
    }
}


impl<T:OrdT> ExVar<T> {
    pub fn new (label: String, dom: SetDom<T>) -> ExVar<T> {
        let ref_dom = Rc::new(RefCell::new(dom));
        Self {
            label,
            dom: ref_dom
        }
    }
    pub fn value(&self) -> Option<T> {
        if self.dom().size() > 1  {None}
        else {Some(self.dom().active_values().get(0).unwrap().clone())}
    }
    pub fn valid_values(&self) -> Vec<T> { self.dom().active_values() }
    pub fn valid_size(&self) -> usize { self.dom().size() }
    pub fn label(&self) -> &String {&self.label}
    pub fn dom(&self) -> std::cell::Ref<'_, SetDom<T>> {
        self.dom.borrow()
    }
    pub fn dom_mut(&self) -> std::cell::RefMut<'_, SetDom<T>> {
        self.dom.borrow_mut()
    }
}

impl<T:OrdT> fmt::Display for ExVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {:?}", self.label, self.dom().active_values())
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::csp::domain::domain::Domain;
    use crate::csp::variable::extvar::SetDom;
    use crate::csp::variable::extvar::ExVar;

    fn int_dom() -> SetDom<i32> {
        SetDom::new(vec![1, 2, 3])
    }

    #[test]
    fn exvar_new_and_label() {
        let x = ExVar::new("x".to_string(), int_dom());
        assert_eq!(x.label(), "x");
    }

    #[test]
    fn exvar_domain_access() {
        let dom = int_dom();
        let x = ExVar::new("x".to_string(), dom.snapshot());

        assert_eq!(x.dom().deref(), &dom);
        assert_eq!(x.valid_values(), vec![1, 2, 3]);
        assert_eq!(x.valid_size(), 3);
    }

    #[test]
    fn exvar_initial_value_is_none() {
        let x = ExVar::new("x".to_string(), int_dom());
        assert_eq!(x.value(), None);
    }

    #[test]
    fn exvar_dom_read() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = ExVar::new("x".into(), dom);

        let values = x.dom().active_values();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn exvar_dom_mut_remove_value() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = ExVar::new("x".into(), dom);

        {
            let mut d = x.dom_mut();
            d.remove_value(&2, 0);
        }

        let values = x.dom().active_values();
        assert_eq!(values, vec![1, 3]);
    }

    #[test]
    fn exvar_snapshot_distinct_domain() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = ExVar::new("x".into(), dom.snapshot());
        let y = ExVar::new("y".into(), dom);

        {
            let mut dx = x.dom_mut();
            dx.remove_value(&1, 0);
        }

        let vx = x.dom().active_values();
        let vy = y.dom().active_values();

        assert_eq!(vx, vec![2, 3]);
        assert_eq!(vy, vec![1, 2, 3]);
    }
}