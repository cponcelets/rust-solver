/**************************************
- Author: Clement Poncelet
- Desc: Arc type for Arc-oriented (coarse-grained) propagation scheme
    - An arc is a tuple <c, x> of a constraint and a variable x in scp(c)
***************************************/

use std::fmt;
use std::rc::Rc;
use crate::csp::constraint::constraint::Constraint;
use crate::csp::domain::domain::OrdT;
use crate::csp::prelude::extvar::ExVar;
/**************************************
            Type Arc
***************************************/

pub struct Arc<T:OrdT> {
    pub constraint: Rc<dyn Constraint<T>>,
    pub variable: Rc<ExVar<T>>
}

impl<T:OrdT> fmt::Display for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.constraint.label(), self.variable.label())
    }
}

impl<T:OrdT> fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arc")
            .field("constraint", &self.constraint)
            .field("variable", &self.variable)
            .finish()
    }
}