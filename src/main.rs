use ba::cnc::northbound::MockUniAdapter;
use ba::cnc::scheduling::MockTSNScheduler;
use ba::cnc::southbound::NetconfAdapter;
use ba::cnc::storage::FileStorage;
use ba::cnc::topology::MockTopology;
use ba::cnc::Cnc;
use std::sync::Arc;

fn main() {
    // Create needed Components
    let northbound = MockUniAdapter::new(String::from("test-cuc-id"));
    let southbound = NetconfAdapter::new();
    let storage = FileStorage::new();
    let topology = MockTopology::new_failing();
    let scheduler = MockTSNScheduler::new();

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
