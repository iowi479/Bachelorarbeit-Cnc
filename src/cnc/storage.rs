use super::cnc::Cnc;
use super::types::sched_types::ConfigurableGateParameterTableEntry;
use super::types::uni_types::{self, stream_request, Cuc, Stream, StreamStatus};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Read, Write};
use std::sync::{RwLock, Weak};

// TODO should probably be removed
const DEFAULT_CUC_ID: &str = "test-cuc-id";

/// Any StorageComponent that should be used with the CNC must implement this trait.
pub trait StorageAdapterInterface {
    /// This gets called when the CNC is created and linked via this.set_cnc_ref(...);
    /// This should fully setup everything the Storage-Component needs. After this is called, it has to be ready to operate.
    fn configure_storage(&self);

    // TODO get streams refactor for needing domain and cuc id
    fn get_all_streams(&self) -> Vec<Stream>;
    fn get_streams_in_domains(
        &self,
        domains: Vec<stream_request::Domain>,
    ) -> Vec<uni_types::Domain>;
    fn get_streams_in_domain(&self, domain: stream_request::Domain) -> Vec<uni_types::Domain>;
    fn get_stream(&self, id: String) -> Option<Stream>;
    fn get_planned_and_modified_streams_in_domains(
        &self,
        domains: Vec<stream_request::Domain>,
    ) -> Vec<uni_types::Domain>;

    fn remove_all_streams(&self, cuc_id: &String);
    fn remove_stream(&self, cuc_id: &String, stream_id: String);

    fn set_stream(&self, stream: Stream);
    fn set_streams(&self, streams: Vec<Stream>);
    fn set_streams_configured(&self, domains: &Vec<uni_types::Domain>);

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

    configs: RwLock<Vec<Config>>,
    cnc: Weak<Cnc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub node_id: u32,
    pub ports: Vec<PortConfiguration>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortConfiguration {
    pub name: String,
    pub config: ConfigurableGateParameterTableEntry,
}

impl FileStorage {
    pub fn new() -> Self {
        Self {
            domains_path: "domain_storage.json",
            configs_path: "config_storage.json",
            domains: RwLock::new(Vec::new()),
            configs: RwLock::new(Vec::new()),
            cnc: Weak::default(),
        }
    }

    fn save_domains(&self) {
        let parsing_res: Result<String, serde_json::Error> = serde_json::to_string(&self.domains);
        match parsing_res {
            Err(_) => panic!("[Storage] couldn't parse store to json..."),
            Ok(s) => {
                let result: Result<(), Error> = Self::write_to_file(self.domains_path, s.clone());
                if let Err(e) = result {
                    println!("[Storage] no existing file found... creating one {e:?}");
                    let result_creating: Result<(), Error> =
                        Self::create_and_write_to_file(self.domains_path, s);
                    if let Err(e) = result_creating {
                        println!("[Storage] error while creating file, {e:?}");
                        panic!("[Storage] not able to function without a file");
                    }
                }
            }
        }
    }

    fn save_configs(&self) {
        let parsing_res: Result<String, serde_json::Error> = serde_json::to_string(&self.configs);
        match parsing_res {
            Err(_) => panic!("[Storage] couldn't parse store to json..."),
            Ok(s) => {
                let result: Result<(), Error> = Self::write_to_file(self.configs_path, s.clone());
                if let Err(e) = result {
                    println!("[Storage] no existing file found... creating one {e:?}");
                    let result_creating: Result<(), Error> =
                        Self::create_and_write_to_file(self.configs_path, s);
                    if let Err(e) = result_creating {
                        println!("[Storage] error while creating file, {e:?}");
                        panic!("[Storage] not able to function without a file");
                    }
                }
            }
        }
    }

    fn write_to_file(file_path: &str, content: String) -> Result<(), Error> {
        let mut file: File = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Result::Ok(())
    }

    fn create_and_write_to_file(file_path: &str, content: String) -> Result<(), Error> {
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
        let configs: Vec<Config> = serde_json::from_str::<Vec<Config>>(&content)?;
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
            let cnc = self.cnc.upgrade().unwrap();
            let cnc_domain: String = cnc.domain.clone();
            let mut domain_lock = self.domains.write().unwrap();

            // generate Mockdata
            domain_lock.push(uni_types::Domain {
                domain_id: cnc_domain,
                cnc_enabled: true,
                cuc: Vec::new(),
            });

            // TODO Maybe do this on receiving change?
            domain_lock[0].cuc.push(Cuc {
                cuc_id: DEFAULT_CUC_ID.to_string(),
                stream: Vec::new(),
            });

            drop(domain_lock);

            self.save_domains();
        }

        let could_load_configs = self.try_load_configs();

        if could_load_configs.is_err() {
            // no configurations could be loaded
            let mut configs_lock = self.configs.write().unwrap();
            *configs_lock = Vec::new();
            drop(configs_lock);

            self.save_configs();
        }
    }

    /// remove all streams from a given cuc in the local cnc.domain
    fn remove_all_streams(&self, cuc_id: &String) {
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

        cuc.stream.clear();
        drop(domain_lock);

        self.save_domains();
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

    fn get_streams_in_domain(&self, get_domain: stream_request::Domain) -> Vec<uni_types::Domain> {
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

    fn get_all_streams(&self) -> Vec<Stream> {
        let mut result: Vec<Stream> = Vec::new();
        let domain_lock = self.domains.write().unwrap();
        for stream in &domain_lock[0].cuc[0].stream {
            result.push(stream.clone());
        }

        drop(domain_lock);
        result
    }

    fn get_stream(&self, id: String) -> Option<Stream> {
        let domain_lock = self.domains.write().unwrap();
        for stream in domain_lock[0].cuc[0].stream.iter() {
            if stream.stream_id == id {
                return Some(stream.clone());
            }
        }

        drop(domain_lock);
        return None;
    }

    fn set_stream(&self, stream: Stream) {
        let mut domain_lock = self.domains.write().unwrap();
        let streams: &Vec<Stream> = &domain_lock[0].cuc[0].stream;

        if let Some(index) = streams.iter().position(|s| s.stream_id == stream.stream_id) {
            domain_lock[0].cuc[0].stream[index] = stream;
        } else {
            domain_lock[0].cuc[0].stream.push(stream);
        }

        drop(domain_lock);
        self.save_domains();
    }

    fn set_streams(&self, mut streams: Vec<Stream>) {
        while let Some(stream) = streams.pop() {
            self.set_stream(stream);
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

    fn get_all_configs(&self) -> Vec<Config> {
        let config_lock = self.configs.write().unwrap();
        let mut result: Vec<Config> = Vec::new();
        for config in config_lock.iter() {
            result.push(config.clone());
        }

        drop(config_lock);
        result
    }

    fn get_config(&self, node_id: u32) -> Option<Config> {
        let config_lock = self.configs.write().unwrap();
        for config in config_lock.iter() {
            if config.node_id == node_id {
                return Some(config.clone());
            }
        }

        drop(config_lock);
        None
    }

    fn set_config(&self, config: Config) {
        let mut config_lock = self.configs.write().unwrap();
        for i in 0..config_lock.len() {
            if config_lock[i].node_id == config.node_id {
                config_lock[i] = config;
                self.save_configs();
                return;
            }
        }

        // id not yet present
        config_lock.push(config);

        drop(config_lock);
        self.save_configs();
    }

    fn set_configs(&self, configs: &Vec<Config>) {
        for config in configs.iter() {
            self.set_config(config.clone());
        }
    }

    /// goes through all domains and retures all with all subsecuent cucs and all their streams
    fn get_streams_in_domains(
        &self,
        domains: Vec<stream_request::Domain>,
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
        domains: Vec<stream_request::Domain>,
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

        result
    }

    /// sets StreamStatus to Configured on all provided streams
    fn set_streams_configured(&self, domains: &Vec<uni_types::Domain>) {
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
                                            stream.stream_status = StreamStatus::Configured;
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
}
