//! # Daemon abstraction
//! This is documentation for the `daemon` module.
//!
//! Lorem Ipsum
//! functionality for building portable Rust software.


extern crate unix_daemonize;

use std::{io, env, time, thread, process};
use std::io::Write;
use self::unix_daemonize::{daemonize_redirect, ChdirMode};


#[derive(Debug, PartialEq)]
pub enum State {
    Running,
    NotRunning,
}

type Status = Result<State, State>;

#[derive(Debug)]
pub struct Daemon<T: ? Sized> {
    state: State,
    pub name: T //unsized must be last
}

impl<T> Daemon<T> {

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

    pub fn start(&mut self) -> Status {
        match self.state {
            State::NotRunning => {
                self.state = State::Running;
                let stdout_filename = "/tmp/stdout.log";
                let stderr_filename = "/tmp/stdout.log";
                println!("Ready to daemonize, target stdout_filename = {}, stderr_filename = {}", stdout_filename, stderr_filename);
                daemonize_redirect(Some(stdout_filename), Some(stderr_filename), ChdirMode::ChdirRoot).unwrap();

                println!("Running");
                for _ in 0..10 {
                    println!("A string for stdout!");
                    println!("A parent state object {:?}", self.state);
                    writeln!(&mut io::stdout(), "Another string for stdout!").unwrap();
                    writeln!(&mut io::stderr(), "A string for stderr!").unwrap();
                    thread::sleep(time::Duration::from_millis(1000));
                }
                println!("Successfull termination");


                Ok(State::Running)
            },
            State::Running => {
                Err(State::Running)
            }
        }
    }

    pub fn stop(&mut self) -> Status {
        match self.state {
            State::Running => {
                self.state = State::NotRunning;
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