mod globals;
mod utils;
mod myfeof;
mod uInt64_toAscii;
mod testStream

use globals::*;
use utils::*;
use uInt64_toAscii::*;
use testStream::test_stream;

use libc::{fclose, fileno, ferror, fflush, fopen, stderr, stdout, FILE};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub fn testf(name: *mut c_char) {
    unsafe {
        let mut in_str: *mut FILE = ptr::null_mut();
        let mut all_ok: Bool;
        let mut stat_buf = std::mem::zeroed::<libc::stat>();

        deleteOutputOnInterrupt = False;

        if name.is_null() && srcMode != SM_I2O {
            panic(CString::new("testf: bad modes\n").unwrap().as_ptr());
        }

        copy_file_name(outName.as_mut_ptr(), b"(none)\0".as_ptr() as *const c_char);
        match srcMode {
            SM_I2O => {
                copy_file_name(inName.as_mut_ptr(), b"(stdin)\0".as_ptr() as *const c_char);
            }
            SM_F2F | SM_F2O => {
                copy_file_name(inName.as_mut_ptr(), name);
            }
            _ => {}
        }

        if srcMode != SM_I2O && contains_dubious_chars(inName.as_mut_ptr()) == True {
            if noisy == True {
                eprintln!(
                    "{}: There are no files matching `{}`.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                    CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>")
                );
            }
            set_exit(1);
            return;
        }

        if srcMode != SM_I2O && file_exists(inName.as_mut_ptr()) == False {
            eprintln!(
                "{}: Can't open input {}: {}.",
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(libc::strerror(*libc::__errno_location()))
                    .to_str()
                    .unwrap_or("<invalid>")
            );
            set_exit(1);
            return;
        }

        if srcMode != SM_I2O && libc::stat(inName.as_ptr(), &mut stat_buf) == 0
            && stat_buf.st_mode & libc::S_IFMT == libc::S_IFDIR
        {
            eprintln!(
                "{}: Input file {} is a directory.",
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>")
            );
            set_exit(1);
            return;
        }

        match srcMode {
            SM_I2O => {
                if libc::isatty(fileno(libc::stdin)) != 0 {
                    eprintln!(
                        "{}: I won't read compressed data from a terminal.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    );
                    eprintln!(
                        "{}: For help, type: `{}` --help.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    );
                    set_exit(1);
                    return;
                }
                in_str = libc::stdin;
            }
            SM_F2O | SM_F2F => {
                in_str = fopen(inName.as_ptr(), b"rb\0".as_ptr() as *const c_char);
                if in_str.is_null() {
                    eprintln!(
                        "{}: Can't open input file {}: {}.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(libc::strerror(*libc::__errno_location()))
                            .to_str()
                            .unwrap_or("<invalid>")
                    );
                    set_exit(1);
                    return;
                }
            }
            _ => {
                panic(CString::new("testf: bad srcMode").unwrap().as_ptr());
            }
        }

        if verbosity >= 1 {
            eprint!("  {}: ", CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"));
            pad(inName.as_mut_ptr());
            fflush(stderr);
        }

        outputHandleJustInCase = ptr::null_mut();
        all_ok = test_stream(in_str);

        if all_ok == True && verbosity >= 1 {
            eprintln!("ok");
        }
        if all_ok == False {
            testFailsExist = True;
        }
    }
}
