/**************************************
- Author: Clement Poncelet
- Desc: Main Trait of consistency management
    Scheme: Arc oriented Algorithm 7 (gac_enforce_arc) | Var oriented Algorithm 9 (gac_enforce_var)
    Revise: AC1 Algorithm 8 (revise) | AC3 Algorithm 18 (seekSupport-3)  | AC2001 Algorithm 19 (seekSupport-2001)
- Optimization:
    add triggerEvent type for handling solver's events (wipeout, var assignments, val deletions, restart...)
***************************************/

use crate::csp::csp::Csp;
use crate::csp::prelude::domain::OrdT;
pub(crate) use crate::solver::consistency::revise::Revise;
use crate::solver::consistency::scheme::{Scheme};

//TriggerEvent

pub struct Consistency<S, R, T:OrdT>
where
    S: Scheme<T, R>,
    R: Revise<T>,
{
    scheme: S,
    revise: R,
    _phantom: std::marker::PhantomData<T>
}

impl<S, R, T:OrdT> Consistency<S, R, T>
where
    S: Scheme<T, R>,
    R: Revise<T>,
{
    pub fn new(scheme: S, revise: R) -> Self {
        Self {
            scheme,
            revise,
            _phantom: std::marker::PhantomData
        }
    }

    pub fn enforce_consistency(&mut self, csp: &mut Csp<T>, events: Vec<String>) {
        self.scheme.enforce(csp, events, &mut self.revise);
    }
}