/**************************************
- Author: Clement Poncelet
- Desc: Contains:
    - Type OrdT, for factorize a solver type value (ordered set)
    - Domain traits
- Todo:
    - Prevent from SetDomIter to appear
***************************************/

/**************************************
            Type
***************************************/
use crate::csp::domain::setdom::SetDomIter;

pub trait OrdT:
Clone + std::fmt::Debug + std::fmt::Display + Eq + Ord + std::hash::Hash
{}

impl<T> OrdT for T
where
    T:Clone + std::fmt::Debug + std::fmt::Display + Eq + Ord + std::hash::Hash
{}

/**************************************
            Domain
***************************************/

pub trait Domain<T:OrdT> {
    fn clone(&self) -> Self;

    //Iterators
    fn iter_all(&self) -> std::slice::Iter<'_, T>;
    fn iter(&self) -> SetDomIter<'_, T>;

    fn get_initial_values(&self) -> &Vec<T>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn min(&self) -> Option<T>;
    fn max(&self) -> Option<T>;

    //Trailing
    fn active_values(&self) -> Vec<T>;
    fn head(&self) -> Option<T>;
    fn tail(&self) -> Option<T>;
    fn remove_value(&mut self, v : &T, lvl : usize) -> ();
    //assignment
    fn reduce_to(&mut self, v : &T, lvl : usize) -> ();
    fn restore_up_to(&mut self, lvl : usize) -> ();
    fn add_value(&mut self, v : &T) -> ();
}