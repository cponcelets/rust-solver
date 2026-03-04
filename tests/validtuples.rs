/**************************************
- Author: Clement Poncelet
- Desc: Test cases for checking valid tuples and invalid position features.
- Optimization:
    - Improve Eq and hash for variables (now on label())
***************************************/

use std::collections::HashMap;
use rust_solver::csp::prelude::vvalue::{vv_from_hashmap, VValue};
use rust_solver::csp::ast::expr::Expr;
use rust_solver::csp::ast::pred::Pred;
use rust_solver::csp::ast::formula::Formula;
use std::rc::Rc;
use rust_solver::csp::domain::setdom::SetDom;
use rust_solver::csp::prelude::extvar::ExVar;
use rust_solver::csp::constraint::intensional::Intensional;
use rust_solver::csp::constraint::extensional::ExtConstraint;
use rust_solver::csp::constraint::constraint::Constraint;
use rust_solver::csp::constraint::extensional::make_extensional_from;
use rust_solver::solver::consistency::cvalue::CValue;
use rust_solver::{atom, eq, var, var_dom, vvals};

#[test]
fn test_cvalue_deep_clone() {
    let dom = SetDom::new(vec!["a", "b", "c", "d"]);
    let x = var_dom!("x".into(), dom.snapshot());
    let y = var_dom!("y".into(), dom);

    let c = Rc::new(Intensional::from_formula(Rc::from(atom!(eq!(var!(x), var!(y))))));

    let cv = CValue {
        constraint: c.clone(),
        variable: x.clone(),
        value: "a",
    };

    let cloned = cv.deep_clone();

    // 1. Constraint must not be same Rc
    assert!(!Rc::ptr_eq(&cv.constraint, &cloned.constraint));

    // 2. Variable must not be same Rc
    assert!(!Rc::ptr_eq(&cv.variable, &cloned.variable));

    // 3. Value must be equal
    assert_eq!(cv.value, cloned.value);

    // 4. Cloned variable must belong to cloned constraint
    let scope = cloned.constraint.scp();
    assert!(scope.iter().any(|v| Rc::ptr_eq(v, &cloned.variable)));
}

#[test]
fn test_get_first_valid() {
    let dom_x = SetDom::new(vec![1, 4, 5]);
    let dom_y = SetDom::new(vec![2, 4]);
    let dom_z = SetDom::new(vec![1, 2]);

    let x = var_dom!("x".into(), dom_x);
    let y = var_dom!("y".into(), dom_y);
    let z = var_dom!("z".into(), dom_z);

    let c = ExtConstraint::new(
        vec![x.clone(), y.clone(), z.clone()],
        vec![
            vvals!("x" => 1, "y" => 2, "z" => 1),
            vvals!("x" => 1, "y" => 2, "z" => 2),
            vvals!("x" => 4, "y" => 2, "z" => 1),
            vvals!("x" => 4, "y" => 2, "z" => 2),
            vvals!("x" => 5, "y" => 2, "z" => 1),
            vvals!("x" => 5, "y" => 2, "z" => 2),
            vvals!("x" => 1, "y" => 4, "z" => 1),
            vvals!("x" => 1, "y" => 4, "z" => 2),
            vvals!("x" => 4, "y" => 4, "z" => 1),
            vvals!("x" => 4, "y" => 4, "z" => 2),
            vvals!("x" => 5, "y" => 4, "z" => 1),
            vvals!("x" => 5, "y" => 4, "z" => 2),
        ],
    );

    //c_(y=4) = {(1, 4, 1), (1, 4, 2), (4, 4, 1), (4, 4, 2), (5, 4, 1), (5, 4, 2)}
    let cval = &CValue { constraint: Rc::new(c), variable: y, value: 4 };
    assert_eq!(
        vv_from_hashmap(&cval.get_first_valid_tuple()),
        vvals!("x" => 1, "y" => 4, "z" => 1));
}

#[test]
fn test_get_next_valid() {
    let dom_x = SetDom::new(vec![1, 4, 5]);
    let dom_y = SetDom::new(vec![2, 4]);
    let dom_z = SetDom::new(vec![1, 2]);

    let x = var_dom!("x".into(), dom_x);
    let y = var_dom!("y".into(), dom_y);
    let z = var_dom!("z".into(), dom_z);

    let c = ExtConstraint::new(
        vec![x.clone(), y.clone(), z.clone()],
        vec![
            vvals!("x" => 1, "y" => 2, "z" => 1),
            vvals!("x" => 1, "y" => 2, "z" => 2),
            vvals!("x" => 4, "y" => 2, "z" => 1),
            vvals!("x" => 4, "y" => 2, "z" => 2),
            vvals!("x" => 5, "y" => 2, "z" => 1),
            vvals!("x" => 5, "y" => 2, "z" => 2),
            vvals!("x" => 1, "y" => 4, "z" => 1),
            vvals!("x" => 1, "y" => 4, "z" => 2),
            vvals!("x" => 4, "y" => 4, "z" => 1),
            vvals!("x" => 4, "y" => 4, "z" => 2),
            vvals!("x" => 5, "y" => 4, "z" => 1),
            vvals!("x" => 5, "y" => 4, "z" => 2),
        ],
    );

    //c_(y=4) = {(1, 4, 1), (1, 4, 2), (4, 4, 1), (4, 4, 2), (5, 4, 1), (5, 4, 2)}
    let cval = CValue { constraint: Rc::new(c), variable: y, value: 4 };
    let first = cval.get_first_valid_tuple();

    let second = cval.get_next_valid_tuple(&first)
        .expect("Error getting second value");
    assert_eq!(vv_from_hashmap(&second), vvals!("x" => 1, "y" => 4, "z" => 2));

    let third = cval.get_next_valid_tuple(&second)
        .expect("Error getting third value");
    assert_eq!(vv_from_hashmap(&third), vvals!("x" => 4, "y" => 4, "z" => 1));

    let last = HashMap::from([
        (String::from("x"), 5),
        (String::from("y"), 4),
        (String::from("z"), 2)]);
    assert_eq!(cval.get_next_valid_tuple(&last), None);
}

#[test]
fn test_get_invalid_position() {
    let dom_v = SetDom::new(vec![1, 3]);
    let dom_w = SetDom::new(vec![3, 4]);
    let dom_x = SetDom::new(vec![1, 4, 5]);
    let dom_y = SetDom::new(vec![2, 4]);
    let dom_z = SetDom::new(vec![1, 2]);

    let y = var_dom!("y".into(), dom_y);

    let c = make_extensional_from(&vec![
        var_dom!("v".into(), dom_v),
        var_dom!("w".into(), dom_w),
        var_dom!("x".into(), dom_x),
        y.clone(),
        var_dom!("z".into(), dom_z)]);

    let tuple1 = HashMap::from([
        (String::from("v"), 3),
        (String::from("w"), 4),
        (String::from("x"), 4),
        (String::from("y"), 2),
        (String::from("z"), 2)]);
    assert_eq!(c.get_first_invalid_pos(Some(&tuple1)), -1);

    let tuple2 = HashMap::from([
        (String::from("v"), 3),
        (String::from("w"), 4),
        (String::from("x"), 6),
        (String::from("y"), 2),
        (String::from("z"), 2)]);
    assert_eq!(c.get_first_invalid_pos(Some(&tuple2)), 3);

    let cval = CValue { constraint: c.clone(), variable: y, value: 2 };
    assert_eq!(cval.get_next_valid_tuple_limit(&tuple2, 3), None);

    let tuple3 = HashMap::from([
        (String::from("v"), 3),
        (String::from("w"), 3),
        (String::from("x"), 6),
        (String::from("y"), 2),
        (String::from("z"), 3)]);
    assert_eq!(c.get_first_invalid_pos(Some(&tuple3)), 3);

    assert_eq!(vv_from_hashmap(&cval.get_next_valid_tuple_limit(&tuple3, 3).expect("Should not be None!")),
               vvals!("v" => 3, "w" => 4, "x" => 1, "y" => 2, "z" => 1));

}