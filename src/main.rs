#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;

use std::process::{exit};

use std::thread::{spawn, sleep};
use std::time::Duration;
use chan::{Receiver};
use chan_signal::{Signal, notify};
use unix_daemonize::{daemonize_redirect, ChdirMode};


fn sig_handler(signal_chan_rx: Receiver<Signal>) {
    loop {
        let sig = signal_chan_rx.recv();
        match sig {
            Some(Signal::INT) => {
                println!("Stopped");
                exit(0);
            },
            Some(Signal::HUP) => println!("Handling HUP"),
            None => { exit(1); },
            Some(_) => { /*ignore*/ }
        }
    }
}


fn main() {
    let stdout_filename = "/tmp/stdout.log";
    let stderr_filename = "/tmp/stdout.log";

    daemonize_redirect(
        Some(stdout_filename),
        Some(stderr_filename),
        ChdirMode::ChdirRoot).unwrap();

    let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
    spawn(|| sig_handler(signal));
    println!("Start");
    loop {
        println!("DAEMON Working");
        sleep(Duration::from_secs(5));
    }
}