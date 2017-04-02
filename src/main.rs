mod base;
mod logging;

#[macro_use] extern crate log;
extern crate daemonize;
extern crate chan;
extern crate chan_signal;
extern crate getopts;

use base::{Operation, DebugPrint, Ls, FakeSpinner};
use logging::{LogDest, Logger};
use base::{Worker, Config};
use getopts::Options;
use std::env;
use std::sync::{Arc};
use std::sync::mpsc::{self};
use std::process::{exit};
use std::thread::{spawn};
use chan::{Receiver};
use chan_signal::{Signal, notify};
use std::sync::mpsc::{Sender};
use daemonize::{Daemonize};

static SIGNALING_ERROR_EXIT_CODE: i32 = 0x1;
static CONFIG_ERROR_EXIT_CODE: i32 = 0x2;
static STD_OUT_ERR_REDIR: &'static str = "/dev/null";


fn initiator(
    reload_trigger_rx: std::sync::mpsc::Receiver<()>,
    logger: Logger,
    cfg_file_path: &str
) {
    loop {
        let config = Config::new(cfg_file_path);

        match config {
            Ok(c) => {
                println!("{:?}", c);
                let mut v: Vec<Box<Operation>> = Vec::new();
                v.push(Box::new(DebugPrint::new(logger.clone())));
                v.push(Box::new(Ls::new(logger.clone())));
                v.push(Box::new(FakeSpinner::new(logger.clone())));
                let w = Worker::new(logger.clone(), Arc::new(v), c);
                w.start();
                let reload = reload_trigger_rx.recv();
                let msg = format!("Worker start {:?}", reload);
                logger.log(&msg);
            }
            Err(err) => {
                println!("Error: {}", err);
                exit(CONFIG_ERROR_EXIT_CODE);
            }
        }
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
//    daemonize_redirect(
//        Some(STD_OUT_ERR_REDIR),
//        Some(STD_OUT_ERR_REDIR),
//        ChdirMode::ChdirRoot).unwrap(); //TODO handle unwrap
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
            let mut ini_logger = Logger::new(LogDest::Syslog);
            if matches.opt_present("d") {

                let daemonize = Daemonize::new()
                    .pid_file("/tmp/test.pid") // Every method except `new` and `start`
//                    .chown_pid_file(true)      // is optional, see `Daemonize` documentation
                    .working_directory("/tmp") // for default behaviour.
                    .user("aaugustyniak")
                    .group("aaugustyniak") // Group name
                    .group(1000)        // or group id.
                    .umask(0o777)    // Set umask, `0o027` by default.
                    .privileged_action(|| "Executed before drop privileges");

                match daemonize.start() {
                    Ok(_) => ini_logger.log("Success, daemonized"),

                    Err(e) => {
                        let msg = format!("error {:?}", e);
                        ini_logger.log(&msg)
                    }
                }


//                demonize();
//                ini_logger = Logger::new(LogDest::Syslog);

//                match fork().expect("fork failed") {
//                    ForkResult::Parent{ child } => {
//                        sleep(5);
//                        kill(child, SIGKILL).expect("kill failed");
//                    }
//                    ForkResult::Child => {
//                        loop {}  // until killed
//                    }
//                }

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


