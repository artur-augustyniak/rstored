extern crate libc;
extern crate libloading as lib;


use super::probe::Probe;
use ::logging::{Logger};
use logging::logger::syslog::Severity;
use self::libc::c_char;
use self::libc::c_int;
use std::ffi::CStr;
use std::str;


#[derive(Debug)]
pub struct CPlugin {
    logger: Logger
}


impl CPlugin {
    pub fn new(logger: Logger) -> CPlugin {
        let mem = CPlugin { logger: logger };
        mem.register_probe();
        mem
    }
}

fn call_dynamic() -> lib::Result<c_int> {
    let lib = try!(lib::Library::new("/tmp/libcexampleplugin.so"));
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> c_int> = try!(lib.get(b"run_probe"));
        Ok(func())
    }
}





//extern {
//    fn my_string() -> *const c_char;
//}

//fn main() {
//
//
//    unsafe {
//        let slice = CStr::from_ptr(my_string());
//        println!("string length: {}", slice.to_bytes().len());
//    }
//}




impl Probe for CPlugin {
    fn exec(&self) -> () {

//        let c_buf: *const c_char = unsafe { my_string() };
//        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
//        let buf: &[u8] = c_str.to_bytes();
//        let str_slice: &str = str::from_utf8(buf).unwrap();
//        let str_buf: String = str_slice.to_owned();  // if necessary
//        println!("{}", str_buf);


        match call_dynamic() {
            Ok(number) => {

//                let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
//                let buf: &[u8] = c_str.to_bytes();
//                let str_slice: &str = str::from_utf8(buf).unwrap();
//                let str_buf: String = str_slice.to_owned();  // if necessary
                println!("{}", number);

//                let msg = format!("@Thread: {} - json_string: {}",
//                                  self.get_thread_id(),
//                                  json_str
//                );
//                self.logger.log(Severity::LOG_INFO, &msg);
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