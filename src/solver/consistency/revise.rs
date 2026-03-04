/**************************************
- Author: Clement Poncelet
- Desc: Propagation variant (revise function)
        - AC1 Algorithm 8 (calling seekSupport)
        - AC3 Algorithm 18 (seekSupport-3)
        - AC2001 Algorithm 19 (seekSupport-2001)
- Optimization:
    - output Option<TriggerEvent>
***************************************/
use std::collections::HashMap;
use std::rc::Rc;
use crate::csp::constraint::constraint::Constraint;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::prelude::extvar::ExVar;
use crate::csp::prelude::vvalue::{vv, vv_from_hashmap};
use crate::instrumentation::monitor::Monitor;
use crate::solver::consistency::arc::Arc;
use crate::solver::consistency::cvalue::CValue;

pub trait Revise<M: Monitor, T: OrdT> {
    fn revise(&mut self, arc : &Arc<T>, level: usize, monitor: &mut M) -> bool;
}

pub struct AC1;

impl<M: Monitor, T:OrdT> Revise<M, T> for AC1 {
    //simple version AC1?
    //true if revision (c,x) effective
    fn revise(&mut self, arc: &Arc<T>, level: usize, monitor : &mut M) -> bool {
        monitor.on_revision_check();
        let size_before = arc.variable.dom().size();
        for a in arc.variable.valid_values() {
            monitor.on_constraint_check();
            if !seek_support(arc.constraint.clone(), arc.variable.clone(), &a) {
                println!("remove {} from {}", a, arc.variable);
                monitor.on_value_deleted();
                arc.variable.dom_mut().remove_value(&a, level);
            }
        }
        size_before != arc.variable.dom().size()
    }
}

fn seek_support<T:OrdT>(c: Rc<dyn Constraint<T>>, x: Rc<ExVar<T>>, a: &T) -> bool {
    c.is_support_asn(&vec![vv(x.label().clone(), a.clone())], false).to_bool().unwrap()
}

pub struct AC3;

impl<M: Monitor, T:OrdT> Revise<M, T> for AC3 {
    fn revise(&mut self, arc : &Arc<T>, level: usize, monitor : &mut M) -> bool {
        monitor.on_revision_check();
        //AC3
        let size_before = arc.variable.dom().size();
        for a in arc.variable.valid_values() {
            if !seek_support3(&CValue {
                constraint: arc.constraint.clone(),
                variable: arc.variable.clone(),
                value: a.clone()
                },
                monitor) {
                println!("remove {} from {}", &a, arc.variable);
                monitor.on_value_deleted();
                arc.variable.dom_mut().remove_value(&a, level);
            }
        }
        size_before != arc.variable.dom().size()
    }
}

fn seek_support3<M: Monitor, T:OrdT>(cval:&CValue<T>, monitor: &mut M) -> bool {
    let mut tuple = Some(cval.get_first_valid_tuple());
    while !tuple.is_none() {
        monitor.on_constraint_check();
        let tau = tuple.unwrap();
        if cval.constraint.check_assignment(&vv_from_hashmap(&tau)).to_bool().unwrap() {
            return true;
        }
        tuple = cval.get_next_valid_tuple(&tau);
    }
    false
}

pub struct AC2001<M: Monitor, T: OrdT> {
    last: HashMap<String, HashMap<String, T>>,
    _phantom: std::marker::PhantomData<M>
}

impl<M: Monitor, T:OrdT> Revise<M, T> for AC2001<M, T> {
    fn revise(&mut self, arc : &Arc<T>, level: usize, monitor : &mut M) -> bool {
        monitor.on_revision_check();
        //AC2001
        let size_before = arc.variable.dom().size();
        for a in arc.variable.valid_values() {
            if !self.seek_support2001(CValue {
                constraint: arc.constraint.clone(),
                variable: arc.variable.clone(),
                value: a.clone()
                },
                monitor) {
                println!("remove {} from {}", &a, arc.variable);
                monitor.on_value_deleted();
                arc.variable.dom_mut().remove_value(&a, level);
            }
        }
        size_before != arc.variable.dom().size()
    }
}

impl<M: Monitor, T:OrdT> AC2001<M, T> {
    pub(crate) fn new() -> Self {
        Self {
            last: HashMap::new(),
            _phantom: std::marker::PhantomData
        }
    }

    pub fn last_supports(&self) -> &HashMap<String, HashMap<String, T>> {&self.last}

    //optimal for binary constraints
    fn seek_support2001(&mut self, cval: CValue<T>, monitor: &mut M) -> bool {
        let mut tau = None;
        let last_support = self.last.get(&cval.label());

        match last_support {
            None => tau = Some(cval.get_first_valid_tuple()),
            Some(l_cval) => {
                let j = cval.constraint.get_first_invalid_pos(Some(l_cval));
                if j == -1 {
                    return true;
                } else {
                    tau = cval.get_next_valid_tuple_limit(l_cval, j);
                }
            }
        }
        while !tau.is_none() {
            let t = tau.expect("Should not be None!");
            monitor.on_constraint_check();
            if cval.constraint.check_assignment(&vv_from_hashmap(&t)).to_bool().unwrap() {
                if let Some (e) = self.last.get_mut(&cval.label()) {
                    *e = t;
                } else {
                    self.last.insert(cval.label(), t);
                }
                return true;
            }
            tau = cval.get_next_valid_tuple(&t);
        }
        false
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
    use crate::csp::prelude::extvar::ExVar;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::csp::domain::setdom::SetDom;
    use crate::{and, atom, cst, eq, or, var, var_dom, vvals};
    use crate::csp::constraint::intensional::Intensional;
    use crate::csp::prelude::domain::Domain;
    use crate::csp::prelude::vvalue::{vv_from_hashmap, VValue};
    use crate::instrumentation::monitor::Statistics;
    use crate::solver::consistency::arc::{Arc};
    use crate::solver::consistency::cvalue::CValue;
    use crate::solver::consistency::revise::{Revise, AC2001, AC3};

    #[test] //figure 4.3
    fn test_revise() {
        let dom_x = SetDom::new(vec!["a", "b", "c"]);
        let dom_y = SetDom::new(vec!["a", "b", "c", "d"]);

        let x = var_dom!("x".into(), dom_x);
        let y = var_dom!("y".into(), dom_y);

        let mut vmap: HashMap<String, _> = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());

        let fxa = and!(
            atom!(eq!(var!(x), cst!("a"))),
            or!(atom!(eq!(var!(y), cst!("a"))), atom!(eq!(var!(y), cst!("b")))));
        let fxb = and!(
            atom!(eq!(var!(x), cst!("b"))),
            or!(atom!(eq!(var!(y), cst!("c"))), atom!(eq!(var!(y), cst!("d")))));
        let fxc = and!(
            atom!(eq!(var!(x), cst!("c"))),
            atom!(eq!(var!(y), cst!("d"))));

        let c =  Rc::new(Intensional::from_formula(Rc::new(
            and!(
                                    or!(fxa, fxb, fxc)
                                   ))));

        //also check the calls to revise ? -> Stats?
        let revision = Arc { constraint: c.clone(), variable: x.clone() };
        let mut consistency_ac3 = AC3;
        let mut monitor_ac3 = Statistics::default();
        //nothing to change
        assert!(!consistency_ac3.revise(&revision, 1, &mut monitor_ac3));
        //8 constraints checks
        assert_eq!(monitor_ac3.checks, 8);

        let mut consistency_ac2001 = AC2001::new();
        assert!(!consistency_ac2001.revise(&revision, 1, &mut monitor_ac3)); //just to not use NoMonitor...
        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "a" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "a", "y" => "a"));

        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "b" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "b", "y" => "c"));

        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "c" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "c", "y" => "d"));

        //Suppose now that the `v-value` $(y, c)$ has been deleted
        y.dom_mut().remove_value(&"c", 2);

        let mut monitor_ac2001 = Statistics::default();
        //GAC3 -> 7 constraints checks
        assert!(!consistency_ac3.revise(&revision, 1, &mut monitor_ac2001)); //still nothing to remove
        assert_eq!(monitor_ac2001.checks, 7);

        let mut monitor_ac2001_2 = Statistics::default();
        //GAC2001 -> 1 constraints check + 3 validity checks
        assert!(!consistency_ac2001.revise(&revision, 1, &mut monitor_ac2001_2));
        assert_eq!(monitor_ac2001_2.checks, 1);
        //add validity checks? (push the monitor into constraints
        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "a" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "a", "y" => "a"));

        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "b" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "b", "y" => "d"));

        assert_eq!(vv_from_hashmap(consistency_ac2001.last_supports().get(
            &CValue { constraint: c.clone(), variable: x.clone(), value: "c" }.label())
            .expect("Should not be None")),
                   vvals!("x" => "c", "y" => "d"));
    }
}
