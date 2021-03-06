extern crate unix_daemonize;

use std::{io, env, time, thread, process};
use std::io::Write;
use unix_daemonize::{daemonize_redirect, ChdirMode};



fn main() {

    let mut daemon = Daemon::new("some_name");
    let expected = Ok(State::Running);
    let actual = daemon.start();
    assert_eq!(expected, actual);

    let mut args = env::args();
    let cmd_proc = args.next().unwrap();
    let t = Blah { state: 666 };

    if let (Some(stdout_filename), Some(stderr_filename)) = (args.next(), args.next()) {
        println!("Ready to daemonize, target stdout_filename = {}, stderr_filename = {}", stdout_filename, stderr_filename);
        daemonize_redirect(Some(stdout_filename), Some(stderr_filename), ChdirMode::ChdirRoot).unwrap();
        println!("Running");
        for _ in 0..10 {
            println!("A string for stdout!");
            println!("A parent state object {}", t.state);
            writeln!(&mut io::stdout(), "Another string for stdout!").unwrap();
            writeln!(&mut io::stderr(), "A string for stderr!").unwrap();
            thread::sleep(time::Duration::from_millis(1000));
        }
        println!("Successfull termination");
        //        panic!("An now a panic occurs!");
    } else {
        writeln!(&mut io::stderr(), "Usage: {} <stdout_filename> <stderr_filename>", cmd_proc).unwrap();
        process::exit(1);
    }
    println!("Stopping");
}