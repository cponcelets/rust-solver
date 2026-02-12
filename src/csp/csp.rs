use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use petgraph::graph::UnGraph;
use statrs::function::factorial::binomial;
use crate::csp::constraint::constraint::Constraint;
use crate::csp::domain::setdom::CartesianWalker;
use crate::csp::domain::domain::{Domain, OrdT};
use crate::csp::truth::Truth;
use crate::csp::variable::extvar::ExVar;
use crate::csp::variable::vvalue::{vv, VValue};

pub struct Csp<T:OrdT> {
    vars : HashMap<String, Rc<ExVar<T>>>,
    constraints : Vec<Rc<dyn Constraint<T>>>,
    // -- for consistencies --
    past : Vec<String>, //instantiated variables
}

impl<T:OrdT> Csp<T> {
    pub fn new (v: HashMap<String, Rc<ExVar<T>>>, c: Vec<Rc<dyn Constraint<T>>>) -> Csp<T> {
        Csp { vars: v, constraints: c, past : Vec::new() }
    }

    pub fn constraints(&self) -> &Vec<Rc<dyn Constraint<T>>> {&self.constraints}
    pub fn vars(&self) -> &HashMap<String, Rc<ExVar<T>>> {&self.vars}

    pub fn cover(&self, asn: &Vec<VValue<T>>) -> Vec<&dyn Constraint<T>> {
        self.constraints
            .iter()
            .map(|c| c.as_ref())
            .filter(|c| c.is_covered(asn))
            .collect()
    }

    pub fn is_locally_consistent(&self, asn: &Vec<VValue<T>>) -> Truth {
        let mut result = Truth::True;
        for c in self.cover(asn) {
            match c.check_assignment(asn) {
                Truth::False => return Truth::False,
                Truth::Unknown => result = Truth::Unknown,
                Truth::True => {}
            }
        }
        result
    }

    pub fn is_globally_consistent(&self, asn: &Vec<VValue<T>>) -> Truth {
        if self.is_locally_consistent(asn).to_bool().unwrap() {
            if self.is_solution(asn).to_bool().unwrap() {
                Truth::True
            }  else {
                let assigned_labels: std::collections::HashSet<_> =
                    asn.iter().map(|v| &v.label).collect();
                let var_to_extends: Vec<_> = self.vars.values()
                    .filter(|v| !assigned_labels.contains(v.label()))
                    .cloned()
                    .collect();

                Truth::from(exists_extension(asn, &var_to_extends, |asn| {
                    self.constraints.iter()
                        .all(|c| c.check_assignment(asn) != Truth::False)
                }))
            }
        } else { Truth::False }
    }

    //An assignment is a solution if it covers all constraints and is locally consistent
    pub fn is_solution(&self, asn: &Vec<VValue<T>>) -> Truth {
        Truth::from(
            self.constraints.iter()
                .all(|c|c.is_covered(asn))
        ) & self.is_locally_consistent(asn)
    }

    //The number of variables (n = |vars(P)|)
    pub fn n(&self) -> usize {self.vars.keys().len()}

    //The number of constraints (e = |cons(P)|)
    pub fn e(&self) -> usize {self.constraints.len()}

    //The greatest domain size (d = max_{x ∈ vars(P)}, |dom(x)|)
    pub fn d(&self) -> usize {self.vars.values().map(|v| v.valid_size()).max().unwrap_or(0)}

    //The greatest constraint arity (r = max_{c∈cons(P)} | scp(c)|)
    pub fn r(&self) -> usize {self.constraints.iter().map(|c| c.scp().len()).max().unwrap_or(0)}

    pub fn is_normalized(&self) -> bool {
        //for all c if scp(ci) == scp(cj) -> ci == cj
        let mut seen = HashSet::new();
        for c in &self.constraints {
            let key = scope_key(c.as_ref());

            if !seen.insert(key) {
                // scope already seen → not normalized
                return false;
            }
        }
        true
    }

    fn density(&self) -> f64 {
        let bin_n = binomial(self.n() as u64, self.r() as u64);
        self.e() as f64 / bin_n
    }

    pub fn primal_graph(&self) -> UnGraph<String, String> {
        let mut g: UnGraph<String, String> = UnGraph::new_undirected();
        let mut imap = HashMap::new();
        for v in self.vars.keys() {
            imap.insert(v.clone(),g.add_node(v.clone()));
        }
        for c in &self.constraints {
            let op = c.scp();
            g.add_edge(
                imap.get(op.get(0).unwrap().label()).unwrap().clone(),
                imap.get(op.get(1).unwrap().label()).unwrap().clone(),
                c.to_string());
        }
        g
    }

    pub fn dual_graph(&self) -> UnGraph<String, String> {
        let mut g: UnGraph<String, String> = UnGraph::new_undirected();
        let mut cmap = HashMap::new();

        for (i, c) in self.constraints.iter().enumerate() {
            let idx = g.add_node(c.to_string());
            cmap.insert(i, idx);
        }

        for i in 0..self.constraints.len() {
            let ops_i = self.constraints[i].scp();

            for j in (i + 1)..self.constraints.len() {
                let ops_j = self.constraints[j].scp();

                // Find shared variables
                let shared: Vec<String> = ops_i
                    .iter()
                    .filter(|vi|
                        ops_j.iter().any(|vj| Rc::ptr_eq(vi, vj))
                    )
                    .map(|v| v.label().to_string())
                    .collect();

                if !shared.is_empty() {
                    let label = shared.join(", ");
                    g.add_edge(
                        cmap[&i],
                        cmap[&j],
                        label,
                    );
                }
            }
        }
        g
    }

    pub fn micro_structure(&self) -> UnGraph<String, String> {
        let mut g: UnGraph<String, String> = UnGraph::new_undirected();
        let mut vvmap = HashMap::new();

        for v in self.vars.values() {
            for a in v.valid_values() {
                let vval : VValue<T> = VValue {
                    label: v.label().clone(),
                    value: a.clone(),
                };
                let idx = g.add_node(vval.to_string());
                vvmap.insert(vval, idx);
            }
        }

        for c in &self.constraints {
            let rel = c.rel();
            for tuple in rel {
                for i in 0..tuple.len() {
                    for j in i + 1..tuple.len() {
                        let vi = &tuple[i];
                        let vj = &tuple[j];

                        let ni = vvmap[vi];
                        let nj = vvmap[vj];

                        g.add_edge(ni, nj, c.to_string());
                    }
                }
            }
        }
        g
    }

    //--- --- ------ --- ------ --- ------ --- ---
    pub fn past(&self) -> &Vec<String> {&self.past}
    pub fn level(&self) -> usize {self.past.len()}

}

impl<'a, T:OrdT> fmt::Display for Csp<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "csp {{")?;
        writeln!(f, "   variable {{")?;
        for (_, v) in self.vars.iter() {
            writeln!(f, "        {}", v)?;
        }
        writeln!(f, "   }}")?;
        writeln!(f, "   constraint {{")?;
        for c in &self.constraints {
            writeln!(f, "      {}", c)?;
        }
        writeln!(f, "   }}")?;
        write!(f, "}}")
    }
}

fn scope_key<T:OrdT>(c: &dyn Constraint<T>) -> Vec<String> {
    let mut key: Vec<String> =
        c.scp().iter().map(|v| v.label().clone()).collect();
    key.sort();
    key
}

pub fn exists_extension<T: OrdT>(asn: &[VValue<T>], missing_vars: &Vec<Rc<ExVar<T>>>, constraint: impl Fn(&Vec<VValue<T>>) -> bool) -> bool {
    let missing_doms: Vec<_> = missing_vars.iter().map(|v| v.dom().active_values()).collect();
    let walker = CartesianWalker::new(missing_doms);

    for tuple in walker {
        let mut asn_t = asn.to_vec();
        for (var, value) in missing_vars.iter().zip(tuple.into_iter()) {
            asn_t.push(vv(var.label().clone(), value));
        }
        if constraint(&asn_t) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
use crate::csp::ast::expr::Expr;
use crate::csp::ast::pred::Pred;
use crate::csp::ast::formula::Formula;
use std::collections::HashMap;
    use std::rc::Rc;
    use petgraph::dot::Dot;
    use crate::{atom, dom, eq, lt, neq, var, var_dom};
    use crate::csp::constraint::intensional::Intensional;
    use crate::csp::csp::Csp;
    use crate::csp::domain::setdom::SetDom;
    use crate::csp::domain::domain::OrdT;
    use crate::csp::variable::extvar::{generate_variables, ExVar};
    use crate::csp::variable::vvalue::{vv, VValue};

    fn setup_csp<'a, T: OrdT>() -> (Csp<i32>, Rc<ExVar<i32>>, Rc<ExVar<i32>>, Rc<ExVar<i32>>) {
        let dom = SetDom::new(vec![1, 2]);

        let x = var_dom!("x".into(), dom.clone());
        let y = var_dom!("y".into(), dom.clone());
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

        let csp = Csp::new(vmap,vec![Rc::new(c1), Rc::new(c2), Rc::new(c3)]);
        (csp, x, y, z)
    }

    #[test]
    fn color_constraints() {
        let dom_color = SetDom::new(vec!["dg", "mg", "lg", "w"]);
        let vmap = generate_variables("x", 9, &dom_color);
        let p_init = Csp::new(vmap.clone(),
                                   {vec![
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x1")).unwrap().clone()), var!(vmap.get(&String::from("x3")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x1")).unwrap().clone()), var!(vmap.get(&String::from("x4")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x1")).unwrap().clone()), var!(vmap.get(&String::from("x7")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x1")).unwrap().clone()), var!(vmap.get(&String::from("x2")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x2")).unwrap().clone()), var!(vmap.get(&String::from("x7")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x2")).unwrap().clone()), var!(vmap.get(&String::from("x8")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x2")).unwrap().clone()), var!(vmap.get(&String::from("x9")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x3")).unwrap().clone()), var!(vmap.get(&String::from("x4")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x3")).unwrap().clone()), var!(vmap.get(&String::from("x5")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x3")).unwrap().clone()), var!(vmap.get(&String::from("x6")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x4")).unwrap().clone()), var!(vmap.get(&String::from("x5")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x4")).unwrap().clone()), var!(vmap.get(&String::from("x7")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x5")).unwrap().clone()), var!(vmap.get(&String::from("x6")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x5")).unwrap().clone()), var!(vmap.get(&String::from("x7")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x5")).unwrap().clone()), var!(vmap.get(&String::from("x8")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x6")).unwrap().clone()), var!(vmap.get(&String::from("x8")).unwrap().clone())))),
                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x6")).unwrap().clone()), var!(vmap.get(&String::from("x9")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x7")).unwrap().clone()), var!(vmap.get(&String::from("x8")).unwrap().clone())))),

                                       Rc::new(Intensional::from_pred(neq!(var!(vmap.get(&String::from("x8")).unwrap().clone()), var!(vmap.get(&String::from("x9")).unwrap().clone())))),
                                   ]}
        );

        //For a given constraint network $P$ , we denote by:
        // - $n$ the number of variables, $n = | vars(P )|$;
        // - $e$ the number of constraints, $e = | cons(P )|$;
        // - $d$ the **greatest domain size**, $d = max_{x∈vars(P)} \, | \, dom(x)|$;
        // - $r$ the **greatest constraint arity**, $r = max_{c∈cons(P)} | scp(c)|$.
        println!("{}", p_init);
        println!("n: {}, e: {}, d: {}, r: {}.", p_init.n(), p_init.e(), p_init.d(), p_init.r());

        //constraint hypergraph (macro-structure)
        //primal graph
        let primal_graph = p_init.primal_graph();
        let primal_dot = Dot::new(&primal_graph);
        println!("Primal graph in DOT format:\n{:?}\n", primal_dot);
    }

    #[test]
    fn normalized_csp() {
        let dom = dom![1, 2];

        let x = var_dom!("x".into(), dom.clone());
        let y = var_dom!("y".into(), dom.clone());
        let z = var_dom!("z".into(), dom);
        let mut vmap = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());
        vmap.insert("z".into(), z.clone());

        let c1 = Intensional::from_pred(eq!(var!(x), var!(y)));
        let c2 = Intensional::from_pred(lt!(var!(x), var!(y)));

        let csp = Csp::new(vmap,vec![Rc::new(c1), Rc::new(c2)]);
        assert!(!csp.is_normalized());
    }

    #[test]
    fn non_normalized_same_scope() {
        let dom = dom![1, 2];

        let x = var_dom!("x".into(), dom.clone());
        let y = var_dom!("y".into(), dom);
        let mut vmap = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());

        let c1 = Intensional::from_pred(eq!(var!(x), var!(y)));
        let c2 = Intensional::from_pred(neq!(var!(x), var!(y)));

        let csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2)]);
        assert!(!csp.is_normalized());
    }

    #[test]
    fn non_normalized_scope_order_irrelevant() {
        let dom = dom![1, 2];

        let x = var_dom!("x".into(), dom.clone());
        let y = var_dom!("y".into(), dom);
        let mut vmap = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());

        let c1 = Intensional::from_pred(eq!(var!(x), var!(y)));
        let c2 = Intensional::from_pred(lt!(var!(y), var!(x)));

        let csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2)]);
        assert!(!csp.is_normalized());
    }

    #[test]
    fn hypergraph_density_binary() {
        let dom = dom![1, 2];

        let x = var_dom!(String::from("x"), dom.clone());
        let y = var_dom!(String::from("y"), dom.clone());
        let z = var_dom!(String::from("z"), dom);
        let mut vmap = HashMap::new();
        vmap.insert("x".into(), x.clone());
        vmap.insert("y".into(), y.clone());
        vmap.insert("z".into(), z.clone());

        let c1 = Intensional::from_pred(eq!(var!(x), var!(y)));
        let c2 = Intensional::from_pred(neq!(var!(y), var!(z)));

        let csp = Csp::new(vmap, vec![Rc::new(c1), Rc::new(c2)]);
        let d = csp.density();

        let expected = 2.0 / 3.0;
        assert!((d - expected).abs() < 1e-9);
    }

    #[test]
    fn cover_empty_assignment() {
        let (csp, _, _, _) = setup_csp::<i32>();
        let asn: Vec<VValue<i32>> = vec![];

        let covered = csp.cover(&asn);
        assert_eq!(covered.len(), 0);
    }

    #[test]
    fn cover_partial_assignment() {
        let (csp, x, _, _) = setup_csp::<i32>();
        let asn = vec![vv(x.label().clone(), 1)];
        let covered = csp.cover(&asn);

        assert_eq!(covered.len(), 0); // no constraint fully covered
    }

    #[test]
    fn cover_xy_assignment() {
        let (csp, x, y, _) = setup_csp::<i32>();
        let asn = vec![
            vv(x.label().clone(), 1),
            vv(y.label().clone(), 2),
        ];
        let covered = csp.cover(&asn);

        assert_eq!(covered.len(), 1);
        assert_eq!(covered[0].scp().len(), 2);
    }

    #[test]
    fn cover_yz_assignment() {
        let (csp, _, y, z) = setup_csp::<i32>();
        let asn = vec![
            vv(y.label().clone(), 1),
            vv(z.label().clone(), 2),
        ];
        let covered = csp.cover(&asn);

        assert_eq!(covered.len(), 1);
    }

    #[test]
    fn cover_full_assignment() {
        let (csp, x, y, z) = setup_csp::<i32>();
        let asn = vec![
            vv(x.label().clone(), 1),
            vv(y.label().clone(), 1),
            vv(z.label().clone(), 2),
        ];
        let covered = csp.cover(&asn);

        assert_eq!(covered.len(), 3);
    }

    #[test]
    fn cover_is_sound() {
        let (csp, x, y, _) = setup_csp::<i32>();
        let asn = vec![
            vv(x.label().clone(), 1),
            vv(y.label().clone(), 2),
        ];

        for c in csp.cover(&asn) {
            for v in c.scp() {
                assert!(asn.iter().any(|vv| vv.label == *v.label()));
            }
        }
    }

}