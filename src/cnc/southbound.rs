use std::sync::{RwLock, Weak};

use super::{tsntypes::shed_types::SchedParameters, Cnc};

pub trait SouthboundControllerInterface {}
pub trait SouthboundAdapterInterface {
    fn send_config(&self, config: SchedParameters);
    fn retrieve_station_capibilities(&self);

    // CNC Configuration
    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>);
}

pub struct NetconfAdapter {
    id: u32,
    cnc: Option<Weak<RwLock<Cnc>>>,
}

impl NetconfAdapter {
    pub fn new() -> Self {
        Self { id: 0, cnc: None }
    }
}

impl SouthboundAdapterInterface for NetconfAdapter {
    fn send_config(&self, config: SchedParameters) {
        dbg!(self.id);
        dbg!(config);
        dbg!("sending config ");

        // TODO impl config modification

        // read old config using libnetconf2 and <get-config> rpc
        // modify config with given sched-data using yang2-rs
        // parse data-tree to xml
        // create <edit-config> rpc using libnetconf2
        // send rpc and await <ok> resonse; else error
    }

    fn retrieve_station_capibilities(&self) {
        dbg!("retrieveing stations capibilities");

        // TODO impl retrieve capabilites
        // may not be possible atm using netconf
    }

    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>) {
        self.cnc = Some(cnc);
    }
}
