extern crate rand;

use self::rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, TryRecvError};

static WATCH_DOG_SLEEP_MILLIS: u64 = 500;
static PROBE_SLEEP_MILLIS: u64 = 500;

fn watch_dog(rx: mpsc::Receiver<()>) {
    loop {
        match rx.try_recv() {
            Err(TryRecvError::Disconnected) => {
                println!("Terminating, channel disconnected");
                break;
            }
            Ok(_) => {
                println!("Finishing, poison pill received");
                break
            }
            Err(TryRecvError::Empty) => {
                println!("Idle, no message in channel");
            }
        }
        thread::sleep(Duration::from_millis(WATCH_DOG_SLEEP_MILLIS));
    }
}


fn main() {
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            println!("this is thread number 1");
            thread::sleep(Duration::from_millis(500));
            if rng.gen() {
                panic!();
            }
        }
        let _ = tx.send(());
    });

    


    watch_dog(rx);
}

