extern crate ini;

use std::collections::HashMap;
use self::ini::Ini;

#[derive(Debug)]
pub struct Config {
    timeout: u64
}

impl Config {
    pub fn new(config_file: &str) -> Result<Config, String> {
        let i = Ini::load_from_file(config_file).unwrap();

        let mut config = HashMap::new();
        let general_section_name = "__General__".into();
        for (sec, prop) in i.iter() {
            let mut section_contents = HashMap::new();
            for (k, v) in prop.iter() {
                section_contents.insert(k.clone(), v.clone());
            }
            let section_name = sec.as_ref().unwrap_or(&general_section_name).clone();
            config.insert(section_name, section_contents);
        }

        let t_out = config.get("daemon")
            .ok_or("no daemon section in config file".to_string())
            .and_then(
                |arg| arg.get("worker-loop-timeout")
                    .ok_or("no worker-loop-timeout param in daemon section".to_string())
                    .and_then(
                        |arg| arg.parse::<u64>()
                            .map_err(|err| err.to_string())
                    )
            );
        match t_out {
            Ok(n) => {
                Ok(Config { timeout: n })
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
}


