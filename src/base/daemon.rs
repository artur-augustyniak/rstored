//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

use base::{Operation};
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Running,
    NotRunning,
}

pub type Status = Result<State, State>;

#[derive(Debug)]
pub struct Daemon<T: ? Sized> {
    state: State,
    name: T //unsized must be last
}

impl<T> Display for Daemon<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.name)
    }
}

impl<T> Daemon<T> where T: Display {
    /// Constructs a new `Daemon<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use base::Daemon;
    /// let mut d = Daemon::new("some_name")
    /// ```
    pub fn new(id: T) -> Daemon<T> {
        Daemon { name: id, state: State::NotRunning }
    }

    pub fn start(&mut self, op: Box<Operation>) -> Status {
        match self.state {
            State::NotRunning => {
                self.state = State::Running;
                println!("[-] daemon name {}", self.name);
                println!("[-] spawning worker thread");
                spawn(move || {
                    loop {
                        op.exec();
                        sleep(Duration::from_secs(5));
                    }
                });
                println!("[-] worker thread ready");
                Ok(State::Running)
            },
            State::Running => {
                println!("[-] {} already running", self.name);
                Err(State::Running)
            }
        }
    }

    pub fn stop(&mut self) -> Status {
        match self.state {
            State::Running => {
                self.state = State::NotRunning;
                println!("[-] {} will stop", self.name);
                Ok(State::NotRunning)
            },
            State::NotRunning => {
                Err(State::NotRunning)
            }
        }
    }

    pub fn reload(&mut self) -> Status {
        match self.state {
            State::Running => {
                println!("[-] {} reloading", self.name);
                Ok(State::Running)
            },
            State::NotRunning => {
                Err(State::NotRunning)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_start_when_not_running() {
        let mut daemon = Daemon::new("some_name");
        let expected = Ok(State::Running);
        let actual = daemon.start();
        assert_eq!(expected, actual);
    }


    #[test]
    fn cant_reload_when_not_running() {
        let mut daemon = Daemon::new("some_name");
        let expected = Err(State::NotRunning);
        let actual = daemon.reload();
        assert_eq!(expected, actual);
    }


    #[test]
    fn cant_stop_when_not_running() {
        let mut daemon = Daemon::new("some_name");
        let expected = Err(State::NotRunning);
        let actual = daemon.stop();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_stop_when_already_running() {
        let mut daemon = Daemon::new("some_name");
        match daemon.start() {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::NotRunning);
        let actual = daemon.stop();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_reload_when_already_running() {
        let mut daemon = Daemon::new("some_name");
        match daemon.start() {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq!(expected, actual);
    }


    #[test]
    fn can_reload_when_already_reloaded() {
        let mut daemon = Daemon::new("some_name");
        match daemon.start() {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq!(expected, actual);

        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cant_start_when_already_running() {
        let mut daemon = Daemon::new("some_name");
        match daemon.start() {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Err(State::Running);
        let actual = daemon.start();
        assert_eq!(expected, actual);
    }

    #[test]
    fn cant_start_when_already_reloaded() {
        let mut daemon = Daemon::new("some_name");
        match daemon.start() {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq!(expected, actual);

        let expected = Err(State::Running);
        let actual = daemon.start();
        assert_eq!(expected, actual);
    }
}