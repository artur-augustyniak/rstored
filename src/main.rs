#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;
extern crate getopts;


use getopts::Options;
use std::env;
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

fn demonize() {
    let stdout_filename = "/tmp/stdout.log";
    let stderr_filename = "/tmp/stdout.log";

    daemonize_redirect(
        Some(stdout_filename),
        Some(stderr_filename),
        ChdirMode::ChdirRoot).unwrap(); //TODO handle unwrap
}


fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
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

    //    let mut daemon = Daemon::new("some_name");
    //    let expected = Ok(State::Running);
    //    let actual = daemon.start();


    let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
    spawn(|| sig_handler(signal));
    println!("Start");
    loop {
        println!("DAEMON Working");
        sleep(Duration::from_secs(5));
    }
}