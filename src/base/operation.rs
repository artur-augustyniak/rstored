pub trait Operation: Send + Sync {
    fn exec(&self) -> ();
}

#[derive(Debug)]
pub struct DebugPrint;


impl Operation for DebugPrint {
    fn exec(&self) -> () {
        println!("[+] {:?} working...", self);
    }
}

#[derive(Debug)]
pub struct Ls;

impl Operation for Ls {
    fn exec(&self) -> () {
        use std::process::Command;

        let status = Command::new("ls").status().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        println!("[+] process exited with: {}", status);
    }
}
