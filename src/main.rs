use ba::cnc::{
    middleware::IPVSDsyncTSNScheduling, northbound::MockUniAdapter, southbound::NetconfAdapter,
    storage::FileStorage, topology::MockTopology, Cnc,
};
use std::sync::{Arc, RwLock};

fn main() {
    // Modiy the Component types in mod.rs for specific configurations
    let northbound = MockUniAdapter::new();
    let southbound = NetconfAdapter::new();
    let storage = FileStorage::new();
    let topology = MockTopology::new();
    let scheduler = IPVSDsyncTSNScheduling::new();

    // Configuration for CNC
    let id: u32 = 123;
    let domain: String = String::from("test-domain-id");

    let cnc: Arc<RwLock<Cnc>> = Cnc::new(
        id,
        domain,
        Box::new(northbound),
        Box::new(southbound),
        Box::new(storage),
        Box::new(topology),
        Box::new(scheduler),
    );

    println!("CNC-ID: {}", cnc.read().unwrap().get_id());
    println!("CNC-DOMAIN: {}", cnc.read().unwrap().get_domain());
}
