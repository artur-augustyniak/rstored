#[macro_use]
mod base;

#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;
extern crate getopts;
extern crate syslog;
extern crate ini;


use ini::Ini;
use base::{Daemon, DebugPrint, Ls, FakeSpinner};
use getopts::Options;
use std::env;
use std::sync::mpsc::{self};
use std::process::{exit};
use std::sync::{Arc, Mutex};
use std::thread::{spawn};
use chan::{Receiver};
use chan_signal::{Signal, notify};
use unix_daemonize::{daemonize_redirect, ChdirMode};
use syslog::{Facility, Severity};


static SIGNALING_ERROR_EXIT_CODE: i32 = 0x1;
static CPU_ANTI_HOG_MILLIS_OFFSET: u64 = 100;
static STD_OUT_ERR_REDIR: &'static str = "/dev/null";

pub static REAL_SYSLOG: bool = false;


fn sig_handler(
    signal_chan_rx: Receiver<Signal>,
    daemon: Arc<Mutex<Daemon<String>>>
) {
    loop {
        let signal = signal_chan_rx.recv();
        match signal {
            Some(Signal::INT) => {
                let msg = format!("handling signal {:?}", Signal::INT);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
                let status = daemon.lock().unwrap().stop();
                let msg = format!("end daemon status {:?}", status);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
            },
            Some(Signal::HUP) => {
                let status = daemon.lock().unwrap().reload();
                let msg = format!("hup reload status {:?}", status);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_NOTICE, &msg);
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
    daemonize_redirect(
        Some(STD_OUT_ERR_REDIR),
        Some(STD_OUT_ERR_REDIR),
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
    opts.reqopt("c", "", "ini configuration file path", "[FILE]");
    opts.optflag("d", "demonize", "demonize in old unix fashion");
    opts.optflag("h", "help", "print this message");
    match opts.parse(&args[1..]) {
        Ok(matches) => {
            if matches.opt_present("h") {
                print_usage(&program, opts);
                return;
            }
            if matches.opt_present("d") {
                demonize();
            }
            let config_file = matches.opt_str("c").unwrap();
            let i = Ini::load_from_file(&config_file).unwrap();
            println!("configuration");
            let general_section_name = "__General__".into();
            for (sec, prop) in i.iter() {
                let section_name = sec.as_ref().unwrap_or(&general_section_name);
                println!("-- Section: {:?} begins", section_name);
                for (k, v) in prop.iter() {
                    println!("{}: {:?}", *k, *v);
                }
            }

            let signal = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
            let daemon = Daemon::new(program, CPU_ANTI_HOG_MILLIS_OFFSET);
            let sig_handler_ref = Arc::new(Mutex::new(daemon));
            let main_thread_ref = sig_handler_ref.clone();
            let (end_signal_tx, end_signal_rx) = mpsc::channel();
            spawn(move || { sig_handler(signal, sig_handler_ref); });

            //force mutex unlock
            {
                let op = Box::new(DebugPrint);
                let mut daemon = main_thread_ref.lock().unwrap();
                let start_status = daemon.start(op, end_signal_tx);
                let msg = format!("daemon start status {:?}", start_status);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);

                let op = Box::new(Ls);
                let spawn_status_oneshot = daemon.spawn_one_shot_helper(op);
                let msg = format!("one shot start status {:?}", spawn_status_oneshot);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);

                let op = Box::new(FakeSpinner);
                let spawn_status_spinner1 = daemon.spawn_spinning_helper(op);
                let msg = format!("spinner start status1 {:?}", spawn_status_spinner1);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);

                let op2 = Box::new(FakeSpinner);
                let spawn_status_spinner2 = daemon.spawn_spinning_helper(op2);
                let msg = format!("spinner start status2 {:?}", spawn_status_spinner2);
                log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
            }

            let finish_result = end_signal_rx.recv();
            let msg = format!("finishing in {:?} status", finish_result.unwrap());
            log!(REAL_SYSLOG, Facility::LOG_DAEMON, Severity::LOG_INFO, &msg);
        }
        Err(f) => {
            print_usage(&program, opts);
            panic!(f.to_string())
        }
    };
}


