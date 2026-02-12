/**************************************
- Author: Clement Poncelet
- Desc:
    - ArithT, for OrdT with add, sub, mul trait
    - Arithmetic and simple expressions
- TODO: More operations.
***************************************/

/**************************************
            ArithT
***************************************/
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;
use crate::csp::ast::eval::Eval;
use crate::csp::domain::domain::OrdT;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::VValue;

pub trait ArithT: OrdT
+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
{}

impl<T> ArithT for T
where
    T:OrdT
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
{}

/**************************************
            Type Definition
***************************************/
//Base
#[derive(Clone, Debug)]
pub enum Expr<T: OrdT> {
    Const(T),
    Var(Rc<ExVar<T>>),
}

//Arith
#[derive(Clone, Debug)]
pub enum AExpr<T: ArithT> {
    Base(Expr<T>),
    Add(Box<AExpr<T>>, Box<AExpr<T>>),
    Sub(Box<AExpr<T>>, Box<AExpr<T>>),
    Mul(Box<AExpr<T>>, Box<AExpr<T>>),
}

/**************************************
            Constructors
***************************************/

impl<T:OrdT> Expr<T> {
    pub fn cst(v: T) -> Self { Expr::Const(v) }
    pub fn var(v: Rc<ExVar<T>>) -> Self { Expr::Var(v) }
}

impl<T:ArithT> AExpr<T> {
    pub fn cst(v: T) -> Self { AExpr::Base(Expr::Const(v)) }
    pub fn var(v: Rc<ExVar<T>>) -> Self { AExpr::Base(Expr::Var(v)) }
    pub fn add(a: AExpr<T>, b: AExpr<T>) -> Self { AExpr::Add(Box::new(a), Box::new(b)) }
    pub fn sub(a: AExpr<T>, b: AExpr<T>) -> Self { AExpr::Sub(Box::new(a), Box::new(b)) }
    pub fn mul(a: AExpr<T>, b: AExpr<T>) -> Self { AExpr::Mul(Box::new(a), Box::new(b)) }
}

/**************************************
            Display Trait
***************************************/

impl<T:OrdT> Display for Expr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T:ArithT> Display for AExpr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", self.print())
    }
}

/**************************************
            Eval Trait
***************************************/

impl<T: OrdT> Eval for Expr<T> {
    type Output = T;
    fn eval(&self, asn: &Vec<VValue<T>>) -> Option<T> {
        match self {
            Expr::Const(a)            => Some(a.clone()),
            Expr::Var(v)    => asn.iter()
                .find(|vv| vv.label == *v.label())
                .map(|vv| vv.value.clone()),
        }
    }
    fn print(&self) -> String {
        match self {
            Expr::Const(a)            => a.to_string(),
            Expr::Var(v)    => v.label().to_string(),
        }
    }

    fn collect_vars(&self, acc: &mut HashSet<Rc<ExVar<T>>>) -> () {
        collect_vars_expr(self, acc);
    }
}

impl<T: ArithT> Eval for AExpr<T> {
    type Output = T;
    fn eval(&self, asn: &Vec<VValue<T>>) -> Option<T> {
        match self {
            AExpr::Base(b) => b.eval(asn),
            AExpr::Add(a, b)
            => Some(a.eval(asn)? + b.eval(asn)?),
            AExpr::Sub(a, b)
            => Some(a.eval(asn)? - b.eval(asn)?),
            AExpr::Mul(a, b)
            => Some(a.eval(asn)? * b.eval(asn)?)
        }
    }
    fn print(&self) -> String {
        match self {
            AExpr::Base(b)  => b.print(),
            AExpr::Add(a, b)
            => a.print() + " + " + &*b.print(),
            AExpr::Sub(a, b)
            => a.print() + " - " + &*b.print(),
            AExpr::Mul(a, b)
            => a.print() + " * " + &*b.print(),
        }
    }

    fn collect_vars(&self, acc: &mut HashSet<Rc<ExVar<T>>>) -> () {
        collect_vars_arith(self, acc);
    }
}

/**************************************
           Utilities
***************************************/

fn collect_vars_expr<T: OrdT>(e: &Expr<T>, acc: &mut HashSet<Rc<ExVar<T>>>) {
    match e {
        Expr::Const(_) => {}
        Expr::Var(v) => {acc.insert(v.clone());}
    }
}

fn collect_vars_arith<T: ArithT>(e: &AExpr<T>, acc: &mut HashSet<Rc<ExVar<T>>>) {
    match e {
        AExpr::Base(b) => {collect_vars_expr(b,acc);}

        AExpr::Add(a,b)
        | AExpr::Sub(a,b)
        | AExpr::Mul(a,b)
        => {
            collect_vars_arith(a, acc);
            collect_vars_arith(b, acc);
        }
    }
}

/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::ast::expr::{Expr, AExpr, collect_vars_arith};
    use std::collections::HashSet;
    use std::rc::Rc;
    use crate::{add, base, cst, var, var_dom};
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::variable::extvar::ExVar;

    fn setup_vars() -> (Rc<ExVar<i32>>, Rc<ExVar<i32>>) {
        let dom = SetDom::new(vec![1, 2, 3, 4]);
        let w = Rc::new(ExVar::new("w".into(), dom.clone()));
        let z = Rc::new(ExVar::new("z".into(), dom));
        (w, z)
    }

    #[test]
    fn expr_building() {
        let (_, z) = setup_vars();
        let e = add!(base!(var!(z)), base!(cst!(1)));
        match e {
            AExpr::Add(_, _) => {}
            _ => panic!("Expected Add expression"),
        }
    }

    #[test]
    fn collect_scope_expr() {
        let dom = SetDom::new(vec![1, 2, 3]);
        let x = var_dom!(String::from("x"), dom.clone());
        let y = var_dom!(String::from("y"), dom.clone());

        let e = AExpr::Add(
            Box::new(base!(var!(x))),
            Box::new(base!(var!(y))),
        );

        let mut acc = HashSet::new();
        collect_vars_arith(&e, &mut acc);

        assert_eq!(acc.len(), 2);
        assert!(acc.contains(&x));
        assert!(acc.contains(&y));
    }

    #[test]
    fn exvar_hash_eq() {
        let dom = SetDom::new(vec![1, 2]);
        let x1 = Rc::new(ExVar::new("x".into(), dom.clone()));
        let x2 = Rc::new(ExVar::new("x".into(), dom.clone()));
        let y  = Rc::new(ExVar::new("y".into(), dom.clone()));

        let mut set = std::collections::HashSet::new();
        set.insert(x1.clone());
        set.insert(x2.clone()); // same label → should NOT increase size
        set.insert(y.clone());

        assert_eq!(set.len(), 2);
    }
}