mod base;

use base::Daemon;

#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;
extern crate getopts;


use getopts::Options;
use std::env;
use std::process::{exit};
use std::sync::{Arc, Mutex};

use std::thread::{spawn, sleep};
use std::time::Duration;
use chan::{Receiver};
use chan_signal::{Signal, notify};
use unix_daemonize::{daemonize_redirect, ChdirMode};


fn sig_handler(signal_chan_rx: Receiver<Signal>, mut daemon: std::sync::MutexGuard<Daemon<&str>>) {
    loop {
        let signal = signal_chan_rx.recv();

        match signal {
            Some(Signal::INT) => {
                println!("Handling INT");
                daemon.stop();
                exit(0);
            },
            Some(Signal::HUP) => {
                println!("Handling HUP");
                daemon.reload();
            },
            Some(_) => {
                println!("Unknown Err");
            },
            None => {
                println!("Error");
                daemon.stop();
                exit(1);
            }
        }
    }
}

fn demonize() {
    let stdout_filename = "/tmp/stdout.log";
    let stderr_filename = "/tmp/stdout.log";

    daemonize_redirect(
        Some(stdout_filename),
        Some(stderr_filename),
        ChdirMode::ChdirRoot).unwrap(); //TODO handle unwrap
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {
    let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
    println!("Plumbing");
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("f", "foreground", "do not demonize");
    opts.optflag("h", "help", "print this message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    //    if !matches.opt_present("f") {
    //        demonize();
    //    }

    println!("Starting logic");

    let mut daemon = Daemon::new("sd");



    spawn(|| {
        let mut daemon = Daemon::new("sd");
        let data = Arc::new(Mutex::new(daemon));
        let data_for_thread = data.clone();
        let mut data = data.lock().unwrap();

        sig_handler(signal, data_for_thread);
        data.start();

    });

//daemon.start();
//    data.lock().unwrap().start();

}


