use std::fmt;
use std::rc::Rc;
use crate::csp::constraint::constraint::Constraint;
use crate::csp::csp::Csp;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::prelude::extvar::ExVar;
use crate::csp::variable::vvalue::vv;
/**************************************
            arc
***************************************/

pub struct Arc<T:OrdT> {
    pub constraint: Rc<dyn Constraint<T>>,
    pub variable: Rc<ExVar<T>>
}

impl<T:OrdT> fmt::Display for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.constraint.label(), self.variable.label())
    }
}

impl<T:OrdT> fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Arc")
            .field("constraint", &self.constraint)
            .field("variable", &self.variable)
            .finish()
    }
}

fn enforce_gac_arc<T:OrdT>(csp :&Csp<T>, events: Vec<&String>) -> bool {
    let mut queue: Vec<Arc<T>> = vec![];

    //add a type arc ? <c,v>
    //for each arc
    //   not in past(P)
    //   where y in scp(c) inter events and <> x
    for c in csp.constraints() {
        for x in c.scp() {
            //arcs (c,v)
            if !csp.past().contains(x.label()) {
                //not in past
                for y in c.scp() {
                    if y.label() != x.label() && events.contains(&y.label()) {
                        queue.push(Arc { constraint: c.clone(), variable: x.clone() })
                    }
                }
            }
        }
    }
    let mut step = 1;
    while queue.len() > 0 {
        println!("{}", format!("Step {} Q {}", step, queue.iter().map(|ar| ar.to_string()).collect::<Vec<_>>().join("")));
        let arc_cx = queue.remove(0);
        println!("Pick {} from Q", arc_cx);
        if revise(&arc_cx, csp.level()) {
            if arc_cx.variable.dom().is_empty() {
                return false; //raise dom_wipeout
            }
            for c in csp.constraints() {
                //assumption: normalized csp
                if  c.to_string() != arc_cx.constraint.to_string() &&
                    c.scp().iter().any(|v| v.label() == arc_cx.variable.label()) {
                    //c' != c && x in Scp(c')
                    for x in c.scp() {
                        if  x.label() != arc_cx.variable.label() &&
                            !csp.past().contains(x.label()) {
                            //x' != x && x' not in past
                            queue.push(Arc { constraint: c.clone(), variable: x.clone() })
                        }
                    }
                }
            }
        } else {
            println!("Frutless");
        }
        step +=1;
    }
    true
}

//true if revision (c,x) effective
fn revise<T:OrdT>(arc : &Arc<T>, level: usize) -> bool{
    let size_before = arc.variable.dom().size();
    for a in arc.variable.valid_values() {
        if !seek_support(arc.constraint.clone(), arc.variable.clone(), &a) {
            println!("remove {} from {}", a, arc.variable);
            arc.variable.dom_mut().remove_value(&a, level);
        }
    }
    size_before != arc.variable.dom().size()
}

fn seek_support<T:OrdT>(c: Rc<dyn Constraint<T>>, x: Rc<ExVar<T>>, a: &T) -> bool {
    c.is_support_asn(&vec![vv(x.label().clone(), a.clone())], false).to_bool().unwrap()
}

#[cfg(test)]
    mod tests {
    use crate::csp::ast::expr::{Expr,AExpr};
    use crate::csp::ast::pred::Pred;
    use crate::csp::ast::formula::Formula;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::domain::domain::{OrdT};
    use crate::csp::variable::extvar::ExVar;
    use crate::{add, and, atom, base, cst, eq, lt, or, var, var_dom};
    use crate::csp::constraint::intensional::Intensional;
    use crate::csp::csp::Csp;
    use crate::solver::gac::arc::enforce_gac_arc;

    fn setup_csp<'a, T: OrdT>() -> (Csp<i32>, Rc<ExVar<i32>>, Rc<ExVar<i32>>, Rc<ExVar<i32>>) {
            let dom = SetDom::new(vec![1, 2]);

            let x = var_dom!("x".into(), dom.snapshot());
            let y = var_dom!("y".into(), dom.snapshot());
            let z = var_dom!("z".into(), dom);
            let mut vmap = HashMap::new();
            vmap.insert("x".into(), x.clone());
            vmap.insert("y".into(), y.clone());
            vmap.insert("z".into(), z.clone());

            // x == y
            let f1 = Rc::new(atom!(eq!(var!(x), var!(y))));
            let c1 = Intensional::new(vec![x.clone(),y.clone()], f1);
            // y < z
            let f2 = Rc::new(atom!(lt!(var!(y), var!(z))));
            let c2 = Intensional::new(vec![y.clone(),z.clone()], f2);
            // x < z
            let f3 = Rc::new(atom!(lt!(var!(x), var!(z))));
            let c3 = Intensional::new(vec![x.clone(),z.clone()], f3);

            let csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2), Rc::new(c3)]);
            (csp, x, y, z)
        }

    #[test]
    fn stand_alone() {
        let csp = setup_csp::<i32>();
        assert!(enforce_gac_arc(&csp.0, csp.0.vars().keys().collect()));
    }

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
        let f1 = Rc::new(atom!(eq!(var!(w), var!(x))));
        let c1 = Intensional::new(vec![w.clone(),x.clone()], f1);
        // x == y
        let f2 = Rc::new(atom!(eq!(var!(x), var!(y))));
        let c2 = Intensional::new(vec![x.clone(),y.clone()], f2);
        //y === z
        let f3 = Rc::new(atom!(eq!(var!(y), var!(z))));
        let c3 = Intensional::new(vec![y.clone(),z.clone()], f3);
        // (w == z + 1) OR (w == z AND w == 3)
        let c4 = Intensional::new(vec![w.clone(), z.clone()],
            Rc::new(or!(
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

        let csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2), Rc::new(c3), Rc::new(c4)]);
        enforce_gac_arc(&csp, csp.vars().keys().collect()); //standalone
        //Note: step 7 (c_wz, w) is not fruitless since (w,0) has no supports for c_wz
        //Hyp: switched betwwen w and z (z,0) actually works since w = 0 + 1 is fine
    }
}