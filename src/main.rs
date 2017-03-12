mod base;

#[macro_use]
extern crate chan;
extern crate chan_signal;


use std::time::Duration;

use chan_signal::Signal;

extern crate unix_daemonize;

use std::{io, env, time, thread, process};
use std::io::Write;
use self::unix_daemonize::{daemonize_redirect, ChdirMode};

use base::Daemon;

fn run() {
    println!("Running work for N seconds.");
    println!("Can you send a signal quickly enough?");
    // Do some work.
    for _ in 0..40 {
        println!("A string for stdout!");

        writeln!(&mut io::stdout(), "Another string for stdout!").unwrap();
        writeln!(&mut io::stderr(), "A string for stderr!").unwrap();
        thread::sleep(time::Duration::from_millis(1000));
    }

    // _sdone gets dropped which closes the channel and causes `rdone`
    // to unblock.
}


fn main() {
    let stdout_filename = "/tmp/stdout.log";
    let stderr_filename = "/tmp/stdout.log";
    println!("Ready to daemonize, target stdout_filename = {}, stderr_filename = {}", stdout_filename, stderr_filename);
    daemonize_redirect(Some(stdout_filename), Some(stderr_filename), ChdirMode::ChdirRoot).unwrap();
    println!("Running");

//    let (sdone, rdone) = chan::sync(1);
//    chan_signal::notify_on(&sdone, Signal::HUP);
    // When our work is complete, send a sentinel value on `sdone`.
    // Run work.
    thread::spawn(move || run());
    let signal = chan_signal::notify(&[Signal::HUP]);
    println!("Send a HUP signal my way!");
    // block until we get a signal

    println!("{:?}", signal.recv());
//    assert_eq!(signal.recv(), Some(Signal::INT));
    println!("Thanks :]");

    // Wait for a signal or for work to be done.
//    chan_select! {
////        signal.recv() -> signal => {
////            println!("received signal: {:?}", signal)
////        },
//        rdone.recv() => {
//            println!("Program completed normally.");
//        }
//    }


    println!("Successfull termination");

    //    let mut d = Daemon::new("rstored");
    //    println!("in parent {:?}", d.start());
    //    println!("in parent {:?}", d.start());
    //
    //
    //    println!("in child {:?}", d.stop());
    //    //    println!("{:?}", d.reload());
}