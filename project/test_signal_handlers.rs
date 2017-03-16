#[macro_use]
extern crate chan;
extern crate chan_signal;

use std::process::{exit};

use std::thread::{spawn, sleep};
use std::time::Duration;
use chan::{Receiver};
use chan_signal::{Signal, notify};


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
            Some(_) => ()
        }
    }
}


fn main() {
    let signal = notify(&[Signal::INT, Signal::HUP]);
    spawn(|| sig_handler(signal));
    println!("Start");
    loop {
        println!("Working");
        sleep(Duration::from_secs(5));
    }
}