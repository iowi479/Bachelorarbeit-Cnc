use std::sync::Weak;

use super::{storage::Config, topology::Topology, types::uni_types::Stream, Cnc};

pub struct Schedule {
    configs: Vec<Config>,
    // TODO impl computed Schedule
}

pub trait SchedulerAdapterInterface {
    // TODO streams sorted by domain?
    fn compute_schedule(&self, topology: Topology, streams: Vec<Stream>) -> Schedule;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct IPVSDsyncTSNScheduling {
    // TODO impl sched-algo
    cnc: Weak<Cnc>,
}

impl IPVSDsyncTSNScheduling {
    pub fn new() -> Self {
        Self {
            cnc: Weak::default(),
        }
    }
}

impl SchedulerAdapterInterface for IPVSDsyncTSNScheduling {
    fn compute_schedule(&self, topology: Topology, streams: Vec<Stream>) -> Schedule {
        // TODO call sched-algo
        todo!("compute schedule");
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }
}
