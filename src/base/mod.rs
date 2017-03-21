//! # All base app elements
//! This is documentation for the `base` module.
extern crate syslog;
#[macro_export]
macro_rules! log {
    ($to_syslog:expr, $facility:expr, $severity:expr, $msg:expr) => {
        if !$to_syslog{
            println!("[-] DEBUG SYSLOG - {} | {} | MSG::{}",
                stringify!($facility),
                stringify!($severity),
                $msg
                );
        } else {
            match syslog::unix($facility) {
                Err(e) => println!("[!] impossible to connect to syslog: {:?}", e),
                Ok(writer) => {
                    let r = writer.send($severity, $msg);
                    if r.is_err() {
                        println!("[!] error sending the log {}", r.err().expect("got error"));
                    }
                }
            }
        }
    };
}
pub mod daemon;
pub mod operation;

pub use self::daemon::Daemon;
pub use self::daemon::Status;
pub use self::operation::Operation;
pub use self::operation::DebugPrint;
pub use self::operation::Ls;
pub use self::operation::FakeSpinner;
