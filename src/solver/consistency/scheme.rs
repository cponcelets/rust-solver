/**************************************
- Author: Clement Poncelet
- Desc: Propagation scheme
        - arc oriented (Algorithm 7:gac_enforce_arc)
        - variable oriented (Algorithm 9:gac_enforce_var)
  Contains unit tests for arc oriented consistency enforcement (var oriented tests are into test/var.rs)
- Optimization:
    - output Option<TriggerEvent>
***************************************/
use std::collections::HashMap;
use std::rc::Rc;
use crate::csp::csp::Csp;
use crate::csp::prelude::domain::{Domain, OrdT};
use crate::csp::prelude::extvar::ExVar;
use crate::instrumentation::monitor::Monitor;
use crate::solver::consistency::arc::{Arc};
use crate::solver::consistency::consistency::Revise;

pub trait Scheme <M: Monitor, T: OrdT, R: Revise<M, T>> {
    fn enforce(&mut self, csp: &mut Csp<T>, events: Vec<String>, revise: &mut R, monitor: &mut M) -> bool;
}

pub struct ArcOriented;

impl<M: Monitor, T: OrdT, R: Revise<M, T>> Scheme<M, T, R> for ArcOriented {
    fn enforce(&mut self, csp :&mut Csp<T>, events: Vec<String>, revise: &mut R, monitor: &mut M) -> bool {
        monitor.on_enforce_start();
        // return next arc
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
                            monitor.on_enqueue();
                            queue.push(Arc { constraint: c.clone(), variable: x.clone() })
                        }
                    }
                }
            }
        }

        //Propagation
        let mut step = 1;
        while queue.len() > 0 {
            println!("{}", format!("Step {} Q {}", step, queue.iter().map(|ar| ar.to_string()).collect::<Vec<_>>().join("")));
            monitor.on_dequeue();
            let arc_cx = queue.remove(0);
            println!("Pick {} from Q", arc_cx);
            if revise.revise(&arc_cx, csp.level(), monitor) {
                if arc_cx.variable.dom().is_empty() {
                    monitor.on_domain_wipeout();
                    monitor.on_enforce_end();
                    monitor.on_domain_snapshot(csp);
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
                                monitor.on_enqueue();
                                //x' != x && x' not in past
                                queue.push(Arc { constraint: c.clone(), variable: x.clone() })
                            }
                        }
                    }
                }
            } else {
                monitor.on_revise_fruitless();
                println!("Fruitless");
            }
            step +=1;
        }
        monitor.on_enforce_end();
        monitor.on_domain_snapshot(csp);
        true
    }
}

pub struct VariableOriented;
impl<M: Monitor, T: OrdT, R: Revise<M, T>> Scheme<M, T, R> for VariableOriented {
    fn enforce(&mut self, csp: &mut Csp<T>, events: Vec<String>, revise: &mut R, monitor: &mut M) -> bool {
        monitor.on_enforce_start();

        let mut queue: Vec<Rc<ExVar<T>>> = vec![];
        let mut stamp_var: HashMap<String, usize> = HashMap::new();
        let mut stamp_ctr: HashMap<String, usize> = HashMap::new();
        let mut time = 0;

        //lexicographical sort
        let mut vars = csp.vars().values().cloned().collect::<Vec<_>>();
        vars.sort_by(|a, b| a.label().cmp(b.label()));

        //insert: algorithm 10
        for v in vars {
            if events.contains(&v.label()) {
                insert(&mut queue, &mut stamp_var, v, &mut time, monitor);
            }
        }

        let mut step = 1;
        while queue.len() > 0 {
            println!("{}", format!("Step {} Q {}", step, queue.iter().map(|v| v.label().clone()).collect::<Vec<_>>().join(",")));
            monitor.on_dequeue();
            let x = queue.remove(0);
            println!("Pick {} from Q", x);

            for c in csp.constraints() {
                if c.scp().contains(&x) && stamp_var.get(x.label()) > stamp_ctr.get(&c.label()) {
                    for y in c.scp() {
                        if !csp.past().contains(y.label()) {
                            if x != *y
                                || c.scp().iter()
                                .any(|z| *z != x && stamp_var.get(z.label()) > stamp_ctr.get(&c.label())) {
                                println!("Revise <{},{}>", c.label(), y.label());
                                if revise.revise(&Arc { constraint: c.clone(), variable: y.clone() }, csp.level(), monitor) {
                                    if y.dom().is_empty() {
                                        monitor.on_domain_wipeout();
                                        monitor.on_enforce_end();
                                        monitor.on_domain_snapshot(csp);
                                        return false; //raise dom_wipeout
                                    }
                                    insert(&mut queue, &mut stamp_var, y.clone(), &mut time, monitor);
                                } else {
                                    println!("Fruitless");
                                    monitor.on_revise_fruitless();
                                }
                            }
                        }
                    }
                }
                time += 1;
                stamp_ctr.entry(c.label()).or_insert(time);
            }
            step +=1;
        }
        monitor.on_enforce_end();
        monitor.on_domain_snapshot(csp);
        true
    }
}

//helper to insert values into queue
fn insert<M:Monitor, T: OrdT>(queue : &mut Vec<Rc<ExVar<T>>>, stamp_var: &mut HashMap<String, usize>, v : Rc<ExVar<T>>,  time : &mut usize, monitor: &mut M) {
    monitor.on_enqueue();
    queue.push(v.clone());
    *time +=1;
    if let Some (t) =stamp_var.get_mut(&v.label().clone()) {
        *t = *time
    } else {
        stamp_var.insert(v.label().clone(), *time);
    }
}

/**************************************
        Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::ast::expr::{Expr,AExpr};
    use crate::csp::ast::pred::Pred;
    use crate::csp::ast::formula::Formula;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::domain::domain::{Domain, OrdT};
    use crate::csp::variable::extvar::ExVar;
    use crate::{add, and, atom, base, cst, eq, lt, or, var, var_dom};
    use crate::csp::constraint::intensional::Intensional;
    use crate::csp::csp::Csp;
    use crate::instrumentation::monitor::{NoMonitor, Statistics};
    use crate::solver::consistency::revise::AC1;
    use crate::solver::consistency::scheme::{ArcOriented, Scheme};

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
    fn stand_alone_arc() {
        let mut csp = setup_csp::<i32>();
        let vars = csp.0.vars().keys().cloned().collect();
        assert!(ArcOriented.enforce(&mut csp.0, vars, &mut AC1, &mut NoMonitor));
    }

    #[test]
    fn domino_example_arc() {
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

        let mut csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2), Rc::new(c3), Rc::new(c4)]);
        let mut arc_scheme = ArcOriented;
        let vars = csp.vars().keys().cloned().collect();
        let mut stats = Statistics::default();
        arc_scheme.enforce(&mut csp, vars, &mut AC1, &mut stats); //standalone
        //Note: step 7 (c_wz, w) is not fruitless since (w,0) has no supports for c_wz
        //Hyp: switched between w and z (z,0) actually works since w = 0 + 1 is fine
        //Result: all domains reduced to {3}
        assert!(csp.vars().values().all(|v| v.dom().size() == 1));
        assert!(csp.vars().values().all(|v| v.dom().active_values() == vec![3]));

        let avg = stats.total_enforce_time / stats.enforce_calls as u32;
        println!("Lasting {} millisecond(s) in average", avg.as_millis());
        println!("{:#?}", stats.domain_histogram);
    }
}