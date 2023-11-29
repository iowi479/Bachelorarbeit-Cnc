use std::sync::{RwLock, Weak};

use super::{topology::Topology, tsntypes::uni_types::Stream, Cnc};

#[derive(Debug)]
pub struct Schedule {
    // TODO impl computed Schedule
}

pub trait SchedulerAdapterInterface {
    // TODO streams sorted by domain?
    fn compute_schedule(&self, topology: &Topology, streams: Vec<&Stream>) -> Schedule;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = Some(cnc);
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>);
}

pub struct IPVSDsyncTSNScheduling {
    // TODO impl sched-algo
    cnc: Option<Weak<RwLock<Cnc>>>,
}

impl IPVSDsyncTSNScheduling {
    pub fn new() -> Self {
        Self { cnc: None }
    }
}

impl SchedulerAdapterInterface for IPVSDsyncTSNScheduling {
    fn compute_schedule(&self, topology: &Topology, streams: Vec<&Stream>) -> Schedule {
        // TODO call sched-algo
        dbg!("compute schedule from flow");
        Schedule {}
    }

    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>) {
        self.cnc = Some(cnc);
    }
}
