//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.

extern crate syslog;

use base::{Operation};
use std::thread::{spawn, sleep};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};


#[derive(Debug)]
pub struct Worker {
    rx: Arc<Mutex<Receiver<()>>>,
    tx: Arc<Mutex<Sender<()>>>
}

impl Worker {
    pub fn new() -> Worker {
        let (tx, rx) = mpsc::channel();
        Worker { rx: Arc::new(Mutex::new(rx)), tx: Arc::new(Mutex::new(tx)) }
    }

    pub fn start(&self) -> () {
        let rx = self.rx.clone();
        spawn(move || {
            loop {
                match rx.lock().unwrap().try_recv() {
                    Err(TryRecvError::Disconnected) => {
                        println!("Terminating, channel disconnected");
                        //                        process::exit(THREAD_ERROR_CODE);
                    }
                    Ok(_) => {
                        println!("Finishing, poison pill received");
                        break
                    }
                    Err(TryRecvError::Empty) => {
                        println!("Work");
                    }
                }
                sleep(Duration::from_millis(2000));
            }
        });
    }
}


impl Drop for Worker {
    fn drop(&mut self) {
        self.tx.lock().unwrap().send(());
        println!("Worker Drop");
    }
}


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
    logger: ::Logger,
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
    pub fn new(
        id: T,
        anti_hog_millis: u64,
        logger: ::Logger
    ) -> Daemon<T> {
        let ss = Arc::new(AtomicBool::new(false));
        let fin = Arc::new(AtomicBool::new(false));
        Daemon {
            name: id,
            cpu_anti_hog_millis: anti_hog_millis,
            should_stop: ss,
            finished: fin,
            state: State::NotRunning,
            logger: logger
        }
    }

    pub fn start(&mut self, synced_spinnable_op: Box<Operation>) -> Status {
        match self.state {
            State::NotRunning => {
                self.state = State::Running;
                let msg = format!("daemon name {}", self.name);
                self.logger.log(&msg);
                let msg = format!("spawning worker thread");
                self.logger.log(&msg);

                //Main spinning worker, resource acquisition allowed
                // only one such thread allowed (synchronizing graceful pool close)
                let stop = self.should_stop.clone();
                let finished = self.finished.clone();
                let anti_hog_sleep = self.cpu_anti_hog_millis;
                let logger = self.logger.clone();

                spawn(move || {
                    loop {
                        if stop.load(Ordering::Relaxed) {
                            break;
                        }
                        synced_spinnable_op.exec();
                        sleep(Duration::from_millis(anti_hog_sleep));
                    }
                    finished.store(true, Ordering::Relaxed);
                    let msg = format!("main spinning worker finished");
                    logger.log(&msg);
                });


                let msg = format!("worker threads ready");
                self.logger.log(&msg);

                Ok(State::Running)
            },
            State::Running => {
                let msg = format!("{} already running", self.name);
                self.logger.log(&msg);
                Err(State::Running)
            }
        }
    }

    pub fn stop(&mut self) -> Status {
        match self.state {
            State::Running => {
                self.state = State::NotRunning;
                let msg = format!("{} will stop", self.name);
                self.logger.log(&msg);
                let stop = self.should_stop.clone();
                //                let finished = self.finished.clone();
                stop.store(true, Ordering::Relaxed);
                //                while !finished.load(Ordering::Relaxed) {
                //                    let msg = format!("worker thread closing");
                //                    self.logger.log(&msg);
                //                    sleep(Duration::from_millis(self.cpu_anti_hog_millis));
                //                }
                Ok(State::NotRunning)
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
                self.logger.log(&msg);
                Err(State::NotRunning)
            }
        }
    }


    //    pub fn spawn_one_shot_helper(
    //        &mut self,
    //        operation: Box<Operation>
    //    ) -> Status {
    //        match self.state {
    //            State::Running => {
    //                let logger = self.logger.clone();
    //                //Helper one shot worker no resource acquisition allowed no execution
    //                spawn(move || {
    //                    operation.exec();
    //                    let msg = format!("helper one-shot thread finished");
    //                    logger.log(&msg);
    //                });
    //                Ok(State::Running)
    //            },
    //            State::NotRunning => {
    //                let msg = format!("{} not running", self.name);
    //                self.logger.log(&msg);
    //                Err(State::NotRunning)
    //            }
    //        }
    //    }
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