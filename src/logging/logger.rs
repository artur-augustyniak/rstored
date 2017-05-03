pub extern crate syslog;

pub use self::syslog::Severity;


#[cfg(not(test))]
use self::prod_calls::{stdout_log, syslog_log};
#[cfg(test)]
use self::test_calls::{stdout_log, syslog_log};


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
                stdout_log(severity, msg);
            }
            LogDest::Syslog => {
                syslog_log(severity, msg);
            }
        }
    }
}

#[allow(dead_code)]
mod prod_calls {
    extern crate ansi_term;

    use self::ansi_term::Colour::{Red, Yellow, White, Green, Cyan, Blue};
    use super::Severity;
    use super::syslog::{unix, Facility};

    pub fn stdout_log(severity: Severity, msg: &str) -> () {
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

    pub fn syslog_log(severity: Severity, msg: &str) -> () {
        match unix(Facility::LOG_DAEMON) {
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

#[allow(dead_code)]
mod test_calls {
    use super::Severity;
    use std::mem;


    pub static mut FUNC_USED: u8 = 0;
    pub static mut SEVERITY_TRANSMUTED: u8 = 255;


    pub fn stdout_log(severity: Severity, msg: &str) -> () {
        unsafe {
            let ordinal: u8 = mem::transmute(severity);
            FUNC_USED = 1;
            SEVERITY_TRANSMUTED = ordinal;
            println!("stdout_log({}, {})", ordinal, msg);
        }
    }

    pub fn syslog_log(severity: Severity, msg: &str) -> () {
        unsafe {
            let ordinal: u8 = mem::transmute(severity);
            FUNC_USED = 2;
            SEVERITY_TRANSMUTED = ordinal;
            println!("syslog_log({}, {})", ordinal, msg);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Logger, LogDest};
    use super::{Severity};

    #[test]
    fn test_stdout_match() {
        let l = Logger::new(LogDest::StdOut);
        l.log(Severity::LOG_EMERG, "foo");
        unsafe {
            assert_eq! (1, super::test_calls::FUNC_USED);
            assert_eq! (0, super::test_calls::SEVERITY_TRANSMUTED);
        }
    }

    #[test]
    fn test_syslog_match() {
        let l = Logger::new(LogDest::Syslog);
        l.log(Severity::LOG_ALERT, "foo");
        unsafe {
            assert_eq! (2, super::test_calls::FUNC_USED);
            assert_eq! (1, super::test_calls::SEVERITY_TRANSMUTED);
        }
    }
}