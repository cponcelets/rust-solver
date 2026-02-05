use crate::extvar::ExVar;
use std::collections::HashMap;
use std::rc::Rc;
use rust_solver::csp::constraint::traits::Constraint;
use rust_solver::csp::csp::Csp;
use rust_solver::csp::prelude::*;
use rust_solver::csp::domain::setdom::{SetDom};
use rust_solver::csp::domain::traits::Domain;
use rust_solver::csp::prelude::intensional::{EqConstraint, LtConstraint, NeqConstraint};
use rust_solver::csp::prelude::vvalue::vv;
use rust_solver::var;

#[test]
fn test_lc_and_solution() { //Book's Example
    let dom012 = SetDom::new(vec![0, 1, 2]);

    let vmap = HashMap::from([
        (String::from("x"), Rc::new(ExVar::new(String::from("x"), dom012.snapshot()))),
        (String::from("y"), Rc::new(ExVar::new(String::from("y"), dom012.snapshot()))),
        (String::from("z"), Rc::new(ExVar::new(String::from("z"), dom012.snapshot())))
    ]);

    let p_init = Csp::new(vmap.clone(),
                                            {vec![
                                                Box::new(EqConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("y")).unwrap().clone())),
                                                Box::new(LtConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("y")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone()))
                                            ]}
    );

    //(x,1)             locally consistent
    //{(x,1),(y,0)}     not locally consistent (cover Cxy but not satisfied)
    let ixy = vec![vv(String::from("x"), 1), vv(String::from("y"), 0)];
    assert_eq!(p_init.is_locally_consistent(&ixy), Truth::False);
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixy), p_init.constraints()[0].is_support_asn_rel(&ixy));
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixy), Truth::False);
    //Then
    assert_eq!(p_init.is_solution(&ixy), Truth::False);

    let ixy_lc = vec![vv(String::from("x"), 1), vv(String::from("y"), 1)];
    assert_eq!(p_init.is_locally_consistent(&ixy_lc), Truth::True);
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixy_lc), p_init.constraints()[0].is_support_asn_rel(&ixy_lc));
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixy_lc), Truth::True);
    //Then
    assert_eq!(p_init.is_solution(&ixy_lc), Truth::False);

    let ixyz = vec![vv(String::from("x"), 1), vv(String::from("y"), 1), vv(String::from("z"), 0)];
    assert_eq!(p_init.is_locally_consistent(&ixyz), Truth::False);
    assert_eq!(p_init.constraints()[1].is_support_asn(&ixyz), p_init.constraints()[1].is_support_asn_rel(&ixyz));
    assert_eq!(p_init.constraints()[1].is_support_asn(&ixyz), Truth::False);
    //Then
    assert_eq!(p_init.is_solution(&ixyz), Truth::False);

    let ixyz_lc = vec![vv(String::from("x"), 1), vv(String::from("y"), 1), vv(String::from("z"), 2)];
    assert_eq!(p_init.is_locally_consistent(&ixyz_lc), Truth::True);
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixyz_lc), p_init.constraints()[0].is_support_asn_rel(&ixyz_lc));
    assert_eq!(p_init.constraints()[0].is_support_asn(&ixyz_lc), Truth::True);
    assert_eq!(p_init.constraints()[1].is_support_asn(&ixyz_lc), p_init.constraints()[1].is_support_asn_rel(&ixyz_lc));
    assert_eq!(p_init.constraints()[1].is_support_asn(&ixyz_lc), Truth::True);
    assert_eq!(p_init.constraints()[2].is_support_asn(&ixyz_lc), p_init.constraints()[2].is_support_asn_rel(&ixyz_lc));
    assert_eq!(p_init.constraints()[2].is_support_asn(&ixyz_lc), Truth::True);
    //Then
    assert_eq!(p_init.is_solution(&ixyz_lc), Truth::True);

    //still from the book
    //globally consistency

    //locally inconsistent
    assert_eq!(p_init.is_globally_consistent(&ixy), Truth::False);
    //can be extended to a solution
    assert_eq!(p_init.is_globally_consistent(&ixy_lc), Truth::True);
    //locally consistent but globally inconsistent
    let iyz_lc = vec![vv(String::from("y"), 2), vv(String::from("z"), 1)];
    assert_eq!(p_init.is_globally_consistent(&iyz_lc), Truth::False);
    let ix_lc = vec![vv(String::from("x"), 2)];
    assert_eq!(p_init.is_globally_consistent(&ix_lc), Truth::False);
}

fn small_csp() -> Csp<i32> {
    let dom = SetDom::new(vec![1,2]);

    let x = var!("x".into(), dom.snapshot());
    let y = var!("y".into(), dom.snapshot());
    let z = var!("z".into(), dom.snapshot());
    let mut vmap = HashMap::new();
    vmap.insert("x".into(), x.clone());
    vmap.insert("y".into(), y.clone());
    vmap.insert("z".into(), z.clone());

    let c1 = EqConstraint::new(x.clone(), y.clone()); // x = y
    let c2 = LtConstraint::new(y.clone(), z.clone()); // y < z

    Csp::new(vmap, vec![Box::new(c1), Box::new(c2)])
}

#[test]
fn csp_local_consistency() {
    let csp = small_csp();

    for c in csp.constraints() {
        for var in c.scp() {
            for v in var.dom().iter_on_active() {
                let vv = vv(var.label().clone(), v.clone());
                let t = c.is_support(&vv);
                assert_ne!(t, Truth::Unknown);
            }
        }
    }
}

#[test]
fn csp_is_normalized() {
    let csp = small_csp();
    assert!(csp.is_normalized());
}

#[test]
fn rel_matches_support() {
    let csp = small_csp();

    for c in csp.constraints() {
        let rel = c.rel();
        for tuple in rel {
            assert_eq!(c.check_assignment(&tuple), Truth::True);
        }
    }
}

#[test]
fn trailing_preserves_consistency() {
    let dom = SetDom::new(vec![1,2]);
    let x = var!("x".into(), dom.snapshot());
    let y = var!("y".into(), dom.snapshot());
    let c = EqConstraint::new(x.clone(), y.clone());

    assert_eq!(c.is_support(&vv("x".into(), 1)), Truth::True);

    x.dom_mut().remove_value(&1, 1);
    assert_eq!(c.is_support(&vv("x".into(), 1)), Truth::False);
}