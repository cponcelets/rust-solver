/**************************************
- Author: Clement Poncelet
- Desc: Trait to implement monitoring solver feature
***************************************/
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use crate::csp::csp::Csp;
use crate::csp::prelude::domain::OrdT;

pub trait Monitor {
    fn on_revision_check(&mut self);
    fn on_revise_fruitless(&mut self);
    fn on_constraint_check(&mut self);
    fn on_value_deleted(&mut self);
    fn on_domain_wipeout(&mut self);
    fn on_enforce_start(&mut self);
    fn on_enforce_end(&mut self);
    fn on_enqueue(&mut self);
    fn on_dequeue(&mut self);
    fn on_domain_snapshot<T: OrdT>(&mut self, csp: &Csp<T>);
}


/**************************************
        Monitor off
***************************************/

pub struct NoMonitor;

impl Monitor for NoMonitor {
    #[inline(always)]
    fn on_revision_check(&mut self) {}

    #[inline(always)]
    fn on_revise_fruitless(&mut self) {}

    #[inline(always)]
    fn on_constraint_check(&mut self) {}

    #[inline(always)]
    fn on_value_deleted(&mut self) {}

    #[inline(always)]
    fn on_domain_wipeout(&mut self) {}
    #[inline(always)]
    fn on_enforce_start(&mut self) {}
    #[inline(always)]
    fn on_enforce_end(&mut self) {}
    #[inline(always)]
    fn on_enqueue(&mut self) {}
    #[inline(always)]
    fn on_dequeue(&mut self) {}
    #[inline(always)]
    fn on_domain_snapshot<T: OrdT>(&mut self, csp: &Csp<T>) {}
}

/**************************************
        Monitor
***************************************/

#[derive(Default)]
pub struct Statistics {
    //Enforcement strategy
    pub enforce_calls: usize,
    pub total_enforce_time: Duration,
    start_time: Option<Instant>,
    pub nb_enqueue: usize,
    pub max_queue_size: usize,
    //Propagation
    pub revise_calls: usize,
    pub revise_fruitless: usize,
    pub checks: usize,
    //CSP
    pub domain_wipeouts: usize,
    pub value_deletions: usize,
    pub domain_histogram: BTreeMap<usize, usize>
}

impl Monitor for Statistics {
    fn on_revision_check(&mut self) {
        self.revise_calls += 1;
    }

    fn on_revise_fruitless(&mut self) {
        self.revise_fruitless += 1;
    }

    fn on_constraint_check(&mut self) {
        self.checks += 1;
    }

    fn on_value_deleted(&mut self) {
        self.value_deletions += 1;
    }

    fn on_domain_wipeout(&mut self) {
        self.domain_wipeouts += 1;
    }
    fn on_enforce_start(&mut self) {self.start_time = Some(Instant::now());}
    fn on_enforce_end(&mut self) {
        if let Some(start) = self.start_time.take() {
            self.total_enforce_time += start.elapsed();
            self.enforce_calls += 1;
        }
    }

    fn on_enqueue(&mut self) {
        self.nb_enqueue += 1;
        if self.nb_enqueue > self.max_queue_size {
            self.max_queue_size = self.nb_enqueue;
        }
    }

    fn on_dequeue(&mut self) {
        self.nb_enqueue -= 1;
    }

    fn on_domain_snapshot<T: OrdT>(&mut self, csp: &Csp<T>) {
        for v in csp.vars().values() {
            let size = v.valid_size();
            *self.domain_histogram.entry(size).or_insert(0) += 1;
        }
    }
}