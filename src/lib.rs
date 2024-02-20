pub mod cnc;

#[cfg(test)]
mod tests {
    use crate::cnc::northbound::{
        MockComputeStreamAdapter, MockInsertStreamAdapter, MockRemoveStreamAdapter,
    };
    use crate::cnc::scheduling::MockTSNScheduler;
    use crate::cnc::southbound::{NetconfAdapter, SouthboundAdapterInterface};
    use crate::cnc::storage::{FileStorage, StorageAdapterInterface};
    use crate::cnc::topology::MockTopology;
    use crate::cnc::types::topology::SSHConfigurationParams;
    use crate::cnc::types::uni_types::compute_streams::{CucElement, Domain};
    use crate::cnc::types::uni_types::StreamStatus;
    use crate::cnc::Cnc;
    use serial_test::serial;
    use std::fs::File;
    use std::io::prelude::*;
    use std::sync::Arc;

    #[test]
    #[serial]
    fn insert_streams() {
        // Configuration for CNC
        let id: u32 = 0;
        let domain: String = String::from("test-domain-id");

        // This does only work for the preimplemented Filestorage
        let bspstorage = r#"[{"domain_id":"test-domain-id","cnc_enabled":true,"cuc":[{"cuc_id":"test-cuc-id","stream":[]}]}]"#;
        let mut file = File::create("domain_storage.json").expect("couldnt create file");
        file.write_all(bspstorage.as_bytes())
            .expect("couldnt write to file");

        // Create needed Components
        let northbound = MockInsertStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_functioning();
        let scheduler = MockTSNScheduler::new();

        Cnc::run(
            id,
            domain.clone(),
            Arc::new(northbound),
            Arc::new(southbound),
            Arc::new(storage),
            Arc::new(topology),
            Arc::new(scheduler),
        );

        // check storage
        let storage = FileStorage::new();
        storage.configure_storage();
        let storage_domain = storage.get_streams_in_domain(Domain {
            domain_id: domain.clone(),
            cuc: vec![CucElement {
                cuc_id: String::from("test-cuc-id"),
                stream_list: None,
            }],
        });

        let mut streams = vec![
            String::from("00-00-00-00-00-01:00-01"),
            String::from("00-00-00-00-00-01:00-02"),
            String::from("00-00-00-00-00-02:00-03"),
        ];
        for stream in storage_domain[0].cuc[0].stream.iter() {
            let i = streams
                .iter()
                .position(|x| *x == stream.stream_id)
                .expect("has to be in here");
            streams.remove(i);
        }
        assert_eq!(streams.len(), 0);
        drop(storage);
    }

    #[test]
    #[serial]
    fn remove_stream() {
        // Configuration for CNC
        let id: u32 = 0;
        let domain: String = String::from("test-domain-id");

        // create precondition
        insert_streams();

        // Create needed Components
        let northbound = MockRemoveStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_functioning();
        let scheduler = MockTSNScheduler::new();

        Cnc::run(
            id,
            domain.clone(),
            Arc::new(northbound),
            Arc::new(southbound),
            Arc::new(storage),
            Arc::new(topology),
            Arc::new(scheduler),
        );

        // check storage
        let storage = FileStorage::new();
        storage.configure_storage();
        let storage_domain = storage.get_streams_in_domain(Domain {
            domain_id: domain.clone(),
            cuc: vec![CucElement {
                cuc_id: String::from("test-cuc-id"),
                stream_list: Some(vec![String::from("00-00-00-00-00-01:00-01")]),
            }],
        });

        for stream in storage_domain[0].cuc[0].stream.iter() {
            // stream was removed
            assert!(stream.stream_id != String::from("00-00-00-00-00-01:00-01"));
        }
        drop(storage);
    }

    #[test]
    #[serial]
    fn compute_all() {
        // Configuration for CNC
        let id: u32 = 0;
        let domain: String = String::from("test-domain-id");

        // this inserts streams
        insert_streams();

        println!("precondition is established");

        // Create needed Components
        let northbound = MockComputeStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_functioning();
        let scheduler = MockTSNScheduler::new();

        Cnc::run(
            id,
            domain.clone(),
            Arc::new(northbound),
            Arc::new(southbound),
            Arc::new(storage),
            Arc::new(topology),
            Arc::new(scheduler),
        );

        println!("finished running the fake scheduler");

        // check storage for configuration
        // check storage for configured and attribs on streams
        // check configuration on switch

        // check storage
        let storage = FileStorage::new();
        storage.configure_storage();
        let storage_domain = storage.get_streams_in_domain(Domain {
            domain_id: domain.clone(),
            cuc: vec![CucElement {
                cuc_id: String::from("test-cuc-id"),
                stream_list: None,
            }],
        });

        for stream in storage_domain[0].cuc[0].stream.iter() {
            // check if stream is set as configured
            assert!(stream.stream_status == StreamStatus::Configured);

            // check for set latency values
            assert!(
                stream
                    .talker
                    .group_status_talker_listener
                    .accumulated_latency
                    != 0
            );
            for listener in stream.listener.iter() {
                assert!(listener.group_status_talker_listener.accumulated_latency != 0);
            }
        }

        println!("Check manualle if the configuration is set correctly on the switch.");
        println!(
            "This can be done by manually connecting via netconf and check the configuration."
        );
        println!("Since a Switch may have a GUI, this can be done by checking the GUI.");

        drop(storage);
    }

    #[test]
    #[serial]
    fn get_all_streams() {
        // create precondition
        insert_streams();

        let domain: String = String::from("test-domain-id");
        let storage = FileStorage::new();
        storage.configure_storage();
        let storage_domain = storage.get_streams_in_domain(Domain {
            domain_id: domain.clone(),
            cuc: vec![CucElement {
                cuc_id: String::from("test-cuc-id"),
                stream_list: None,
            }],
        });

        assert!(storage_domain.len() > 0);
        assert!(storage_domain[0].cuc.len() > 0);
        let streams = &storage_domain[0].cuc[0].stream;
        assert_eq!(streams.len(), 3);
        drop(storage);
    }

    #[test]
    fn lldp_neighbours() {
        let config = SSHConfigurationParams {
            username: String::from("admin"),
            password: String::from("admin"),
            ip: String::from("10.2.0.1"),
            port: 830,
        };

        let sb = NetconfAdapter::new();
        let neighbours = sb.retrieve_lldp(config);
        // atleast this machine should appear since we communicate with the switch.
        // this assumes this machine has lldpd installed
        assert!(neighbours.len() > 0);

        println!("Check manually if the neighbours are correct");
        for neighbour in neighbours.iter() {
            println!("{:?}", neighbour);
        }
    }

    #[test]
    fn port_capabilities() {
        let config = SSHConfigurationParams {
            username: String::from("admin"),
            password: String::from("admin"),
            ip: String::from("10.2.0.1"),
            port: 830,
        };

        let sb = NetconfAdapter::new();
        let delays = sb.retrieve_station_capibilities(config);
        assert!(delays.len() > 0);

        println!("Check manuall if the data is correct");
        for delay in delays.iter() {
            println!("{:?}", delay);
        }
    }
}
