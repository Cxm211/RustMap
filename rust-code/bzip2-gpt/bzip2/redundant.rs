mod globals;

use globals::*;
use std::ffi::CStr;
use libc::stderr;
use std::os::raw::c_char;

pub fn redundant(flag: *const c_char) {
    unsafe {
        eprintln!(
            "{}: {} is redundant in versions 0.9.5 and above",
            CStr::from_ptr(progName)
                .to_str()
                .unwrap_or("<invalid>"),
            CStr::from_ptr(flag)
                .to_str()
                .unwrap_or("<invalid>")
        );
    }
}
