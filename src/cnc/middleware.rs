use std::sync::{RwLock, Weak};

use super::Cnc;

pub struct Schedule {
    // TODO impl computed Schedule
}

#[derive(Debug)]
pub struct Flow {
    // TODO impl flows
}

pub trait SchedulerAdapterInterface {
    fn compute_schedule(&self, flow: Flow) -> Schedule;

    // CNC Configuration
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
    fn compute_schedule(&self, flow: Flow) -> Schedule {
        // TODO call sched-algo
        dbg!("compute schedule from flow");
        dbg!(flow);
        Schedule {}
    }

    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>) {
        self.cnc = Some(cnc);
    }
}
