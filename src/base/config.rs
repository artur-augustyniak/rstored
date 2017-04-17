extern crate ini;

use std::collections::HashMap;
use self::ini::Ini;

#[derive(Debug)]
pub struct Config {
    timeout: u64
}

impl Config {
    pub fn new(config_file: &str) -> Result<Config, String> {
        match Ini::load_from_file(config_file) {
            Ok(ini) => {
                let mut config = HashMap::new();
                let general_section_name = "__General__".into();
                for (sec, prop) in ini.iter() {
                    let mut section_contents = HashMap::new();
                    for (k, v) in prop.iter() {
                        section_contents.insert(k.clone(), v.clone());
                    }
                    let section_name = sec.as_ref().unwrap_or(&general_section_name).clone();
                    config.insert(section_name, section_contents);
                }

                config.get("daemon")
                    .ok_or("no daemon section in config file".to_string())
                    .and_then(|section| section.get("worker-loop-timeout")
                        .ok_or("no worker-loop-timeout param in daemon section".to_string()))
                    .and_then(|section_param| section_param.parse::<u64>().map_err(|err| err.to_string()))
                    .and_then(|timeout_converted| Ok(Config { timeout: timeout_converted }))
            }

            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
}


