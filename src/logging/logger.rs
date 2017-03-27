extern crate syslog;

use self::syslog::{Facility, Severity};

#[derive(Debug, PartialEq, Clone)]
pub enum LogDest {
    StdOut,
    Syslog,
}

#[derive(Debug, Clone)]
pub struct Logger {
    dest: LogDest
}


impl Logger {
    pub fn new(dest: LogDest) -> Logger {
        Logger { dest: dest }
    }

    pub fn log(&self, msg: &str) -> () {
        match self.dest {
            LogDest::StdOut => {
                println!("LOGGER STDOUT {}", msg);
            }
            LogDest::Syslog => {
                println!("LOGGER SYSLOG {}", msg);


                match syslog::unix(Facility::LOG_DAEMON) {
                    Err(e) => println!("[!] impossible to connect to syslog: {:?}", e),
                    Ok(writer) => {
                        let r = writer.send(Severity::LOG_INFO, msg);
                        if r.is_err() {
                            println!("[!] error sending the log {}", r.err().expect("got error"));
                        }
                    }
                }
            }
        }
    }
}