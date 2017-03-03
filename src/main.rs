mod base;

use base::Daemon;

fn main() {
    let mut d = Daemon::new("some_name");
    println!("{:?}", d.name);
    println!("{:?}", d.start());
    println!("{:?}", d.reload());
    println!("{:?}", d.stop());
}
