use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::io::{self, BufRead};

fn main() {
    println!("Press enter to wake up the child thread");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            println!("Suspending...");
            match rx.try_recv() {
                Ok(_) => {
                    println!("Terminating.");
                    break;
                }
                Err(_) => {
                    println!("Working...");
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(50000));
    let _ = tx.send(());
    //    let mut line = String::new();
    //    let stdin = io::stdin();
    //    for _ in 0..4 {
    //        let _ = stdin.lock().read_line(&mut line);
    //        let _ = tx.send(());
    //    }
}