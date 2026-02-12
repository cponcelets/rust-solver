/**************************************
            Factories
***************************************/

// VValue
#[macro_export] macro_rules! vvals {
    ($( $l:expr => $v:expr ),* $(,)?) => {
        vec![
            $( VValue { label: $l.to_string(), value: $v } ),*
        ]
    };
}

//Extensional domain
#[macro_export]
macro_rules! dom {
     ($($x:expr),+ $(,)?) => {
        SetDom::new(vec![$($x),+])
    };
}

//Variable with Domain
#[macro_export] macro_rules! var_dom {
    ($name:expr, $dom:expr) => {
        Rc::new(ExVar::new($name, $dom))
    };
}

//Formulas
#[macro_export]
macro_rules! atom {
    ($p:expr) => {
        Formula::atom($p)
    };
}

#[macro_export]
macro_rules! not {
    ($f:expr) => {
        Formula::not($f)
    };
}

#[macro_export]
macro_rules! and {
    ($($f:expr),+ $(,)?) => {
        Formula::and(vec![$($f),+])
    };
}

#[macro_export]
macro_rules! or {
    ($($f:expr),+ $(,)?) => {
        Formula::or(vec![$($f),+])
    };
}

//Predicate
#[macro_export]
macro_rules! eq {
    ($a:expr, $b:expr) => {
        Pred::eq($a, $b)
    };
}

#[macro_export]
macro_rules! neq {
    ($a:expr, $b:expr) => {
        Pred::neq($a, $b)
    };
}

#[macro_export]
macro_rules! lt {
    ($a:expr, $b:expr) => {
        Pred::lt($a, $b)
    };
}

#[macro_export]
macro_rules! le {
    ($a:expr, $b:expr) => {
        Pred::le($a, $b)
    };
}

//Expressions
#[macro_export]
macro_rules! cst {
    ($v:expr) => {
        Expr::cst($v)
    };
}

#[macro_export]
macro_rules! var {
    ($v:expr) => {
        Expr::var($v.clone())
    };
}

#[macro_export]
macro_rules! base {
    ($a:expr) => {
        AExpr::Base($a)
    };
}

#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {
        AExpr::add($a, $b)
    };
}

#[macro_export]
macro_rules! sub {
    ($a:expr, $b:expr) => {
        AExpr::sub($a, $b)
    };
}

#[macro_export]
macro_rules! mul {
    ($a:expr, $b:expr) => {
        AExpr::mul($a, $b)
    };
}

#[macro_export]
macro_rules! expr {
    ($v:expr) => {
        AExpr::Base(Expr::Const($v))
    };
    ($v:ident) => {
        AExpr::Base(Expr::Var($v.clone()))
    };
    ($a:tt + $b:tt) => {
        AExpr::Add(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ($a:tt - $b:tt) => {
        AExpr::Sub(Box::new(expr!($a)), Box::new(expr!($b)))
    };
    ($a:tt * $b:tt) => {
        AExpr::Mul(Box::new(expr!($a)), Box::new(expr!($b)))
    };
}

#[macro_export]
macro_rules! pred {
    ($a:tt == $b:tt) => {
        Pred::Eq(expr!($a), expr!($b))
    };
    ($a:tt != $b:tt) => {
        Pred::Neq(expr!($a), expr!($b))
    };
    ($a:tt < $b:tt) => {
        Pred::Lt(expr!($a), expr!($b))
    };
    ($a:tt <= $b:tt) => {
        Pred::Le(expr!($a), expr!($b))
    };
    ($a:tt > $b:tt) => {
        Pred::Gt(expr!($a), expr!($b))
    };
    ($a:tt >= $b:tt) => {
        Pred::Ge(expr!($a), expr!($b))
    };
}