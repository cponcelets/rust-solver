/**************************************
- Author: Clement Poncelet
- Desc: Contains:
    - Extensional Domain, an ordered set of values
***************************************/

/**************************************
            ExDom
***************************************/
use crate::csp::domain::traits::{Domain, OrdT};

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct ExDom<OrdT> {
    values: Vec<OrdT>
}

impl<T:OrdT> ExDom<T> {
    //Constructor
    pub fn new (values: Vec<T>) -> ExDom<T> {
        ExDom { values }
    }
}

impl<T:OrdT> Domain<T> for ExDom<T> {
    fn clone(&self) -> Self {
        ExDom::new(self.values.clone())
    }
    //Iterator
    fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    //API
    fn get(&self) -> &Vec<T> { &self.values }
    fn size(&self) -> usize {
        self.values.len()
    }
    fn is_empty(&self) -> bool { self.size() == 0 }
    fn min(&self) -> Option<T> { Some(self.values[0].clone()) }
    fn max(&self) -> Option<T> { Some(self.values[self.size() - 1].clone()) }
}

impl<T: OrdT> std::fmt::Display for ExDom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in &self.values {
            write!(f, "{},", i)?;
        }
        write!(f, "}}")
    }
}

// ----  Smart iterator for cartesian products

pub struct CartesianWalker<'a, T:OrdT> {
    domains: Vec<&'a ExDom<T>>,
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T:OrdT> CartesianWalker<'a, T> {
    pub fn new(domains: Vec<&'a ExDom<T>>) -> Self {
        let k = domains.len();

        Self {
            domains,
            indices: vec![0; k],
            done: false,
        }
    }
}

impl<'a, T:OrdT> Iterator for CartesianWalker<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.domains.is_empty() {
            self.done = true;
            return Some(vec![]);
        }

        // Build current tuple
        let tuple = self.indices
            .iter()
            .enumerate()
            .map(|(i, &idx)| self.domains[i].values[idx].clone())
            .collect();

        // Advance indices (odometer-style)
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;

            if self.indices[i] < self.domains[i].values.len() {
                break;
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    self.done = true;
                }
            }
        }
        Some(tuple)
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
    use crate::csp::domain::extdom::{ExDom, CartesianWalker};
    use crate::csp::domain::traits::Domain;

    #[test]
    fn domain_str_value() {
        let dom_color = ExDom::new(vec!["dg", "mg", "lg", "w"]);
        //ExDom: size
        assert_eq!(dom_color.size(), 4);
        //ExDom: ordering
        assert_eq!(dom_color.min(), Some("dg"));
        assert_eq!(dom_color.max(), Some("w"));
        assert!(dom_color.min() < dom_color.max());
        //ExDom: membership
        assert!(dom_color.get().contains(&"mg"));
        assert!(!dom_color.get().contains(&"z"));
    }

    #[test]
    fn domain_int_value() {
        let dom_int = ExDom::new(vec![1, 2, 3, 4]);
        //ExDom: size
        assert_eq!(dom_int.size(), 4);
        //ExDom: ordering
        assert_eq!(dom_int.min(), Some(1));
        assert_eq!(dom_int.max(), Some(4));
        assert!(dom_int.min() < dom_int.max());
        //ExDom: membership
        assert!(dom_int.get().contains(&3));
        assert!(!dom_int.get().contains(&5));
    }

    //CartesianWalker
    #[test]
    fn cartesian_walker_two_domains() {
        let d1 = ExDom::new(vec![1, 2]);
        let d2 = ExDom::new(vec![3, 4]);

        let doms = vec![&d1, &d2];
        let w = CartesianWalker::new(doms);

        let results: Vec<Vec<i32>> = w.collect();

        //size
        assert_eq!(results.len(), 4);

        //contains
        assert!(results.contains(&vec![1, 3]));
        assert!(results.contains(&vec![1, 4]));
        assert!(results.contains(&vec![2, 3]));
        assert!(results.contains(&vec![2, 4]));
    }

    #[test]
    fn cartesian_walker_single_domain() {
        let d = ExDom::new(vec![1, 2, 3]);
        let doms = vec![&d];

        let results: Vec<Vec<i32>> =
            CartesianWalker::new(doms).collect();

        assert_eq!(results, vec![
            vec![1],
            vec![2],
            vec![3],
        ]);
    }

    #[test]
    fn cartesian_walker_zero_arity() {
        let results: Vec<Vec<i32>> =
            CartesianWalker::new(vec![]).collect();
        assert_eq!(results, vec![vec![]]);
    }

    #[test]
    fn cartesian_walker_order() {
        let d1 = ExDom::new(vec![1, 2]);
        let d2 = ExDom::new(vec![3, 4]);

        let results: Vec<Vec<i32>> =
            CartesianWalker::new(vec![&d1, &d2]).collect();

        assert_eq!(results, vec![
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
        ]);
    }
}