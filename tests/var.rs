/**************************************
- Author: Clement Poncelet
- Desc: Unit tests for Variable-oriented coarse-grained propagation
    - with time-stamping
***************************************/

/**************************************
        Unit Tests
***************************************/

use rust_solver::csp::ast::expr::{Expr, AExpr};
use rust_solver::csp::ast::pred::Pred;
use rust_solver::csp::ast::formula::Formula;
use std::collections::HashMap;
use std::rc::Rc;
use rust_solver::{add, and, atom, base, cst, eq, or, var, var_dom};
use rust_solver::csp::domain::setdom::SetDom;
use rust_solver::csp::prelude::extvar::ExVar;
use rust_solver::csp::constraint::intensional::Intensional;
use rust_solver::csp::csp::Csp;
use rust_solver::csp::domain::domain::Domain;
use rust_solver::solver::consistency::consistency::Consistency;
use rust_solver::solver::consistency::revise::AC1;
use rust_solver::solver::consistency::scheme::VariableOriented;
use rust_solver::instrumentation::monitor::NoMonitor;

#[test]
fn domino_example() {
    let dom = SetDom::new(vec![0, 1, 2, 3]);

    let w = var_dom!("w".into(), dom.snapshot());
    let x = var_dom!("x".into(), dom.snapshot());
    let y = var_dom!("y".into(), dom.snapshot());
    let z = var_dom!("z".into(), dom);

    let mut vmap: HashMap<String, _> = HashMap::new();
    vmap.insert("w".into(), w.clone());
    vmap.insert("x".into(), x.clone());
    vmap.insert("y".into(), y.clone());
    vmap.insert("z".into(), z.clone());

    //w == x
    let f1 = Rc::new(atom!(eq!(base!(var!(w)), base!(var!(x)))));
    let c1 = Intensional::new(vec![w.clone(),x.clone()], f1);
    // x == y
    let f2 = Rc::new(atom!(eq!(base!(var!(x)), base!(var!(y)))));
    let c2 = Intensional::new(vec![x.clone(),y.clone()], f2);
    //y === z
    let f3 = Rc::new(atom!(eq!(base!(var!(y)), base!(var!(z)))));
    let c3 = Intensional::new(vec![y.clone(),z.clone()], f3);
    // (w == z + 1) OR (w == z AND w == 3)
    let c4 = Intensional::new(vec![w.clone(), z.clone()], Rc::new(or!(
                        atom!(eq!(
                            base!(var!(w)),
                            add!(base!(var!(z)), base!(cst!(1)))
                        )),
                        and!(
                            atom!(eq!(
                                base!(var!(w)),
                                base!(var!(z))
                            )),
                            atom!(eq!(
                                base!(var!(w)),
                                base!(cst!(3))
                            ))
                        )
                )));

    let mut csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2), Rc::new(c3), Rc::new(c4)]);

    let mut consistency = Consistency::new(VariableOriented, AC1, NoMonitor);
    let vars = csp.vars().keys().cloned().collect();
    consistency.enforce_consistency(&mut csp, vars); //standalone
    //Note: step 5 (c_wz, x) is not fruitless since (w,0) has no supports for c_wz
    //Hyp: switched between w and x
    //Result: all domains reduced to {3}
    assert!(csp.vars().values().all(|v| v.dom().size() == 1));
    assert!(csp.vars().values().all(|v| v.dom().active_values() == vec![3]));
}

