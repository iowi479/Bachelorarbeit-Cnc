use super::types::scheduling::Config;
use super::types::uni_types::{self, compute_streams, Cuc, Stream, StreamStatus};
use super::types::FailedInterfaces;
use super::{Cnc, CNC_NOT_PRESENT};
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::sync::{RwLock, Weak};

/// Any StorageComponent that should be used with the CNC must implement this trait.
pub trait StorageAdapterInterface {
    /// This gets called when the CNC is created and linked via this.set_cnc_ref(...);
    /// This should fully setup everything the Storage-Component needs. After this is called, it has to be ready to operate.
    fn configure_storage(&self);

    fn get_streams_in_domains(
        &self,
        domains: Vec<compute_streams::Domain>,
    ) -> Vec<uni_types::Domain>;
    fn get_streams_in_domain(&self, domain: compute_streams::Domain) -> Vec<uni_types::Domain>;
    fn get_planned_and_modified_streams_in_domains(
        &self,
        domains: Vec<compute_streams::Domain>,
    ) -> Vec<uni_types::Domain>;

    fn remove_all_streams(&self, cuc_id: &String);
    fn remove_stream(&self, cuc_id: &String, stream_id: String);

    fn set_stream(&self, cuc_id: &String, stream: &Stream);
    fn set_streams(&self, cuc_id: &String, streams: &Vec<Stream>);

    fn modify_streams(&self, domains: &Vec<uni_types::Domain>);

    /// This gets called after the configuration of the requested Streams was successfull.
    fn set_streams_configured(
        &self,
        domains: &Vec<uni_types::Domain>,
        failed_interfaces: &FailedInterfaces,
    );

    /// Returns the domain of the requesting CUC
    /// If the domain or cuc_id could not be found: returns None
    fn get_domain_id_of_cuc(&self, cuc_id: String) -> Option<String>;

    fn get_all_configs(&self) -> Vec<Config>;
    fn get_config(&self, node_id: u32) -> Option<Config>;

    fn set_config(&self, config: Config);
    fn set_configs(&self, configs: &Vec<Config>);

    /// In the fully centralized model, this should not be used.
    /// The CUC should take care of that because it nows the MAC-Addresses of its listeners.
    /// This implementation returns a free id but with MAC-Address 0
    ///
    /// # Examples
    ///
    /// stream_id: 00-00-00-00-00-00:7A-6E
    ///
    /// stream_id: 00-00-00-00-00-00:11-22
    fn get_free_stream_id(&self, domain_id: String, cuc_id: String) -> Option<String>;

    /// # CNC Configuration
    /// Minimum requirement:
    /// ```
    /// self.cnc = cnc;
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>);
}

pub struct FileStorage {
    domains_path: &'static str,
    configs_path: &'static str,

    domains: RwLock<Vec<uni_types::Domain>>,

    configs: RwLock<HashMap<u32, Config>>,
    cnc: Weak<Cnc>,
}

impl FileStorage {
    pub fn new() -> Self {
        Self {
            domains_path: "domain_storage.json",
            configs_path: "config_storage.json",
            domains: RwLock::new(Vec::new()),
            configs: RwLock::new(HashMap::new()),
            cnc: Weak::default(),
        }
    }

    fn save_domains(&self) {
        match serde_json::to_string(&self.domains) {
            Err(_) => panic!("[Storage] couldn't parse store to json..."),
            Ok(s) => {
                let result: Result<(), Error> = Self::write_to_file(self.domains_path, s.clone());
                if let Err(e) = result {
                    println!("[Storage] error while creating file, {e:?}");
                    panic!("[Storage] not able to function without a file");
                }
            }
        }
    }

    fn save_configs(&self) {
        match serde_json::to_string(&self.configs) {
            Err(_) => panic!("[Storage] couldn't parse store to json..."),
            Ok(s) => {
                let result: Result<(), Error> = Self::write_to_file(self.configs_path, s.clone());
                if let Err(e) = result {
                    println!("[Storage] error while creating file, {e:?}");
                    panic!("[Storage] not able to function without a file");
                }
            }
        }
    }

    fn write_to_file(file_path: &str, content: String) -> Result<(), Error> {
        let mut file: File = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Result::Ok(())
    }

    fn read_from_file(file_path: &str) -> Result<String, Error> {
        let mut file: File = File::open(file_path)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;
        Result::Ok(content)
    }

    fn try_load_domains(&self) -> Result<(), Error> {
        let content: String = Self::read_from_file(self.domains_path)?;
        let domains: Vec<uni_types::Domain> =
            serde_json::from_str::<Vec<uni_types::Domain>>(&content)?;
        let mut domains_lock = self.domains.write().unwrap();
        *domains_lock = domains;
        drop(domains_lock);
        println!("[Storage] Successfully loaded domains");
        return Result::Ok(());
    }

    fn try_load_configs(&self) -> Result<(), Error> {
        let content: String = Self::read_from_file(self.configs_path)?;
        let configs: HashMap<u32, Config> = serde_json::from_str::<HashMap<u32, Config>>(&content)?;
        let mut config_lock = self.configs.write().unwrap();
        *config_lock = configs;
        drop(config_lock);
        println!("[Storage] Successfully loaded configurations");
        return Result::Ok(());
    }

    fn random_stream_id() -> String {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let a: u8 = rng.gen_range(0..255);
        let b: u8 = rng.gen_range(0..255);

        format!("00-00-00-00-00-00:{:02X}-{:02X}", a, b)
    }
}

impl StorageAdapterInterface for FileStorage {
    fn configure_storage(&self) {
        let could_load_domains = self.try_load_domains();

        if could_load_domains.is_err() {
            let cnc = self.cnc.upgrade().expect(CNC_NOT_PRESENT);
            let cnc_domain: String = cnc.domain.clone();
            let mut domain_lock = self.domains.write().unwrap();

            // generate empty domain of cnc
            domain_lock.push(uni_types::Domain {
                domain_id: cnc_domain,
                cnc_enabled: true,
                cuc: Vec::new(),
            });

            drop(domain_lock);

            self.save_domains();
        }

        let could_load_configs = self.try_load_configs();

        if could_load_configs.is_err() {
            // no configurations could be loaded
            let mut configs_lock = self.configs.write().unwrap();
            *configs_lock = HashMap::new();
            drop(configs_lock);

            self.save_configs();
        }
    }

    /// remove all streams from a given cuc in the local cnc.domain
    fn remove_all_streams(&self, cuc_id: &String) {
        let mut domain_lock = self.domains.write().unwrap();
        let cnc_domain_name = &self.cnc.upgrade().expect(CNC_NOT_PRESENT).domain;

        let domain = domain_lock
            .iter_mut()
            .find(|d| d.domain_id == *cnc_domain_name);

        if let Some(domain) = domain {
            let cuc = domain.cuc.iter_mut().find(|c| &c.cuc_id == cuc_id);

            if let Some(cuc) = cuc {
                cuc.stream.clear();
                drop(domain_lock);

                self.save_domains();
            }
        }
    }

    /// remove a single stream by id from a given cuc in the local cnc.domain
    fn remove_stream(&self, cuc_id: &String, stream_id: String) {
        let mut domain_lock = self.domains.write().unwrap();

        let domain = domain_lock
            .iter_mut()
            .find(|d| d.domain_id == self.cnc.upgrade().unwrap().domain);

        if domain.is_none() {
            return;
        }

        let domain = domain.unwrap();
        let cuc = domain.cuc.iter_mut().find(|c| &c.cuc_id == cuc_id);

        if cuc.is_none() {
            return;
        }
        let cuc = cuc.unwrap();

        if let Some(index) = cuc.stream.iter().position(|s| s.stream_id == stream_id) {
            domain_lock[0].cuc[0].stream.remove(index);
            drop(domain_lock);

            self.save_domains();
        }
    }

    /// get streams of a single cuc in the domain
    fn get_streams_in_domain(&self, get_domain: compute_streams::Domain) -> Vec<uni_types::Domain> {
        let domain_lock = self.domains.write().unwrap();
        let mut result: Vec<uni_types::Domain> = Vec::new();

        for domain in domain_lock.iter() {
            if domain.domain_id == get_domain.domain_id {
                let mut result_domain = uni_types::Domain {
                    domain_id: domain.domain_id.clone(),
                    cnc_enabled: domain.cnc_enabled.clone(),
                    cuc: Vec::new(),
                };

                for cuc in domain.cuc.iter() {
                    if cuc.cuc_id == get_domain.cuc[0].cuc_id {
                        result_domain.cuc.push(cuc.clone());
                    }
                }

                result.push(result_domain);
            }
        }
        drop(domain_lock);
        return result;
    }

    /// if the provided stream is already present, it will get replaced. Otherwise it will be added to the streamlist of the provided cuc
    fn set_stream(&self, cuc_id: &String, stream: &Stream) {
        let mut domain_lock = self.domains.write().unwrap();
        let cnc_domain_name = &self.cnc.upgrade().expect(CNC_NOT_PRESENT).domain;

        let domain = domain_lock
            .iter_mut()
            .find(|d| d.domain_id == *cnc_domain_name);

        if let Some(domain) = domain {
            let cuc = domain.cuc.iter_mut().find(|c| c.cuc_id == *cuc_id);

            if let Some(cuc) = cuc {
                let found_stream = cuc
                    .stream
                    .iter_mut()
                    .find(|s| s.stream_id == stream.stream_id);

                if let Some(s) = found_stream {
                    *s = stream.clone();
                    s.stream_status = StreamStatus::Modified;
                } else {
                    cuc.stream.push(stream.clone());
                }
            } else {
                domain.cuc.push(Cuc {
                    cuc_id: cuc_id.clone(),
                    stream: vec![stream.clone()],
                })
            }
        }

        drop(domain_lock);
        self.save_domains();
    }

    /// calls self.set_stream(...) for every provided stream. This is not really efficient...
    fn set_streams(&self, cuc_id: &String, streams: &Vec<Stream>) {
        for s in streams {
            self.set_stream(cuc_id, s);
        }
    }

    fn get_domain_id_of_cuc(&self, cuc_id: String) -> Option<String> {
        for domain in self.domains.read().unwrap().iter() {
            let res: Option<_> = domain.cuc.iter().find(|cuc| cuc.cuc_id == cuc_id);
            if res.is_some() {
                return Some(domain.domain_id.clone());
            }
        }
        return None;
    }

    fn get_free_stream_id(&self, _domain_id: String, _cuc_id: String) -> Option<String> {
        let id: String = Self::random_stream_id();
        'outer: loop {
            for domain in self.domains.read().unwrap().iter() {
                for cuc in domain.cuc.iter() {
                    for stream in cuc.stream.iter() {
                        if stream.stream_id == id {
                            continue 'outer;
                        }
                    }
                }
            }
            return Some(id);
        }
    }

    fn set_cnc_ref(&mut self, cnc: Weak<Cnc>) {
        self.cnc = cnc;
    }

    /// returnes the whole config store. Maybe usefull for configuration on startup...
    fn get_all_configs(&self) -> Vec<Config> {
        let config_lock = self.configs.write().unwrap();
        let mut result: Vec<Config> = Vec::new();

        for config in config_lock.values() {
            result.push(config.clone());
        }

        drop(config_lock);
        result
    }

    /// returnes the requested config. If it is not present, this will return None
    fn get_config(&self, node_id: u32) -> Option<Config> {
        let config_lock = self.configs.write().unwrap();

        return match config_lock.get(&node_id) {
            None => None,
            Some(config) => Some(config.clone()),
        };
    }

    /// stores the provided config
    fn set_config(&self, config: Config) {
        let mut config_lock = self.configs.write().unwrap();

        config_lock.insert(config.node_id, config);

        drop(config_lock);
        self.save_configs();
    }

    /// stores all provided configs
    fn set_configs(&self, configs: &Vec<Config>) {
        let mut config_lock = self.configs.write().unwrap();

        for config in configs.iter() {
            let config = config.clone();
            config_lock.insert(config.node_id, config);
        }

        drop(config_lock);
        self.save_configs();
    }

    /// goes through all domains and retures all with all subsecuent cucs and all their streams
    fn get_streams_in_domains(
        &self,
        domains: Vec<compute_streams::Domain>,
    ) -> Vec<uni_types::Domain> {
        let mut result: Vec<uni_types::Domain> = Vec::new();

        for req_domain in domains.iter() {
            for domain in self.domains.read().unwrap().iter() {
                if req_domain.domain_id == domain.domain_id {
                    let mut domain_copy = uni_types::Domain {
                        domain_id: domain.domain_id.clone(),
                        cnc_enabled: domain.cnc_enabled,
                        cuc: Vec::new(),
                    };

                    for req_cuc in req_domain.cuc.iter() {
                        for cuc in domain.cuc.iter() {
                            if req_cuc.cuc_id == cuc.cuc_id {
                                domain_copy.cuc.push(cuc.clone());
                            }
                        }
                    }

                    result.push(domain_copy);
                }
            }
        }

        result
    }

    /// goes through all domains and retures all with all subsecuent cucs and their planned or modified streams
    fn get_planned_and_modified_streams_in_domains(
        &self,
        domains: Vec<compute_streams::Domain>,
    ) -> Vec<uni_types::Domain> {
        let domain_lock = self.domains.read().unwrap();
        let mut result: Vec<uni_types::Domain> = Vec::new();

        for req_domain in domains.iter() {
            for domain in domain_lock.iter() {
                if req_domain.domain_id == domain.domain_id {
                    let mut domain_copy = uni_types::Domain {
                        domain_id: domain.domain_id.clone(),
                        cnc_enabled: domain.cnc_enabled,
                        cuc: Vec::new(),
                    };

                    for req_cuc in req_domain.cuc.iter() {
                        for cuc in domain.cuc.iter() {
                            if req_cuc.cuc_id == cuc.cuc_id {
                                let mut cuc_copy = uni_types::Cuc {
                                    cuc_id: cuc.cuc_id.clone(),
                                    stream: Vec::new(),
                                };

                                for stream in cuc.stream.iter() {
                                    if stream.stream_status == StreamStatus::Planned
                                        || stream.stream_status == StreamStatus::Modified
                                    {
                                        cuc_copy.stream.push(stream.clone());
                                    }
                                }

                                domain_copy.cuc.push(cuc_copy);
                            }
                        }
                    }

                    result.push(domain_copy);
                }
            }
        }

        drop(domain_lock);

        result
    }

    /// sets StreamStatus to Configured on all provided streams. Although if the stream_id is also in the failed_streams it gets set to planned instead.
    fn set_streams_configured(
        &self,
        domains: &Vec<uni_types::Domain>,
        failed_interfaces: &FailedInterfaces,
    ) {
        // gets all streams that failed
        let failed_stream_ids: HashSet<String> =
            failed_interfaces
                .interfaces
                .iter()
                .fold(HashSet::new(), |mut acc, fi| {
                    fi.affected_streams.iter().for_each(|id| {
                        acc.insert(id.clone());
                    });
                    acc
                });

        // apply changes
        let mut domain_lock = self.domains.write().unwrap();
        for change_domain in domains.iter() {
            for domain in domain_lock.iter_mut() {
                if domain.domain_id == change_domain.domain_id {
                    for change_cuc in change_domain.cuc.iter() {
                        for cuc in domain.cuc.iter_mut() {
                            if cuc.cuc_id == change_cuc.cuc_id {
                                for change_stream in change_cuc.stream.iter() {
                                    for stream in cuc.stream.iter_mut() {
                                        if stream.stream_id == change_stream.stream_id {
                                            if failed_stream_ids.get(&stream.stream_id).is_none() {
                                                stream.stream_status = StreamStatus::Configured;
                                            } else {
                                                stream.stream_status = StreamStatus::Modified;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        drop(domain_lock);
        self.save_domains();
    }

    fn modify_streams(&self, domains: &Vec<uni_types::Domain>) {
        let mut domain_lock = self.domains.write().unwrap();
        for change_domain in domains.iter() {
            for domain in domain_lock.iter_mut() {
                if domain.domain_id == change_domain.domain_id {
                    for change_cuc in change_domain.cuc.iter() {
                        for cuc in domain.cuc.iter_mut() {
                            if cuc.cuc_id == change_cuc.cuc_id {
                                for change_stream in change_cuc.stream.iter() {
                                    for stream in cuc.stream.iter_mut() {
                                        if stream.stream_id == change_stream.stream_id {
                                            // replace old stream
                                            *stream = change_stream.clone();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        drop(domain_lock);
        self.save_domains();
    }
}
