use ba::cnc::{
    middleware::IPVSDsyncTSNScheduling, northbound::MockUniAdapter, southbound::NetconfAdapter,
    storage::FileStorage, topology::MockTopology, Cnc,
};

fn main() {
    // Create needed Components
    let northbound = MockUniAdapter::new();
    let southbound = NetconfAdapter::new();
    let storage = FileStorage::new();
    let topology = MockTopology::new();
    let scheduler = IPVSDsyncTSNScheduling::new();

    // Configuration for CNC
    let id: u32 = 123;
    let domain: String = String::from("test-domain-id");

    Cnc::run(
        id,
        domain,
        Box::new(northbound),
        Box::new(southbound),
        Box::new(storage),
        Box::new(topology),
        Box::new(scheduler),
    );
}
