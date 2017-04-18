pub extern crate syslog;
extern crate ansi_term;

use self::syslog::Facility;
pub use self::syslog::Severity;
use self::ansi_term::Colour::{Red, Yellow, White, Green, Cyan, Blue};


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

    pub fn log(&self, severity: Severity, msg: &str) -> () {
        match self.dest {
            LogDest::StdOut => {
                match severity {
                    Severity::LOG_EMERG => {
                        println!("<0> {}", Cyan.paint(msg));
                    }
                    Severity::LOG_ALERT => {
                        println!("<1> {}", Red.paint(msg));
                    }
                    Severity::LOG_CRIT => {
                        println!("<2> {}", Red.paint(msg));
                    }
                    Severity::LOG_ERR => {
                        println!("<3> {}", Red.paint(msg));
                    }
                    Severity::LOG_WARNING => {
                        println!("<4> {}", Yellow.paint(msg));
                    }
                    Severity::LOG_NOTICE => {
                        println!("<5> {}", Blue.paint(msg));
                    }
                    Severity::LOG_INFO => {
                        println!("<6> {}", White.paint(msg));
                    }
                    Severity::LOG_DEBUG => {
                        println!("<7> {}", Green.paint(msg));
                    }
                }
            }
            LogDest::Syslog => {
                match syslog::unix(Facility::LOG_DAEMON) {
                    Err(e) => println!("[!] impossible to connect to syslog: {:?}", e),
                    Ok(writer) => {
                        let r = writer.send(severity, msg);
                        if r.is_err() {
                            println!("[!] error sending the log {}", r.err().expect("got error"));
                        }
                    }
                }
            }
        }
    }
}