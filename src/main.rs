mod base;

#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;
extern crate getopts;
use std::time::Duration;
use base::{Daemon, Status, DebugPrint, Ls};
use getopts::Options;
use std::env;
use std::sync::mpsc::{self, Sender};
use std::process::{exit};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep};
use chan::{Receiver};
use chan_signal::{Signal, notify};
use unix_daemonize::{daemonize_redirect, ChdirMode};

static SIGNALING_ERROR_EXIT_CODE: i32 = 0x1;

fn sig_handler(
    signal_chan_rx: Receiver<Signal>,
    daemon: Arc<Mutex<Daemon<String>>>,
    finish_chan_tx: Sender<Status>
) {
    loop {
        let signal = signal_chan_rx.recv();
        match signal {
            Some(Signal::INT) => {
                let status = daemon.lock().unwrap().stop();
                sleep(Duration::from_secs(5));
                let notification_status = finish_chan_tx.send(status);
                println!("[-] finish msg send status {:?}", notification_status);
            },
            Some(Signal::HUP) => {
                let status = daemon.lock().unwrap().reload();
                println!("[-] hup reload status {:?}", status);
            },
            Some(_) => {
                ();
            },
            None => {
                exit(SIGNALING_ERROR_EXIT_CODE);
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
    opts.optflag("d", "demonize", "demonize in old unix fashion");
    opts.optflag("h", "help", "print this message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("d") {
        demonize();
    }


    let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
    let daemon = Daemon::new(program);
    let sig_handler_ref = Arc::new(Mutex::new(daemon));
    let main_thread_ref = sig_handler_ref.clone();
    let (end_signal_tx, end_signal_rx) = mpsc::channel();
    spawn(move || { sig_handler(signal, sig_handler_ref, end_signal_tx); });


    //force mutex unlock
    {
        let op = Box::new(DebugPrint);
        let mut daemon = main_thread_ref.lock().unwrap();
        let start_status = daemon.start(op);
        println!("[-] daemon start status {:?}", start_status);
    }
    let finish_result = end_signal_rx.recv();
    println!("[-] finishing in {:?} status", finish_result.unwrap());
}


