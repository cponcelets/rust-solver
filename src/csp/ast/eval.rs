/**************************************
- Author: Clement Poncelet
- Desc: Trait Eval to merge arithmetic and simple expressions
***************************************/

/**************************************
            Eval
***************************************/
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;
use crate::csp::domain::domain::OrdT;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::VValue;

pub trait Eval : Debug {
    type Output: OrdT;
    fn eval(&self, asn: &Vec<VValue<Self::Output>>) -> Option<Self::Output>;
    fn print(&self) -> String;
    //Helper to gather variables in the scope
    fn collect_vars(&self, acc: &mut HashSet<Rc<ExVar<Self::Output>>>) -> ();
}