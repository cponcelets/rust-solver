use petgraph::graph::UnGraph;
use petgraph::dot::Dot;

pub mod csp;
mod lib;

fn main() {

    //let p_init = csp::new()

    println!("Hello, world!");
    let g = UnGraph::<(), ()>::from_edges(&[(0, 1), (1, 2), (2, 3), (0, 3)]);
    println!("Graph: {:?}", g);

    let basic_dot = Dot::new(&g);
    println!("Basic DOT format:\n{:?}\n", basic_dot);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn it_works() {
        let dom012 = csp::domains::ExDom::new(vec![0,1,2]);
        let vmap = HashMap::from([
                                     (String::from("x"), Rc::new(csp::variables::ExVar::new(String::from("x"), dom012.clone()))),
                                     (String::from("y"), Rc::new(csp::variables::ExVar::new(String::from("y"), dom012.clone()))),
                                     (String::from("z"), Rc::new(csp::variables::ExVar::new(String::from("z"), dom012.clone())))
            ]);

        let p_init = csp::Csp::new(&vmap,
        {vec![
            Box::new(csp::constraints::EqConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("y")).unwrap().clone())),
            Box::new(csp::constraints::LtConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone())),
            Box::new(csp::constraints::NeqConstraint::new(vmap.get(&String::from("x")).unwrap().clone(), vmap.get(&String::from("z")).unwrap().clone()))
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



        //Let $c$ be a constraint, $x ∈ scp(c)$ and $a ∈ dom(x)$.
        // - The set of **valid tuples** for $(x, a)$ on $c$ is $val(c)_{x=a} = \{\tau∈ val(c) \, | \, \tau[x] = a\}$.
        // - The set of **supports** for $(x, a)$ on $c$ is $sup(c)_{x=a} = val(c)_{x=a} ∩ rel(c)$.
        // - The set of **conflicts** for $(x, a)$ on $c$ is $con(c)_{x=a} = val(c)_{x=a} \setminus sup(c)$.
        // - The set of **strict supports** for $(x, a)$ on $c$ is $sup(c)\downarrow_{x=a} = \{\tau[scp(c) \setminus {x}] \, | \, \tau ∈ sup(c)_{x=a}\}$.

        // - tightness and looseness
        // - normalized

        //assert_eq!(csp::solve(p_init), true);
    }
}

// **Primal Graph**
/* Constraint:
 - It has $n$ vertices corresponding to the variables of $P$ and
  one edge **for each pair** of variables residing in the same constraint scope.
 */

// **Dual Graph**
/* Constraint:
It has:
- $e$ vertices corresponding to the **constraints** of $P$
- and one edge for **each pair** of constraints *sharing at least one variable*.
 */

// Compatibility Hypergraph (micro-structure)
/* Constraint:
- one vertex per `v-value` of $P$
- and one hyperedge per constraint support.
 */


// **Search algorithms**
//test csp:
/*
 - Vars {x, y, z}
 - Dom {0,1,2}
 - Cons {x = y, x < z, y <> z}
 */