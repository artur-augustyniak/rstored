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


fn sig_handler(signal_chan_rx: Receiver<Signal>, daemon: Arc<Mutex<Daemon<String>>>) {
    loop {
        let signal = signal_chan_rx.recv();
        match signal {
            Some(Signal::INT) => {
                println!("Handling INT");
                daemon.lock().unwrap().stop();
                exit(0);
            },
            Some(Signal::HUP) => {
                daemon.lock().unwrap().reload();
                println!("Handling HUP");
            },
            Some(_) => {
                println!("Unknown SIGNAL");
            },
            None => {
                println!("Error");
                //                daemon.stop();
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
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {
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
    if !matches.opt_present("f") {
        demonize();
    }
    println!("Starting logic");
    let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);

    let mut daemon = Daemon::new(program);

    let sig_handler_ref = Arc::new(Mutex::new(daemon));
    let main_thread_ref = sig_handler_ref.clone();
    spawn(move || { sig_handler(signal, sig_handler_ref); });

    //unlock fast
    {
        let mut daemon = main_thread_ref.lock().unwrap();
        daemon.start();
    }
    sleep(Duration::from_secs(30));
//    loop {
//        println!("DAEMON Working");
//    }

}


