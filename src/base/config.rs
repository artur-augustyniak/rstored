extern crate ini;

use self::ini::Ini;
use self::ini::ini::Error as IniError;
use std::path::Path;
use std::error::Error;
use std::num::ParseIntError;
use std::fs;

#[derive(Debug)]
pub struct ConfigError {
    pub msg: String
}

impl From<IniError> for ConfigError {
    fn from(e: IniError) -> ConfigError {
        ConfigError { msg: e.msg }
    }
}


impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> ConfigError {
        ConfigError { msg: e.description().to_string() }
    }
}


#[derive(Debug)]
pub struct Config {
    timeout: u64,
    probes_folder_path: String
}

impl Config {
    pub fn new<P: AsRef<Path>>(config_file: P) -> Result<Config, ConfigError> {
        let ini = Ini::load_from_file(config_file)?;

        let timeout_converted = ini.get_from(Some("daemon"), "worker-loop-timeout")
            .ok_or(
                ConfigError { msg: "worker-loop-timeout err".to_string() }
            )
            .and_then(|v| v.parse::<u64>()
                .map_err(
                    |err| ConfigError::from(err)
                )
            )?;

        let probes_path = ini.get_from(Some("daemon"), "probes-folder-path")
            .ok_or(
                ConfigError { msg: "probes-folder-path err".to_string() }
            )?;

        Ok(Config { timeout: timeout_converted, probes_folder_path: probes_path.to_string() })
    }


    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }


    pub fn get_plugins_paths(&self) -> Vec<String> {
        fn is_so(e: &fs::DirEntry) -> bool {
            let p = e.path();
            p.is_file() && p.extension().map(|s| s == "so").unwrap_or(false)
        }


        let mut v: Vec<String> = Vec::new();

        if let Ok(entries) = fs::read_dir(&Path::new(&self.probes_folder_path)) {
            for entry in entries.filter_map(|e| e.ok()).filter(is_so) {
                v.push(entry.path().into_os_string().into_string().unwrap());
            }
        }

        return v;
    }
}



