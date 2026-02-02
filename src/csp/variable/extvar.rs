
/**************************************
- Author: Clement Poncelet
- Desc: Variable, a string label associated with a domain<T> (set of possible values)
        ExVar because it is for now implemented using extensional domains
***************************************/

/**************************************
            Factories
***************************************/

pub fn generate_variables<T:OrdT>(base_name: &str, n:usize, dom : &ExDom<T>) -> HashMap<String, Rc<ExVar<T>>> {
    let mut vmap = HashMap::new();
    for i in 1..n+1 {
        vmap.insert(
            String::from(base_name.to_owned() + &*i.to_string()),
            var!(String::from(base_name.to_owned() + &*i.to_string()), Clone::clone(&dom))
        );
    }
    vmap
}

/**************************************
            Variables
***************************************/
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use crate::csp::domain::extdom::ExDom;
use crate::csp::domain::traits::{Domain, OrdT};
use crate::var;

pub struct ExVar<T:OrdT> {
    label: String,
    dom: ExDom<T>
}

impl<T:OrdT> ExVar<T> {
    pub fn new (label: String, dom: ExDom<T>) -> ExVar<T> {
        ExVar { label, dom }
    }
    pub fn value(&self) -> Option<T> {
        if self.dom.size() > 1  {None}
        else {Some(self.dom.get().get(0).unwrap().clone())}
    }
    pub fn valid_values(&self) -> &Vec<T> { self.dom.get() }
    pub fn valid_size(&self) -> usize { self.dom.size() }
    pub fn label(&self) -> &String {&self.label}
    pub fn dom(&self) -> &ExDom<T> {&self.dom}
}

impl<T:OrdT> fmt::Display for ExVar<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.label, self.dom)
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::variable::extvar::ExDom;
    use crate::csp::variable::extvar::ExVar;

    fn int_dom() -> ExDom<i32> {
        ExDom::new(vec![1, 2, 3])
    }

    #[test]
    fn exvar_new_and_label() {
        let x = ExVar::new("x".to_string(), int_dom());
        assert_eq!(x.label(), "x");
    }

    #[test]
    fn exvar_domain_access() {
        let dom = int_dom();
        let x = ExVar::new("x".to_string(), dom.clone());

        assert_eq!(x.dom(), &dom);
        assert_eq!(x.valid_values(), &vec![1, 2, 3]);
        assert_eq!(x.valid_size(), 3);
    }

    #[test]
    fn exvar_initial_value_is_none() {
        let x = ExVar::new("x".to_string(), int_dom());
        assert_eq!(x.value(), None);
    }

}