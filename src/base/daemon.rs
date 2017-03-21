//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

extern crate syslog;

use ::REAL_SYSLOG as REAL_SYSLOG;
use base::{Operation};
use std::thread::{spawn, sleep};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender};
use syslog::{Facility, Severity};

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Running,
    NotRunning,
}

pub type Status = Result<State, State>;

#[derive(Debug)]
pub struct Daemon<T: ? Sized> {
    cpu_anti_hog_millis: u64,
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
    pub fn new(id: T, anti_hog_millis: u64) -> Daemon<T> {
        let ss = Arc::new(AtomicBool::new(false));
        let fin = Arc::new(AtomicBool::new(false));
        Daemon {
            name: id,
            cpu_anti_hog_millis: anti_hog_millis,
            should_stop: ss,
            finished: fin,
            state: State::NotRunning
        }
    }

    pub fn start(
        &mut self,
        synced_spinnable_op: Box<Operation>,
        finish_chan_tx: Sender<Status>
    ) -> Status {
        match self.state {
            State::NotRunning => {
                self.state = State::Running;

                let msg = format!("daemon name {}", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                let msg = format!("spawning worker thread");
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);

                //Main spinning worker resource acquisition allowed
                // only one such thread allowed (synchronizing graceful pool close)
                let stop = self.should_stop.clone();
                let finished = self.finished.clone();
                let anti_hog_sleep = self.cpu_anti_hog_millis;
                spawn(move || {
                    loop {
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        synced_spinnable_op.exec();
                        sleep(Duration::from_millis(anti_hog_sleep));
                    }
                    finished.store(true, Ordering::Relaxed);
                    let notification_status = finish_chan_tx.send(Ok(State::NotRunning));
                    let msg = format!("finish msg send status {:?}", notification_status);
                    log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                    let msg = format!("main spinning worker finished");
                    log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                });

                let msg = format!("worker threads ready");
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);

                Ok(State::Running)
            },
            State::Running => {
                let msg = format!("{} already running", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                Err(State::Running)
            }
        }
    }

    pub fn stop(&mut self) -> Status {
        match self.state {
            State::Running => {
                self.state = State::NotRunning;
                let msg = format!("{} will stop", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                let stop = self.should_stop.clone();
                let finished = self.finished.clone();
                stop.store(true, Ordering::Relaxed);
                while !finished.load(Ordering::Relaxed) {
                    let msg = format!("worker thread closing");
                    log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                    sleep(Duration::from_millis(self.cpu_anti_hog_millis));
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
                let msg = format!("{} reloading", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                Ok(State::Running)
            },
            State::NotRunning => {
                Err(State::NotRunning)
            }
        }
    }

    pub fn spawn_spinning_helper(
        &mut self,
        spinnable: Box<Operation>
    ) -> Status {
        match self.state {
            State::Running => {
                //Helper spinning worker
                // no resource acquisition allowed
                let stop = self.should_stop.clone();
                let anti_hog_sleep = self.cpu_anti_hog_millis;
                spawn(move || {
                    loop {
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        spinnable.exec();
                        sleep(Duration::from_millis(anti_hog_sleep));
                    }
                });
                Ok(State::Running)
            },
            State::NotRunning => {
                let msg = format!("{} not running", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                Err(State::NotRunning)
            }
        }
    }


    pub fn spawn_one_shot_helper(
        &mut self,
        operation: Box<Operation>
    ) -> Status {
        match self.state {
            State::Running => {
                //Helper one shot worker no resource acquisition allowed no execution
                spawn(move || {
                    operation.exec();
                    let msg = format!("helper one-shot thread finished");
                    log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                });
                Ok(State::Running)
            },
            State::NotRunning => {
                let msg = format!("{} not running", self.name);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                Err(State::NotRunning)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self};

    static TEST_CPU_ANTI_HOG_MILLIS: u64 = 0x0;

    #[derive(Debug)]
    pub struct OperationMock;


    impl Operation for OperationMock {
        fn exec(&self) -> () {}
    }

    #[test]
    fn can_start_when_not_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let expected = Ok(State::Running);
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        let actual = daemon.start(op, end_signal_tx);
        assert_eq! (expected, actual);
    }

    #[test]
    fn cant_reload_when_not_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let expected = Err(State::NotRunning);
        let actual = daemon.reload();
        assert_eq! (expected, actual);
    }

    #[test]
    fn cant_stop_when_not_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let expected = Err(State::NotRunning);
        let actual = daemon.stop();
        assert_eq! (expected, actual);
    }

    #[test]
    fn can_stop_when_already_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        match daemon.start(op, end_signal_tx) {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::NotRunning);
        let actual = daemon.stop();
        assert_eq! (expected, actual);
    }

    #[test]
    fn can_reload_when_already_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        match daemon.start(op, end_signal_tx) {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq! (expected, actual);
    }


    #[test]
    fn can_reload_when_already_reloaded() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        match daemon.start(op, end_signal_tx) {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq! (expected, actual);

        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq! (expected, actual);
    }

    #[test]
    fn cant_start_when_already_running() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        match daemon.start(op, end_signal_tx) {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Err(State::Running);
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        let actual = daemon.start(op, end_signal_tx);
        assert_eq! (expected, actual);
    }

    #[test]
    fn cant_start_when_already_reloaded() {
        let mut daemon = Daemon::new(
            "some_name",
            TEST_CPU_ANTI_HOG_MILLIS
        );
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        match daemon.start(op, end_signal_tx) {
            Err(_) => {
                panic!("Error, can't run daemon for unknown reason");
            },
            _ => ()
        }
        let expected = Ok(State::Running);
        let actual = daemon.reload();
        assert_eq! (expected, actual);

        let expected = Err(State::Running);
        let op = Box::new(OperationMock);
        let (end_signal_tx, _) = mpsc::channel();
        let actual = daemon.start(op, end_signal_tx);
        assert_eq! (expected, actual);
    }
}