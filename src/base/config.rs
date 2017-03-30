extern crate ini;

use std::collections::HashMap;
use self::ini::Ini;

#[derive(Debug)]
pub struct Config {
    timeout: u64
}


impl Config {
    pub fn new(config_file: &str) -> Result<Config, &str> {
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
        let t_out: u64 = config.get("daemon")
            .unwrap()
            .get("worker-loop-timeout")
            .unwrap()
            .parse()
            .unwrap();
        if false {
            Err("To implement up fold")
        } else {
            Ok(Config { timeout: t_out })
        }
    }


    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }
}


