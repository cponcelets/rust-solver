/**************************************
- Author: Clement Poncelet
- Desc: Forward checking, backward search algorithm with partial form of GAC
-       Serve as a standalone (and tests) for consistency enforcement using AC1 (useful?)
***************************************/
use std::rc::Rc;
use crate::csp::csp::Csp;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::prelude::extvar::ExVar;
use crate::instrumentation::monitor::NoMonitor;
use crate::solver::consistency::arc::{Arc};
use crate::solver::consistency::revise::{Revise, AC1};

// Return true iff FC(P,x) not bot
fn apply_fc<T:OrdT>(csp :&Csp<T>, x : Rc<ExVar<T>>) -> bool {
    apply_fc_set(csp, vec![x.label()])
}

fn apply_fc_set<T:OrdT>(csp :&Csp<T>, events: Vec<&String>) -> bool {
    assert!(events.iter().all(|v| csp.past().contains(v)),
            "Require events instantiated (into csp.Past)");

    let mut ac1 = AC1;
    for c in csp.constraints() {
        if c.scp().iter().any(|x| events.contains(&x.label())) {
            for y in c.scp() {
                if !csp.past().contains(&y.label()) {
                    if ac1.revise(&Arc { constraint: c.clone(), variable: y.clone() }, csp.level(), &mut NoMonitor) {
                        if y.dom().is_empty() {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::ast::expr::{Expr};
    use crate::csp::ast::pred::Pred;
    use crate::csp::ast::formula::Formula;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::variable::extvar::ExVar;
    use crate::{and, atom, cst, eq, or, var, var_dom};
    use crate::csp::constraint::intensional::Intensional;
    use crate::csp::csp::Csp;
    use crate::csp::prelude::domain::Domain;
    use crate::csp::prelude::vvalue::vv;
    use crate::solver::consistency::fc::apply_fc;

    #[test] //figure 4.2
    fn fc_example() {
        let dom = SetDom::new(vec!["a", "b", "c", "d"]);

        let x = var_dom!("x".into(), dom.snapshot());
        let y = var_dom!("y".into(), dom.snapshot());
        let z = var_dom!("z".into(), dom);

        let mut vmap: HashMap<String, _> = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());
        vmap.insert("z".into(), z.clone());

        //w == x
        let fxa = and!(
            atom!(eq!(var!(x), cst!("a"))),
            atom!(eq!(var!(y), cst!("b"))));
        let fxb = and!(
            atom!(eq!(var!(x), cst!("b"))),
            atom!(eq!(var!(y), cst!("a"))));
        let fxc = and!(
            atom!(eq!(var!(x), cst!("c"))),
            or!(atom!(eq!(var!(y), cst!("b"))), atom!(eq!(var!(y), cst!("d")))));
        let fxd = and!(
            atom!(eq!(var!(x), cst!("d"))),
            atom!(eq!(var!(y), cst!("c"))));

        let fya = and!(
            atom!(eq!(var!(y), cst!("a"))),
            atom!(eq!(var!(z), cst!("d"))));
        let fyb = and!(
            atom!(eq!(var!(y), cst!("b"))),
            atom!(eq!(var!(z), cst!("a"))));
        let fyc = and!(
            atom!(eq!(var!(y), cst!("c"))),
            or!(atom!(eq!(var!(z), cst!("b"))), atom!(eq!(var!(z), cst!("c")))));
        let fyd = and!(
            atom!(eq!(var!(y), cst!("d"))),
            or!(atom!(eq!(var!(z), cst!("a"))), atom!(eq!(var!(z), cst!("d")))));

        let mut csp = Csp::new(vmap,
                               vec![ Rc::new(Intensional::from_formula(Rc::new(
                                   and!(
                                    or!(fxa, fxb, fxc, fxd),
                                    or!(fya, fyb, fyc, fyd)
                                   ))))
                           ]);

        // y -> "c"
        csp.assign(vv(y.label().clone(), "c"));
        assert!(apply_fc(&csp, y.clone()));

        //Result: {<x,"d">,<y, "c">, <z, "b">, <z, "c">
        assert_eq!(x.dom().active_values(), vec!["d"]);
        assert_eq!(y.dom().active_values(), vec!["c"]);
        assert_eq!(z.dom().active_values(), vec!["b", "c"]);
    }
}
