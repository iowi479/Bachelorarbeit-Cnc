use ba::cnc::{
    Cnc, NorthboundComponent, ScheduleComponent, SouthboundComponent, StorageComponent,
    TopologyComponent,
};
use std::{cell::RefCell, sync::Arc};

fn main() {
    // Modiy the Component types in mod.rs for specific configurations
    let northbound = NorthboundComponent::new();
    let southbound = SouthboundComponent::new();
    let storage = StorageComponent::new();
    let topology = TopologyComponent::new();
    let scheduler = ScheduleComponent::new();

    // Configuration for CNC
    let id: u32 = 123;
    let domain: String = String::from("test-domain-id");

    let cnc: Arc<RefCell<Cnc>> = Cnc::new(
        id, domain, northbound, southbound, storage, topology, scheduler,
    );

    println!("CNC-ID: {}", cnc.borrow().get_id());
    println!("CNC-DOMAIN: {}", cnc.borrow().get_domain());
}
