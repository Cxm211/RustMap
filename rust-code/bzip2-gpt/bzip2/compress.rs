mod utils;
mod myfeof;
mod uInt64_toAscii;
mod compressStream;
mod globals;

use globals::*;
use utils::*;
use uInt64_toAscii::*;
use compressStream::compress_stream;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use libc::{strcat, fflush, fclose, fopen, remove, stderr, stdout, strerror, FILE, fileno, isatty};
use std::ptr;
use libc;


pub fn compress(name: *mut c_char) {
    unsafe {
        let mut in_str: *mut FILE = ptr::null_mut();
        let mut out_str: *mut FILE = ptr::null_mut();
        let mut n: Int32;
        let mut i: Int32;
        let mut stat_buf = std::mem::zeroed::<libc::stat>();

        deleteOutputOnInterrupt = False;

        if name.is_null() && srcMode != SM_I2O {
            panic(CString::new("compress: bad modes\n").unwrap().as_ptr());
        }

        match srcMode {
            SM_I2O => {
                copy_file_name(inName.as_mut_ptr(), b"(stdin)\0".as_ptr() as *const c_char);
                copy_file_name(outName.as_mut_ptr(), b"(stdout)\0".as_ptr() as *const c_char);
            }
            SM_F2F => {
                copy_file_name(inName.as_mut_ptr(), name);
                copy_file_name(outName.as_mut_ptr(), name);
                strcat(outName.as_mut_ptr(), b".bz2\0".as_ptr() as *const c_char);
            }
            SM_F2O => {
                copy_file_name(inName.as_mut_ptr(), name);
                copy_file_name(outName.as_mut_ptr(), b"(stdout)\0".as_ptr() as *const c_char);
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
                "{}: Can't open input file {}: {}.",
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(libc::strerror(*libc::__errno_location())).to_str().unwrap_or("<invalid>")
            );
            set_exit(1);
            return;
        }

        for i in 0..4 {
            if hasSuffix(inName.as_mut_ptr(), Z_SUFFIX[i]) == True {
                if noisy == True {
                    eprintln!(
                        "{}: Input file {} already has {} suffix.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        Z_SUFFIX[i]
                    );
                }
                set_exit(1);
                return;
            }
        }

        if srcMode == SM_F2F || srcMode == SM_F2O {
            if libc::stat(inName.as_ptr(), &mut stat_buf) != 0 {
                panic(CString::new("Failed to stat input file").unwrap().as_ptr());
            }

            if stat_buf.st_mode & libc::S_IFMT == libc::S_IFDIR {
                eprintln!(
                    "{}: Input file {} is a directory.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                    CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>")
                );
                set_exit(1);
                return;
            }
        }

        if srcMode == SM_F2F && forceOverwrite == False && not_a_standard_file(inName.as_mut_ptr()) == True {
            if noisy == True {
                eprintln!(
                    "{}: Input file {} is not a normal file.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                    CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>")
                );
            }
            set_exit(1);
            return;
        }

        if srcMode == SM_F2F && file_exists(outName.as_mut_ptr()) == True {
            if forceOverwrite == True {
                remove(outName.as_ptr());
            } else {
                eprintln!(
                    "{}: Output file {} already exists.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                    CStr::from_ptr(outName.as_ptr()).to_str().unwrap_or("<invalid>")
                );
                set_exit(1);
                return;
            }
        }

        if srcMode == SM_F2F && forceOverwrite == False && (n = count_hard_links(inName.as_mut_ptr())) > 0 {
            eprintln!(
                "{}: Input file {} has {} other link{}.",
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                n,
                if n > 1 { "s" } else { "" }
            );
            set_exit(1);
            return;
        }

        if srcMode == SM_F2F {
            save_input_file_meta_info(inName.as_mut_ptr());
        }

        match srcMode {
            SM_I2O => {
                in_str = libc::stdin;
                out_str = libc::stdout;
                if libc::isatty(fileno(libc::stdout)) != 0 {
                    eprintln!(
                        "{}: I won't write compressed data to a terminal.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    );
                    eprintln!("{}: For help, type: `{} --help'.", progName, progName);
                    set_exit(1);
                    return;
                }
            }
            SM_F2O => {
                in_str = fopen(inName.as_ptr(), b"rb\0".as_ptr() as *const c_char);
                out_str = libc::stdout;
                if libc::isatty(fileno(libc::stdout)) != 0 {
                    eprintln!(
                        "{}: I won't write compressed data to a terminal.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    );
                    eprintln!("{}: For help, type: `{} --help'.", progName, progName);
                    if !in_str.is_null() {
                        fclose(in_str);
                    }
                    set_exit(1);
                    return;
                }
                if in_str.is_null() {
                    eprintln!(
                        "{}: Can't open input file {}: {}.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(libc::strerror(*libc::__errno_location())).to_str().unwrap_or("<invalid>")
                    );
                    set_exit(1);
                    return;
                }
            }
            SM_F2F => {
                in_str = fopen(inName.as_ptr(), b"rb\0".as_ptr() as *const c_char);
                out_str = fopen_output_safely(outName.as_mut_ptr(), b"wb\0".as_ptr() as *const c_char);
                if out_str.is_null() {
                    eprintln!(
                        "{}: Can't create output file {}: {}.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(outName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(libc::strerror(*libc::__errno_location())).to_str().unwrap_or("<invalid>")
                    );
                    if !in_str.is_null() {
                        fclose(in_str);
                    }
                    set_exit(1);
                    return;
                }
                if in_str.is_null() {
                    eprintln!(
                        "{}: Can't open input file {}: {}.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(libc::strerror(*libc::__errno_location())).to_str().unwrap_or("<invalid>")
                    );
                    if !out_str.is_null() {
                        fclose(out_str);
                    }
                    set_exit(1);
                    return;
                }
            }
            _ => {
                panic(CString::new("compress: bad srcMode").unwrap().as_ptr());
            }
        }

        if verbosity >= 1 {
            eprint!("  {}: ", CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"));
            pad(inName.as_mut_ptr());
            fflush(stderr);
        }

        outputHandleJustInCase = out_str;
        deleteOutputOnInterrupt = True;
        compress_stream(in_str, out_str);
        outputHandleJustInCase = ptr::null_mut();

        if srcMode == SM_F2F {
            apply_saved_time_info_to_output_file(outName.as_mut_ptr());
            deleteOutputOnInterrupt = False;
            if keepInputFiles == False {
                let ret_val = remove(inName.as_ptr());
                error_if_not_zero(ret_val);
            }
        }

        deleteOutputOnInterrupt = False;
    }
}
