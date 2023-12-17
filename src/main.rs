use ba::cnc::{
    middleware::IPVSDsyncTSNScheduling, northbound::MockUniAdapter, southbound::NetconfAdapter,
    storage::FileStorage, topology::MockTopology, Cnc,
};
use std::sync::Arc;

fn main() {
    // Create needed Components
    let northbound = MockUniAdapter::new(String::from("test-cuc-id"));
    let southbound = NetconfAdapter::new();
    let storage = FileStorage::new();
    let topology = MockTopology::new();
    let scheduler = IPVSDsyncTSNScheduling::new();

    // Configuration for CNC
    let id: u32 = 0;
    let domain: String = String::from("test-domain-id");

    Cnc::run(
        id,
        domain,
        Arc::new(northbound),
        Arc::new(southbound),
        Arc::new(storage),
        Arc::new(topology),
        Arc::new(scheduler),
    );
}
