use rust_solver::csp::prelude::extvar::ExVar;
use std::rc::Rc;
use rust_solver::csp::constraint::traits::Constraint;
use rust_solver::csp::prelude::extdom::ExDom;
use rust_solver::csp::prelude::intensional::{EqConstraint, LtConstraint, NeqConstraint};
use rust_solver::csp::prelude::vvalue::VValue;
use rust_solver::var;

#[test]
fn eq_constraint_rel() {
    let dom_int = ExDom::new(vec![1, 2, 3]);
    let x = var!(String::from("x"), dom_int.clone());
    let y = var!(String::from("y"), dom_int);

    let c = EqConstraint::new(x.clone(), y.clone());
    let rel = c.rel();

    //println!("{:?}", rel);
    assert_eq!(rel.len(), 3); //[(1,1),(2,2),(3,3)]
    assert!(rel.contains(&vec![VValue { label: x.label().clone(), value: 1 }, VValue { label: y.label().clone(), value: 1 }]));
    assert!(!rel.contains(&vec![VValue { label: x.label().clone(), value: 1 }, VValue { label: y.label().clone(), value: 2 }]));
}

#[test]
fn lt_constraint_rel() {
    let dom_int = ExDom::new(vec![1, 2, 3]);
    let x = var!(String::from("x"), dom_int.clone());
    let y = var!(String::from("y"), dom_int);

    let c = LtConstraint::new(x.clone(), y.clone());
    let rel = c.rel();

    //println!("{:?}", rel);
    assert_eq!(rel.len(), 3); //[(1,2),(1,3),(2,3)]
    assert!(rel.contains(&vec![VValue { label: x.label().clone(), value: 1 }, VValue { label: y.label().clone(), value: 3 }]));
    assert!(!rel.contains(&vec![VValue { label: x.label().clone(), value: 2 }, VValue { label: y.label().clone(), value: 2 }]));
}

#[test]
fn neq_constraint_rel() {
    let dom_int = ExDom::new(vec!["dg", "mg", "lg", "w"]);
    let x = var!(String::from("x"), dom_int.clone());
    let y = var!(String::from("y"), dom_int);

    let c = NeqConstraint::new(x.clone(), y.clone());
    let rel = c.rel();

    println!("{:?}", rel);
    assert_eq!(rel.len(), 12); //[(dg,mg) .. (w,lg)]
    assert!(rel.contains(&vec![VValue { label: x.label().clone(), value: "mg" }, VValue { label: y.label().clone(), value: "w" }]));
    assert!(!rel.contains(&vec![VValue { label: x.label().clone(), value: "lg" }, VValue { label: y.label().clone(), value: "lg" }]));
}