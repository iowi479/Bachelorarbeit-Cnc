pub mod cnc;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::cnc::{
        middleware::IPVSDsyncTSNScheduling,
        northbound::{MockInsertStreamAdapter, MockRemoveStreamAdapter},
        southbound::NetconfAdapter,
        storage::FileStorage,
        topology::MockTopology,
        Cnc,
    };

    #[test]
    fn it_works() {
        assert_eq!(true, true)
    }

    #[test]
    fn test_insert_stream() {
        // Create needed Components
        let northbound = MockInsertStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_failing();
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

    #[test]
    fn test_remove_stream() {
        // Create needed Components
        let northbound = MockRemoveStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_failing();
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
        println!("done");
        assert_eq!(true, true)
    }

    #[test]
    fn test_compute_all() {
        assert_eq!(true, true)
    }
}
