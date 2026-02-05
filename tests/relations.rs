use rust_solver::csp::prelude::extvar::ExVar;
use std::rc::Rc;
use rust_solver::csp::constraint::traits::Constraint;
use rust_solver::csp::domain::traits::Domain;
use rust_solver::csp::prelude::setdom::SetDom;
use rust_solver::csp::prelude::intensional::{EqConstraint, LtConstraint, NeqConstraint};
use rust_solver::csp::prelude::vvalue::VValue;
use rust_solver::var;

#[test]
fn eq_constraint_rel() {
    let dom_int = SetDom::new(vec![1, 2, 3]);
    let x = var!(String::from("x"), Clone::clone(&dom_int));
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
    let dom_int = SetDom::new(vec![1, 2, 3]);
    let x = var!(String::from("x"), Clone::clone(&dom_int));
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
    let dom_int = SetDom::new(vec!["dg", "mg", "lg", "w"]);
    let x = var!(String::from("x"), Clone::clone(&dom_int));
    let y = var!(String::from("y"), dom_int);

    let c = NeqConstraint::new(x.clone(), y.clone());
    let rel = c.rel();

    println!("{:?}", rel);
    assert_eq!(rel.len(), 12); //[(dg,mg) .. (w,lg)]
    assert!(rel.contains(&vec![VValue { label: x.label().clone(), value: "mg" }, VValue { label: y.label().clone(), value: "w" }]));
    assert!(!rel.contains(&vec![VValue { label: x.label().clone(), value: "lg" }, VValue { label: y.label().clone(), value: "lg" }]));
}

#[test]
fn cartesian_respects_trailing() {
    let mut d = SetDom::new(vec![1,2]);
    d.remove_value(&2, 1);

    let x = var!("x".into(), Clone::clone(&d));
    let y = var!("y".into(), Clone::clone(&d));
    let c = EqConstraint::new(x, y);
    let rel = c.rel();

    assert_eq!(rel.len(), 1);
    assert_eq!(rel[0][0].value, 1);
}