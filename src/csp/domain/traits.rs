/**************************************
- Author: Clement Poncelet
- Desc: Contains:
    - Type OrdT, for factorize a solver type value (ordered set)
    - Domain traits
***************************************/

/**************************************
            Type
***************************************/

pub trait OrdT:
Clone + std::fmt::Display + Eq + Ord + std::hash::Hash {}

impl<T> OrdT for T
where
    T:Clone + std::fmt::Display + Eq + Ord + std::hash::Hash {}

/**************************************
            Domain
***************************************/

pub trait Domain<T:OrdT> {
    fn clone(&self) -> Self;
    //Iterator
    fn iter(&self) -> std::slice::Iter<'_, T>;
    fn get(&self) -> &Vec<T>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn min(&self) -> Option<T>;
    fn max(&self) -> Option<T>;
}