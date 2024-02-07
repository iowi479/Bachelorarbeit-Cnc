pub mod cnc;

#[cfg(test)]
mod tests {
    use crate::cnc::northbound::{MockInsertStreamAdapter, MockRemoveStreamAdapter};
    use crate::cnc::scheduling::MockTSNScheduler;
    use crate::cnc::southbound::NetconfAdapter;
    use crate::cnc::storage::{FileStorage, StorageAdapterInterface};
    use crate::cnc::topology::MockTopology;
    use crate::cnc::types::uni_types::compute_streams::{CucElement, Domain};
    use crate::cnc::Cnc;
    use std::sync::Arc;

    #[test]
    fn test_insert_stream() {
        // Configuration for CNC
        let id: u32 = 0;
        let domain: String = String::from("test-domain-id");

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

        assert_eq!(storage_domain[0].cuc[0].stream.len(), 0);
        drop(storage);

        // Create needed Components
        let northbound = MockInsertStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_failing();
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

        let mut stream_contained = vec![
            String::from("00-00-00-00-00-01:00-01"),
            String::from("00-00-00-00-00-01:00-02"),
            String::from("00-00-00-00-00-02:00-03"),
        ];
        for stream in storage_domain[0].cuc[0].stream.iter() {
            let i = stream_contained
                .iter()
                .position(|x| *x == stream.stream_id)
                .expect("has to be in here");
            stream_contained.remove(i);
        }
        assert_eq!(stream_contained.len(), 0);
        drop(storage);
    }

    #[test]
    fn test_remove_stream() {
        // Configuration for CNC
        let id: u32 = 0;
        let domain: String = String::from("test-domain-id");

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

        let mut stream_contained = false;
        for stream in storage_domain[0].cuc[0].stream.iter() {
            if stream.stream_id == String::from("00-00-00-00-00-01:00-01") {
                stream_contained = true;
            }
        }
        assert!(stream_contained, "stream to remove is not present");
        drop(storage);

        // Create needed Components
        let northbound = MockRemoveStreamAdapter::new(String::from("test-cuc-id"));
        let southbound = NetconfAdapter::new();
        let storage = FileStorage::new();
        let topology = MockTopology::new_failing();
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
    fn test_compute_all() {
        // TODO add test
        assert_eq!(true, true)
    }

    #[test]
    fn test_get_all() {
        // TODO does this work?
        test_insert_stream();
        // now the storage is filled with streams

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
}
