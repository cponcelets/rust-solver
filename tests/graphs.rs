use std::collections::HashMap;
use std::rc::Rc;
use petgraph::dot::Dot;
use rust_solver::csp::csp::Csp;
use rust_solver::csp::prelude::extdom::ExDom;
use rust_solver::csp::prelude::extvar::{generate_variables, ExVar};
use rust_solver::csp::prelude::intensional::{EqConstraint, LtConstraint, NeqConstraint};

#[test]
fn test_graphs() {
    let dom012 = ExDom::new(vec![0, 1, 2]);
    let vmap = HashMap::from([
        (String::from("x"), Rc::new(ExVar::new(String::from("x"), dom012.clone()))),
        (String::from("y"), Rc::new(ExVar::new(String::from("y"), dom012.clone()))),
        (String::from("z"), Rc::new(ExVar::new(String::from("z"), dom012.clone())))
    ]);

    let p_init = Csp::new(vmap.clone(),
                                            {vec![
                                                Box::new(EqConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("y")).unwrap().clone())),
                                                Box::new(LtConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone()))
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

    //dual graph - density (e/(\overset{n}{k})?
    let dual_graph = p_init.dual_graph();
    let dual_dot = Dot::new(&dual_graph);
    println!("Dual graph in DOT format:\n{:?}\n", dual_dot);

    //compatibility hypergraph (micro-structure)
    //- one vertex per `v-value` of $P$
    //- and one hyperedge per constraint support.
    let micro_structure = p_init.micro_structure();
    let micro_dot = Dot::new(&micro_structure);
    println!("Micro-structure in DOT format:\n{:?}\n", micro_dot);
}

#[test]
fn test_graph_color() {
    let dom_color = ExDom::new(vec!["dg", "mg", "lg", "w"]);
    let vmap = generate_variables("x", 9, &dom_color);
    let p_init = Csp::new(vmap.clone(),
                                            {vec![
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x1")).unwrap().clone(), vmap.get(&String::from("x3")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x1")).unwrap().clone(), vmap.get(&String::from("x4")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x1")).unwrap().clone(), vmap.get(&String::from("x7")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x1")).unwrap().clone(), vmap.get(&String::from("x2")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x2")).unwrap().clone(), vmap.get(&String::from("x7")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x2")).unwrap().clone(), vmap.get(&String::from("x8")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x2")).unwrap().clone(), vmap.get(&String::from("x9")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x3")).unwrap().clone(), vmap.get(&String::from("x4")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x3")).unwrap().clone(), vmap.get(&String::from("x5")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x3")).unwrap().clone(), vmap.get(&String::from("x6")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x4")).unwrap().clone(), vmap.get(&String::from("x5")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x4")).unwrap().clone(), vmap.get(&String::from("x7")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x5")).unwrap().clone(), vmap.get(&String::from("x6")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x5")).unwrap().clone(), vmap.get(&String::from("x7")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x5")).unwrap().clone(), vmap.get(&String::from("x8")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x6")).unwrap().clone(), vmap.get(&String::from("x8")).unwrap().clone())),
                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x6")).unwrap().clone(), vmap.get(&String::from("x9")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x7")).unwrap().clone(), vmap.get(&String::from("x8")).unwrap().clone())),

                                                Box::new(NeqConstraint::new(vmap.get(&String::from("x8")).unwrap().clone(), vmap.get(&String::from("x9")).unwrap().clone())),
                               ]}
    );

    println!("{}", p_init);
    println!("n: {}, e: {}, d: {}, r: {}.", p_init.n(), p_init.e(), p_init.d(), p_init.r());

    let primal_graph = p_init.primal_graph();
    let primal_dot = Dot::new(&primal_graph);
    println!("Primal graph in DOT format:\n{:?}\n", primal_dot);
}