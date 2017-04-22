extern crate sys_info;


use self::sys_info::*;

use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;


#[derive(Debug)]
pub struct Os {
    logger: Logger
}


impl Os {
    pub fn new(logger: Logger) -> Os {
        let swap = Os { logger: logger };
        swap.register_probe();
        swap
    }
}

impl Probe for Os {
    fn exec(&self) -> () {
        let msg = format!("@Thread: {} - hostname {}: os {} {}, cpu {}, speed {}MHz",
                          self.get_thread_id(),
                          hostname().unwrap_or("unknown".to_string()),
                          os_type().unwrap_or("unknown".to_string()),
                          os_release().unwrap_or("unknown".to_string()),
                          cpu_num().unwrap_or(0),
                          cpu_speed().unwrap_or(0),
        );
        self.logger.log(Severity::LOG_INFO, &msg);
    }

    fn get_logger(&self) -> &Logger {
        &self.logger
    }
}




