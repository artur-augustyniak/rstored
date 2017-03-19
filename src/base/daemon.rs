//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

use base::{Operation};
use std::thread::{spawn, sleep};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender};


#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Running,
    NotRunning,
}

pub type Status = Result<State, State>;

#[derive(Debug)]
pub struct Daemon<T: ? Sized> {
    state: State,
    should_stop: Arc<AtomicBool>,
    finished: Arc<AtomicBool>,
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
        let ss = Arc::new(AtomicBool::new(false));
        let fin = Arc::new(AtomicBool::new(false));
        Daemon { name: id, should_stop: ss, finished: fin, state: State::NotRunning }
    }

    pub fn start(&mut self, op: Box<Operation>, finish_chan_tx: Sender<Status>) -> Status {
        match self.state {
            State::NotRunning => {
                self.state = State::Running;
                println!("[-] daemon name {}", self.name);
                println!("[-] spawning worker thread");
                let stop = self.should_stop.clone();
                let finished = self.finished.clone();
                spawn(move || {
                    loop {
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        op.exec();
                        sleep(Duration::from_secs(1));
                    }
                    finished.store(true, Ordering::Relaxed);
                    let notification_status = finish_chan_tx.send(Ok(State::NotRunning));
                    println!("[-] finish msg send status {:?}", notification_status);
                    println!("[-] worker thread finished");

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
                let stop = self.should_stop.clone();
                let finished = self.finished.clone();
                stop.store(true, Ordering::Relaxed);
                while !finished.load(Ordering::Relaxed) {
                    println!("[-] worker thread closing");
                    sleep(Duration::from_millis(300));
                }
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
        let op = Box::new(DebugPrint);
        let actual = daemon.start(op);
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