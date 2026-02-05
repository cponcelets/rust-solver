/**************************************
- Author: Clement Poncelet
- Desc: Contains:
    - Set Domain, an explicit and ordered set of values
    - Trailing method for backtracking
***************************************/

/**************************************
            SetDom
***************************************/
use crate::csp::domain::traits::{Domain, OrdT};

#[derive(Debug, Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct SetDom<OrdT> {
    values: Vec<OrdT>,
    //trailing (indices on values)
    absent:     Vec<usize>, //lvl of removed values
    next:       Vec<usize>, //links from first to last
    prev:       Vec<usize>, //links from last to first
    prev_absent: Vec<usize>,//links of removed values
    head: usize,
    tail: usize,
    tail_absent: usize,
    //fast access to size
    size:usize
}

impl<T:OrdT> SetDom<T> {
    //Constructor
    pub fn new (values: Vec<T>) -> SetDom<T> {
        let d = values.len();

        let mut next: Vec<_> = (2..d+1).collect();
        let mut prev: Vec<_> = (1..d).collect();
        next.push( 0);
        prev.insert(0, 0);

        SetDom {
            values,
            absent:      vec![0; d],
            next,
            prev,
            prev_absent: vec![0; d],
            head:1,
            tail:d,
            tail_absent:0,
            size:d
        }
    }

    #[cfg(debug_assertions)]
    fn check_size_invariant(&self) {
        let real = self.iter_on_active().count();
        assert_eq!(self.size, real);
    }
}

impl<T:OrdT> Domain<T> for SetDom<T> {
    fn clone(&self) -> Self {
        SetDom {
            values: self.values.clone(),
            absent: self.absent.clone(),
            next:   self.next.clone(),
            prev:   self.prev.clone(),
            prev_absent: self.prev_absent.clone(),

            head:           self.head,
            tail:           self.tail,
            tail_absent: self.tail_absent,

            size: self.size
        }
    }

    fn iter_all(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    fn iter(&self) -> SetDomIter<'_, T> {
        self.iter_on_active()
    }

    //API
    fn get_initial_values(&self) -> &Vec<T> { &self.values }
    fn size(&self) -> usize { self.size }
    fn is_empty(&self) -> bool { self.size() == 0 }
    fn min(&self) -> Option<T> { self.iter_on_active().min().cloned() }
    fn max(&self) -> Option<T> { self.iter_on_active().max().cloned() }

    //trailing
    fn active_values(&self) -> Vec<T> {
        self.iter_on_active().cloned().collect()
    }
    fn head(&self) -> Option<T> { Some(self.values[self.head-1].clone()) }
    fn tail(&self) -> Option<T> { Some(self.values[self.tail-1].clone()) }
    fn remove_value(&mut self, v : &T, lvl : usize) -> () {
        if self.active_values().contains(v) {
            match self.values.iter().position(|i| i == v) {
                Some(idx) => {
                    self.absent[idx] = lvl;
                    self.prev_absent[idx] = self.tail_absent;
                    self.tail_absent = idx + 1;

                    if self.prev[idx] == 0 { self.head = self.next[idx]; } else { self.next[self.prev[idx] - 1] = self.next[idx]; }

                    if self.next[idx] == 0 { self.tail = self.prev[idx]; } else { self.prev[self.next[idx] - 1] = self.prev[idx]; }

                    self.size -= 1;
                }
                _ => panic!("Error value {} not in domain",v)
            }
        }
        #[cfg(debug_assertions)]
        self.check_size_invariant();
    }
    fn reduce_to(&mut self, v : &T, lvl : usize) -> () {
        let mut b = self.head;
        while b != 0 {
            let val = self.values[b-1].clone();
            if val != *v {
                self.remove_value(&val, lvl);
            }
            b = self.next[b-1];
        }
    }

    fn restore_up_to(&mut self, lvl: usize) -> () {
        let mut b = self.tail_absent;
        while b != 0 && self.absent[b-1] >= lvl {
            self.add_value(&self.values[b-1].clone());
            b = self.prev_absent[b-1];
        }
    }

    fn add_value(&mut self, v: &T) -> () {
        if !self.active_values().contains(v) {
            match self.values.iter().position(|i| i == v) {
                Some(idx) => {
                    self.absent[idx] = 0;
                    self.tail_absent = self.prev_absent[idx];

                    if self.prev[idx] == 0 { self.head = idx + 1; } else { self.next[self.prev[idx] - 1] = idx + 1; }

                    if self.next[idx] == 0 { self.tail = idx + 1; } else { self.prev[self.next[idx] - 1] = idx + 1; }

                    self.size += 1;
                }
                _ => panic!("Error value {} not in domain",v)
            }
        }
        #[cfg(debug_assertions)]
        self.check_size_invariant();
    }
}

impl<T: OrdT> std::fmt::Display for SetDom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in &self.values {
            write!(f, "{},", i)?;
        }
        write!(f, "}}")
    }
}

impl<T: Clone> SetDom<T> {
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

// ----  Smart iterator for cartesian products

pub struct SetDomIter<'a, OrdT> {
    dom: &'a SetDom<OrdT>,
    idx: usize,
}
impl<T:OrdT> SetDom<T> {
    pub fn iter_on_active(&self) -> SetDomIter<'_, T> {
        SetDomIter {
            dom: self,
            idx: self.head,
        }
    }
}

impl<'a, T:OrdT> Iterator for SetDomIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 0 {
            return None;
        }
        let i = self.idx - 1;
        self.idx = self.dom.next[i];
        Some(&self.dom.values[i])
    }
}


pub struct CartesianWalker<T:OrdT> {
    domains: Vec<Vec<T>>,
    indices: Vec<usize>,
    done: bool,
}

impl<T:OrdT> CartesianWalker<T> {
    pub fn new(domains: Vec<Vec<T>>) -> Self {
        let k = domains.len();
        Self {
            domains,
            indices: vec![0; k],
            done: false,
        }
    }
}

impl<T:OrdT> Iterator for CartesianWalker<T> {
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
        let tuple: Vec<T> = self.indices
            .iter()
            .enumerate()
            .map(|(i, &idx)| self.domains[i][idx].clone())
            .collect();

        // Advance indices (odometer-style)
        for i in (0..self.indices.len()).rev() {
            self.indices[i] += 1;
            if self.indices[i] < self.domains[i].len() {
                return Some(tuple);
            }
            self.indices[i] = 0;
        }

        self.done = true;
        Some(tuple)
    }
}

/**************************************
            Unit Tests
***************************************/

#[cfg(test)]
mod tests {
use crate::csp::domain::setdom::{SetDom, CartesianWalker};
    use crate::csp::domain::traits::Domain;

    #[test]
    fn domain_size_after_remove() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&2, 0);
        assert_eq!(dom.size(), 2);
    }

    #[test]
    fn domain_size_remove_twice() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&2, 0);
        dom.remove_value(&2, 1);
        assert_eq!(dom.size(), 2); // must not decrement twice
    }

    #[test]
    fn domain_size_empty() {
        let mut dom = SetDom::new(vec![1]);
        dom.remove_value(&1, 0);
        assert_eq!(dom.size(), 0);
    }

    #[test]
    fn domain_str_value() {
        let dom_color = SetDom::new(vec!["dg", "mg", "lg", "w"]);
        //SetDom: size
        assert_eq!(dom_color.size(), 4);
        //SetDom: ordering
        assert_eq!(dom_color.min(), Some("dg"));
        assert_eq!(dom_color.max(), Some("w"));
        assert!(dom_color.min() < dom_color.max());
        //SetDom: membership
        assert!(dom_color.active_values().contains(&"mg"));
        assert!(!dom_color.active_values().contains(&"z"));
    }

    #[test]
    fn domain_int_value() {
        let dom_int = SetDom::new(vec![1, 2, 3, 4]);
        //SetDom: size
        assert_eq!(dom_int.size(), 4);
        //SetDom: ordering
        assert_eq!(dom_int.min(), Some(1));
        assert_eq!(dom_int.max(), Some(4));
        assert!(dom_int.min() < dom_int.max());
        //SetDom: membership
        assert!(dom_int.active_values().contains(&3));
        assert!(!dom_int.active_values().contains(&5));
    }

    //CartesianWalker
    #[test]
    fn cartesian_walker_two_domains() {
        let d1 = SetDom::new(vec![1, 2]);
        let d2 = SetDom::new(vec![3, 4]);

        let doms = vec![d1.active_values(), d2.active_values()];
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
        let d = SetDom::new(vec![1, 2, 3]);
        let results: Vec<Vec<i32>> =
            CartesianWalker::new(vec![d.active_values()]).collect();

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
        let d1 = SetDom::new(vec![1, 2]);
        let d2 = SetDom::new(vec![3, 4]);

        let results: Vec<Vec<i32>> =
            CartesianWalker::new(vec![d1.active_values(), d2.active_values()]).collect();

        assert_eq!(results, vec![
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
        ]);
    }

    #[test]
    fn domain_iter_active() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&2, 0);

        let vals: Vec<_> = dom.iter().cloned().collect();
        assert_eq!(vals, vec![1, 3]);
    }

    #[test]
    fn domain_iter_all() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&2, 0);

        let vals: Vec<_> = dom.iter_all().cloned().collect();
        assert_eq!(vals, vec![1, 2, 3]);
    }

    #[test]
    fn trailing_remove_add() {
        let mut d = SetDom::new(vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.remove_value(&3, 2);
        assert_eq!(d.active_values(), vec![0, 2, 4, 5, 7, 8, 9]);

        d.restore_up_to(2);
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn trailing_remove_add_head() {
        let mut d = SetDom::new(vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.remove_value(&0, 2);
        assert_eq!(d.active_values(), vec![2, 3, 4, 5, 7, 8, 9]);

        d.restore_up_to(2);
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn trailing_remove_add_tail() {
        let mut d = SetDom::new(vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.remove_value(&9, 2);
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8]);

        d.restore_up_to(2);
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn trailing_remove_multiple() {
        let mut d = SetDom::new(vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.remove_value(&2, 1);
        d.remove_value(&4, 1);
        d.remove_value(&3, 2);
        assert_eq!(d.active_values(), vec![0, 5, 7, 8, 9]);
        assert_eq!(*d.get_initial_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.restore_up_to(2);
        assert_eq!(d.active_values(), vec![0, 3, 5, 7, 8, 9]);

        d.restore_up_to(1);
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn trailing_consistency() {
        let mut d = SetDom::new(vec![0, 2, 3, 4, 5, 7, 8, 9]);

        assert_eq!(d.size(), 8);
        assert_eq!(d.head(), Some(0));
        assert_eq!(d.tail(), Some(9));
        assert_eq!(d.active_values(), vec![0, 2, 3, 4, 5, 7, 8, 9]);

        d.remove_value(&3, 2);
        d.remove_value(&7, 2);
        assert_eq!(d.active_values(), vec![0, 2, 4, 5, 8, 9]);

        d.reduce_to(&5, 3);
        assert_eq!(d.active_values(), vec![5]);

        d.restore_up_to(3);
        assert_eq!(d.active_values(), vec![0, 2, 4, 5, 8, 9]);
    }

    //with iterator
    #[test]
    fn domain_trailing_remove_and_restore() {
        let mut d = SetDom::new(vec![1,2,3]);
        let lvl = 1;

        d.remove_value(&2, lvl);
        assert_eq!(d.iter_on_active().cloned().collect::<Vec<_>>(), vec![1, 3]);

        d.restore_up_to(lvl);
        assert_eq!(d.iter_on_active().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn domain_min_max_initial() {
        let dom = SetDom::new(vec![3, 1, 2]);
        assert_eq!(dom.min(), Some(1));
        assert_eq!(dom.max(), Some(3));
    }

    #[test]
    fn domain_min_after_remove() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&1, 0);
        assert_eq!(dom.min(), Some(2));
    }

    #[test]
    fn domain_max_after_remove() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&3, 0);
        assert_eq!(dom.max(), Some(2));
    }

    #[test]
    fn domain_min_max_empty() {
        let mut dom = SetDom::new(vec![1]);
        dom.remove_value(&1, 0);
        assert_eq!(dom.min(), None);
        assert_eq!(dom.max(), None);
    }

    #[test]
    fn domain_min_max_backtrack() {
        let mut dom = SetDom::new(vec![1, 2, 3]);
        dom.remove_value(&1, 1);
        assert_eq!(dom.min(), Some(2));
        dom.restore_up_to(0);
        assert_eq!(dom.min(), Some(1));
    }
}