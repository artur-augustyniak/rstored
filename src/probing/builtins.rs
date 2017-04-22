extern crate sys_info;


use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Mem {
    logger: Logger
}


impl Mem {
    pub fn new(logger: Logger) -> Mem {
        let mem = Mem { logger: logger };
        mem.register_probe();
        mem
    }
}

impl Probe for Mem {
    fn exec(&self) -> () {
        match mem_info() {
            Ok(mem) => {
                let msg = format!("@Thread: {} - mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
                                  self.get_thread_id(),
                                  mem.total,
                                  mem.free,
                                  mem.avail,
                                  mem.buffers,
                                  mem.cached,
                );
                self.logger.log(Severity::LOG_INFO, &msg);
            }

            Err(err) => {
                let msg = format!("{:?}", err);
                self.logger.log(Severity::LOG_ERR, &msg);
            }
        }
    }

    fn get_logger(&self) -> &Logger {
        &self.logger
    }
}
//println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
//        println!("os: {} {}", os_type().unwrap(), os_release().unwrap());
//        println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
//        println!("proc total: {}", proc_total().unwrap());
//        let load = loadavg().unwrap();
//        println!("load: {} {} {}", load.one, load.five, load.fifteen);
//        let disk = disk_info().unwrap();
//        println!("disk: total {} KB, free {} KB", disk.total, disk.free);
//        println!("hostname: {}", hostname().unwrap());