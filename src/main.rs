mod base;
mod logging;

#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate unix_daemonize;
extern crate getopts;
extern crate ini;


use ini::Ini;
use base::{Operation, DebugPrint, Ls, FakeSpinner};
use logging::{LogDest, Logger};
use base::{Worker};
use getopts::Options;
use std::env;
use std::sync::{Arc};
use std::sync::mpsc::{self};
use std::process::{exit};
use std::thread::{spawn};
use chan::{Receiver};
use chan_signal::{Signal, notify};
use unix_daemonize::{daemonize_redirect, ChdirMode};
use std::sync::mpsc::{Sender};
use std::collections::HashMap;


pub static SIGNALING_ERROR_EXIT_CODE: i32 = 0x1;
static STD_OUT_ERR_REDIR: &'static str = "/dev/null";


fn load_config(config_file: &str) {
    let i = Ini::load_from_file(config_file).unwrap();
    println!("configuration");

//    let mut config = HashMap::new();

    let general_section_name = "__General__".into();

    for (sec, prop) in i.iter() {
        let section_name = sec.as_ref().unwrap_or(&general_section_name);
//        let mut section_contents = HashMap::new();
//        config.insert(section_name, section_contents);
        println!("-- Section: {:?} begins", section_name);
        for (k, v) in prop.iter() {
//            section_contents.insert(*k, *v);
            println!("{}: {:?}", *k, *v);
        }
    }
}


fn initiator(
    reload_trigger_rx: std::sync::mpsc::Receiver<()>,
    logger: Logger,
    cfg_file_path: &str
) {
    loop {
        load_config(cfg_file_path);
        let mut v: Vec<Box<Operation>> = Vec::new();

        v.push(Box::new(DebugPrint::new(logger.clone())));
        v.push(Box::new(Ls::new(logger.clone())));
        v.push(Box::new(FakeSpinner::new(logger.clone())));
        let w = Worker::new(logger.clone(), Arc::new(v));
        w.start();
        let reload = reload_trigger_rx.recv();
        let msg = format!("Worker start {:?}", reload);
        logger.log(&msg);
    }
}

fn signal_handler(
    signal_channel_rx: Receiver<Signal>,
    reload_trigger_tx: Sender<()>,
    finish_channel_tx: Sender<()>,
    logger: Logger
) {
    loop {
        let signal = signal_channel_rx.recv();
        match signal {
            Some(Signal::INT) => {
                let msg = format!("Handling {:?}", Signal::INT);
                logger.log(&msg);
                let finish = finish_channel_tx.send(());
                let msg = format!("INT finish status {:?}", finish);
                logger.log(&msg);
            }
            Some(Signal::HUP) => {
                let msg = format!("Handling {:?}", Signal::HUP);
                logger.log(&msg);
                let reload = reload_trigger_tx.send(());
                let msg = format!("HUP reload status {:?}", reload);
                logger.log(&msg);
            }
            Some(_) => {
                ();
            }
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
            let mut ini_logger = Logger::new(LogDest::StdOut);
            if matches.opt_present("d") {
                demonize();
                ini_logger = Logger::new(LogDest::Syslog);
            }
            let logger = ini_logger;
            let signal_handler_logger = logger.clone();
            let initiator_logger = logger.clone();

            let signal_channel_rx = notify(&[Signal::INT, Signal::HUP, Signal::TERM]);
            let (finish_channel_tx, finish_channel_rx) = mpsc::channel();
            let (reload_trigger_tx, reload_trigger_rx) = mpsc::channel();

            let config_file = matches.opt_str("c").unwrap();

            spawn(move || {
                initiator(reload_trigger_rx, initiator_logger, &config_file);
            });

            spawn(move || {
                signal_handler(
                    signal_channel_rx,
                    reload_trigger_tx,
                    finish_channel_tx,
                    signal_handler_logger);
            });

            let finish_result = finish_channel_rx.recv();
            let msg = format!("shutdown {:?} status", finish_result.unwrap());
            logger.log(&msg);
        }
        Err(_) => {
            print_usage(&program, opts);
            return;
        }
    };
}


