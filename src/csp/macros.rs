/**************************************
            Factories
***************************************/

//vvalue
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

//Extensional variable
#[macro_export] macro_rules! var {
    ($name:expr, $dom:expr) => {
        Rc::new(ExVar::new($name, $dom))
    };
}