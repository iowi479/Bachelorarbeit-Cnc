use super::tsntypes::shed_types::ConfigurableGateParameterTableEntry;
use super::tsntypes::uni_types::{Cuc, Domain, Stream};
use super::Cnc;
use rand::Rng;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::sync::{RwLock, Weak};

// TODO should probably be removed
const DEFAULT_CUC_ID: &str = "test-cuc-id";

// TODO propably not needed since Storagecomponent doesnt need to access CNC
pub trait StorageControllerInterface {}

/// Any StorageComponent that should be used with the CNC must implement this trait.
pub trait StorageAdapterInterface {
    /// This gets called when the CNC is created and linked via this.set_cnc_ref(...);
    /// This should fully setup everything the Storage-Component needs. After this is called, it has to be ready to operate.
    fn configure_storage(&mut self);

    fn get_all_streams(&self) -> Vec<&Stream>;
    fn get_streams(&self, ids: Vec<String>) -> Vec<&Stream>;
    fn get_stream(&self, id: String) -> Option<&Stream>;

    fn clear_all_streams(&mut self);
    fn remove_streams(&mut self, ids: Vec<String>);
    fn remove_stream(&mut self, id: String);

    fn set_stream(&mut self, stream: Stream);
    fn set_streams(&mut self, streams: Vec<Stream>);

    /// Returns the domain of the requesting CUC
    /// If the domain or cuc_id could not be found: returns None
    fn get_domain_id_of_cuc(&self, cuc_id: String) -> Option<String>;

    fn get_all_configs(&self) -> Vec<&Config>;
    fn get_config(&self, node_id: u32) -> Option<&Config>;

    fn set_config(&mut self, config: Config);
    fn set_configs(&mut self, configs: Vec<Config>);
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
    /// self.cnc = Some(cnc);
    /// ```
    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>);
}

pub struct FileStorage {
    domains_path: &'static str,
    configs_path: &'static str,

    domains: Vec<Domain>,

    // TODO which types for tas-configuration?
    configs: Vec<Config>,
    cnc: Option<Weak<RwLock<Cnc>>>, // ref to cnc
}

type Config = (u32, ConfigurableGateParameterTableEntry);

impl FileStorage {
    pub fn new() -> Self {
        Self {
            domains_path: "domains.json",
            configs_path: "configs.json",
            domains: Vec::new(),
            configs: Vec::new(),
            cnc: None,
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

    fn try_load_domains(&mut self) -> Result<(), Error> {
        let content: String = Self::read_from_file(self.domains_path)?;
        let domains: Vec<Domain> = serde_json::from_str::<Vec<Domain>>(&content)?;
        self.domains = domains;
        println!("[Storage] Successfully loaded domains");
        return Result::Ok(());
    }

    fn try_load_configs(&mut self) -> Result<(), Error> {
        let content: String = Self::read_from_file(self.configs_path)?;
        let configs: Vec<Config> = serde_json::from_str::<Vec<Config>>(&content)?;
        self.configs = configs;
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
    fn configure_storage(&mut self) {
        let could_load_domains = self.try_load_domains();

        if could_load_domains.is_err() {
            // generate Mockdata
            self.domains.push(Domain {
                domain_id: self
                    .cnc
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .read()
                    .unwrap()
                    .domain
                    .clone(),
                cnc_enabled: true,
                cuc: Vec::new(),
            });

            // TODO Maybe do this on receiving change?
            self.domains[0].cuc.push(Cuc {
                cuc_id: DEFAULT_CUC_ID.to_string(),
                stream: Vec::new(),
            });

            self.save_domains();
        }

        let could_load_configs = self.try_load_configs();

        if could_load_configs.is_err() {
            // no configurations could be loaded
            self.configs = Vec::new();
            self.save_domains();
        }
    }

    fn clear_all_streams(&mut self) {
        self.domains[0].cuc[0].stream.clear();
        self.save_domains();
    }

    fn remove_stream(&mut self, id: String) {
        let streams: &Vec<Stream> = &self.domains[0].cuc[0].stream;

        if let Some(index) = streams.iter().position(|s| s.stream_id == id) {
            self.domains[0].cuc[0].stream.remove(index);
            self.save_domains();
        } else {
            // TODO decide what to do on failure
            println!(
                "[Storage] Tried to remove stream which doesnt exist: {}",
                id
            );
        }
    }

    fn remove_streams(&mut self, ids: Vec<String>) {
        for stream_id in ids {
            self.remove_stream(stream_id);
        }
    }

    fn get_streams(&self, ids: Vec<String>) -> Vec<&Stream> {
        let mut result: Vec<&Stream> = Vec::new();

        for stream_id in ids {
            if let Some(stream) = self.get_stream(stream_id.clone()) {
                result.push(stream);
            } else {
                println!(
                    "[Storage] Tried to request stream which doesnt exist: {}",
                    stream_id
                );
            }
        }

        result
    }

    fn get_all_streams(&self) -> Vec<&Stream> {
        let mut result: Vec<&Stream> = Vec::new();

        for stream in &self.domains[0].cuc[0].stream {
            result.push(stream);
        }
        result
    }

    fn get_stream(&self, id: String) -> Option<&Stream> {
        for stream in self.domains[0].cuc[0].stream.iter() {
            if stream.stream_id == id {
                return Some(stream);
            }
        }
        return None;
    }

    fn set_stream(&mut self, stream: Stream) {
        let streams: &Vec<Stream> = &self.domains[0].cuc[0].stream;

        if let Some(index) = streams.iter().position(|s| s.stream_id == stream.stream_id) {
            self.domains[0].cuc[0].stream[index] = stream;
        } else {
            self.domains[0].cuc[0].stream.push(stream);
        }

        self.save_domains();
    }

    fn set_streams(&mut self, mut streams: Vec<Stream>) {
        while let Some(stream) = streams.pop() {
            self.set_stream(stream);
        }
    }

    fn get_domain_id_of_cuc(&self, cuc_id: String) -> Option<String> {
        for domain in self.domains.iter() {
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
            for domain in self.domains.iter() {
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

    fn set_cnc_ref(&mut self, cnc: Weak<RwLock<Cnc>>) {
        self.cnc = Some(cnc);
    }

    fn get_all_configs(&self) -> Vec<&Config> {
        let mut result: Vec<&Config> = Vec::new();
        for config in &self.configs {
            result.push(config);
        }
        result
    }

    fn get_config(&self, node_id: u32) -> Option<&Config> {
        for config in &self.configs {
            if config.0 == node_id {
                return Some(config);
            }
        }
        None
    }

    fn set_config(&mut self, config: Config) {
        for i in 0..self.configs.len() {
            if self.configs[i].0 == config.0 {
                self.configs[i] = config;
                self.save_configs();
                return;
            }
        }

        // id not yet present
        self.configs.push(config);
        self.save_configs();
    }

    fn set_configs(&mut self, configs: Vec<Config>) {
        for config in configs {
            self.set_config(config);
        }
    }
}
