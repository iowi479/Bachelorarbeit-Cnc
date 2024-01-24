use ba::cnc::northbound::MockUniAdapter;
use ba::cnc::scheduling::MockTSNScheduler;
use ba::cnc::southbound::NetconfAdapter;
use ba::cnc::storage::FileStorage;
use ba::cnc::topology::MockTopology;
use ba::cnc::Cnc;
use std::sync::Arc;

/// this is a example of calling the CNC with the specified components
fn main() {
    // configuration for CNC
    let id: u32 = 0;
    let domain: String = String::from("test-domain-id");

    let cuc_id: String = String::from("test-cuc-id");

    // create needed components
    let northbound = MockUniAdapter::new(cuc_id);
    let southbound = NetconfAdapter::new();
    let storage = FileStorage::new();
    let topology = MockTopology::new_failing();
    let scheduler = MockTSNScheduler::new();

    Cnc::run(
        id,
        domain,
        Arc::new(northbound),
        Arc::new(southbound),
        Arc::new(storage),
        Arc::new(topology),
        Arc::new(scheduler),
    );

    // this is blocked until the cnc stops its operation
    println!("CNC fully stopped and program is exiting...");
}
