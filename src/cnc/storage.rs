use super::tsntypes::uni_types::{Cuc, Domain, Stream};
use super::Cnc;
use rand::Rng;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::{cell::RefCell, sync::Weak};

// FileStorage specific constants
// TODO should probably be removed
const DEFAULT_CUC_ID: &str = "test-cuc-id";

// TODO propably not needed since Storagecomponent doesnt need to access CNC
pub trait StorageControllerInterface {
    fn get_cnc_domain_id(&self) -> String;
}

pub trait StorageAdapterInterface {
    fn configure_storage(&mut self);

    fn get_all_streams(&mut self) -> &Vec<Stream>;
    fn get_stream(&mut self, id: String) -> Option<&Stream>;
    fn clear_all_streams(&mut self);
    fn remove_stream(&mut self, id: String);

    fn set_stream(&mut self, stream: Stream);
    fn set_streams(&mut self, streams: Vec<Stream>);

    fn get_domain_id_of_cuc(&self, cuc_id: String) -> Option<String>;
    fn get_free_stream_id(&self, domain_id: String, cuc_id: String) -> Option<String>;

    // CNC Configuration
    fn set_cnc_ref(&mut self, cnc: Weak<RefCell<Cnc>>);
}

pub struct FileStorage {
    domains: Vec<Domain>,

    // TODO which types for tas-configuration?
    configs: Vec<u32>,
    cnc: Option<Weak<RefCell<Cnc>>>, // ref to cnc
}

impl FileStorage {
    pub fn new() -> Self {
        Self {
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
                println!("{}", s);
                let file_path: &str = "domains.json";
                let result: Result<(), Error> = Self::write_to_file(file_path, s.clone());
                if let Err(_) = result {
                    println!("[Storage] no existing file found... creating one");
                    let result_creating: Result<(), Error> =
                        Self::create_and_write_to_file(file_path, s);
                    if let Err(e) = result_creating {
                        println!("[Storage] error while creating file, {}", e);
                        panic!("[Storage] not able to function without a file");
                    }
                }
            }
        }
    }

    fn write_to_file(file_path: &str, content: String) -> Result<(), Error> {
        let mut file: File = File::open(file_path)?;
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
        let file_path: &str = "domains.json";
        let content: String = Self::read_from_file(file_path)?;
        let domains: Vec<Domain> = serde_json::from_str::<Vec<Domain>>(&content)?;
        self.domains = domains;
        println!("[Storage] Successfully loaded domain configuration");
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
        let could_load = self.try_load_domains();

        if could_load.is_err() {
            // generate Mockdata
            self.domains.push(Domain {
                domain_id: self
                    .cnc
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .get_cnc_domain_id(),
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

    fn get_all_streams(&mut self) -> &Vec<Stream> {
        &self.domains[0].cuc[0].stream
    }

    fn get_stream(&mut self, id: String) -> Option<&Stream> {
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

        // Stream id unique per domain/cuc or global for the cnc
        //
        // for domain in self.domains.iter() {
        //     if domain.domain_id == domain_id {
        //         for cuc in domain.cuc.iter() {
        //             if cuc.cuc_id == cuc_id {
        //                 'randid: loop {
        //                     let id: String = Self::random_stream_id();
        //                     for stream in cuc.stream.iter() {
        //                         if stream.stream_id == id {
        //                             continue 'randid;
        //                         }
        //                     }
        //                     return Some(id);
        //                 }
        //             }
        //         }
        //     }
        // }
        // // random id
        // Some(Self::random_stream_id())
    }

    fn set_cnc_ref(&mut self, cnc: Weak<RefCell<Cnc>>) {
        self.cnc = Some(cnc);
    }
}
