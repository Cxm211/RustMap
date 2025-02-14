 
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use libc::{open, fdopen, O_WRONLY, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR};
use libc::{lstat, stat, S_IFREG, S_IFMT};
use libc::{utime, utimbuf};
use libc::{fchmod, fchown};
use libc::strcat;
use libc;

pub fn show_file_names() {
    unsafe {
        if noisy == True {
            eprintln!(
                "\tInput file = {}, output file = {}",
                String::from_utf8_lossy(&inName),
                String::from_utf8_lossy(&outName)
            );
        }
    }
}

pub fn set_exit(v: Int32) {
    unsafe {
        if v > exitValue {
            exitValue = v;
        }
    }
}

pub fn cadvise() {
    unsafe {
        if noisy == True {
            eprintln!(
                "\nIt is possible that the compressed file(s) have become corrupted.\n\
                 You can use the -tvv option to test integrity of such files.\n\n\
                 You can use the `bzip2recover' program to attempt to recover\n\
                 data from undamaged sections of corrupted files.\n"
            );
        }
    }
}

pub fn clean_up_and_fail(ec: Int32) {
    unsafe {
        let mut ret_val: IntNative;
        let mut stat_buf: libc::stat = std::mem::zeroed();

        if srcMode == SM_F2F && opMode != OM_TEST && deleteOutputOnInterrupt == True {
            ret_val = libc::stat(inName.as_ptr() as *const c_char, &mut stat_buf);

            if ret_val == 0 {
                if noisy == True {
                    let prog_name_str = if progName.is_null() {
                        "<unknown>"
                    } else {
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    };

                    eprintln!(
                        "{}: Deleting output file {}, if it exists.",
                        prog_name_str,
                        String::from_utf8_lossy(&outName)
                    );
                }

                if outputHandleJustInCase != ptr::null_mut() {
                    libc::fclose(outputHandleJustInCase);
                }

                ret_val = libc::remove(outName.as_ptr() as *const c_char);

                if ret_val != 0 {
                    eprintln!(
                        "{}: WARNING: deletion of output file (apparently) failed.",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                    );
                }
            } else {
                eprintln!(
                    "{}: WARNING: deletion of output file suppressed",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                );
                eprintln!(
                    "{}:    since input file no longer exists. Output file",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                );
                eprintln!(
                    "{}:    `{}` may be incomplete.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                    String::from_utf8_lossy(&outName)
                );
                eprintln!(
                    "{}:    I suggest doing an integrity test (bzip2 -tv) of it.",
                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
                );
            }
        }

        if noisy == True && numFileNames > 0 && numFilesProcessed < numFileNames {
            eprintln!(
                "{}: WARNING: some files have not been processed:\n{}:    {} specified on command line, {} not processed yet.\n",
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                numFileNames,
                numFileNames - numFilesProcessed
            );
        }

        set_exit(ec);
        std::process::exit(exitValue);
    }
}




pub fn panic(s: *const Char) {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        eprintln!(
            "\n{}: PANIC -- internal consistency error:\n\t{}\n\tThis is a BUG. Please report it to:\n\tbzip2-devel@sourceware.org\n",
            prog_name_str,
            CStr::from_ptr(s).to_str().unwrap_or("<invalid>")
        );

        show_file_names();
        clean_up_and_fail(3);
    }
}


pub fn io_error() {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        eprintln!(
            "\n{}: I/O or other error, bailing out. Possible reason follows.",
            prog_name_str
        );

        // Rust equivalent for `perror` which prints the last OS error.
        eprintln!("{}: {}", prog_name_str, std::io::Error::last_os_error());

        show_file_names();
        clean_up_and_fail(1);
    }
}

pub fn error_if_eof(i: i32) {
    if i == -1 {
        io_error();
    }
}

pub fn error_if_not_zero(i: i32) {
    if i != 0 {
        io_error();
    }
}

pub fn error_if_minus_one(i: i32) {
    if i == -1 {
        io_error();
    }
}


pub fn out_of_memory() {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        eprintln!(
            "\n{}: couldn't allocate enough memory",
            prog_name_str
        );

        show_file_names();
        clean_up_and_fail(1);
    }
}

pub fn config_error() {
    unsafe {
        eprintln!(
            "bzip2: I'm not configured correctly for this platform!\n\
             \tI require Int32, Int16, and Char to have sizes\n\
             \tof 4, 2, and 1 bytes to run properly, and they don't.\n\
             \tProbably you can fix this by defining them correctly,\n\
             \tand recompiling.  Bye!"
        );

        set_exit(3);
        std::process::exit(exitValue);
    }
}

pub fn crc_error() {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        eprintln!("\n{}: Data integrity error when decompressing.", prog_name_str);

        show_file_names();
        cadvise();
        clean_up_and_fail(2);
    }
}

pub fn my_signal_catcher(n: IntNative) {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        eprintln!("\n{}: Control-C or similar caught, quitting.", prog_name_str);

        clean_up_and_fail(1);
    }
}

pub fn my_sigsegv_or_sigbus_catcher(n: IntNative) {
    unsafe {
        let prog_name_str = if progName.is_null() {
            "<unknown>"
        } else {
            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
        };

        if opMode == OM_Z {
            eprintln!(
                "\n{}: Caught a SIGSEGV or SIGBUS whilst compressing.\n\
                \n\
                Possible causes are (most likely first):\n\
                (1) This computer has unreliable memory or cache hardware\n\
                (a surprisingly common problem; try a different machine.)\n\
                (2) A bug in the compiler used to create this executable\n\
                (unlikely, if you didn't compile bzip2 yourself.)\n\
                (3) A real bug in bzip2 -- I hope this should never be the case.\n\
                The user's manual, Section 4.3, has more info on (1) and (2).\n\
                \n\
                If you suspect this is a bug in bzip2, or are unsure about (1)\n\
                or (2), feel free to report it to: bzip2-devel@sourceware.org.\n\
                Section 4.3 of the user's manual describes the info a useful\n\
                bug report should have.  If the manual is available on your\n\
                system, please try and read it before mailing me.  If you don't\n\
                have the manual or can't be bothered to read it, mail me anyway.\n",
                prog_name_str
            );
        } else {
            eprintln!(
                "\n{}: Caught a SIGSEGV or SIGBUS whilst decompressing.\n\
                \n\
                Possible causes are (most likely first):\n\
                (1) The compressed data is corrupted, and bzip2's usual checks\n\
                failed to detect this.  Try bzip2 -tvv my_file.bz2.\n\
                (2) This computer has unreliable memory or cache hardware\n\
                (a surprisingly common problem; try a different machine.)\n\
                (3) A bug in the compiler used to create this executable\n\
                (unlikely, if you didn't compile bzip2 yourself.)\n\
                (4) A real bug in bzip2 -- I hope this should never be the case.\n\
                The user's manual, Section 4.3, has more info on (2) and (3).\n\
                \n\
                If you suspect this is a bug in bzip2, or are unsure about (2)\n\
                or (3), feel free to report it to: bzip2-devel@sourceware.org.\n\
                Section 4.3 of the user's manual describes the info a useful\n\
                bug report should have.  If the manual is available on your\n\
                system, please try and read it before mailing me.  If you don't\n\
                have the manual or can't be bothered to read it, mail me anyway.\n",
                prog_name_str
            );
        }

        show_file_names();

        if opMode == OM_Z {
            clean_up_and_fail(3);
        } else {
            cadvise();
            clean_up_and_fail(2);
        }
    }
}


pub fn compressed_stream_eof() {
    unsafe {
        if noisy == True {
            let prog_name_str = if progName.is_null() {
                "<unknown>"
            } else {
                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>")
            };

            eprintln!(
                "\n{}: Compressed file ends unexpectedly;\n\t\
                 perhaps it is corrupted?  *Possible* reason follows.",
                prog_name_str
            );

            // Rust equivalent for `perror` to print the last OS error.
            eprintln!("{}: {}", prog_name_str, std::io::Error::last_os_error());

            show_file_names();
            cadvise();
        }
        clean_up_and_fail(2);
    }
}



pub fn pad(s: *const c_char) {
    unsafe {
        let str_len = CStr::from_ptr(s).to_bytes().len() as Int32;
        if str_len >= longestFileName {
            return;
        }
        
        let padding_len = longestFileName - str_len;
        for _ in 0..padding_len {
            eprint!(" ");
        }
    }
}

pub fn copy_file_name(to: *mut c_char, from: *const c_char) {
    unsafe {
        let from_len = CStr::from_ptr(from).to_bytes().len();
        if from_len > 1034 - 10 {
            eprintln!(
                "bzip2: file name\n`{}`\nis suspiciously (more than {} chars) long.\n\
                 Try using a reasonable file name instead. Sorry! :-)",
                CStr::from_ptr(from).to_str().unwrap_or("<invalid>"),
                1034 - 10
            );
            set_exit(1);
            std::process::exit(exitValue);
        }

        ptr::copy_nonoverlapping(from, to, 1034 - 10);
        *to.add(1034 - 10) = 0;
    }
}

pub fn file_exists(name: *const c_char) -> Bool {
    unsafe {
        let tmp = libc::fopen(name, b"rb\0".as_ptr() as *const c_char);
        let exists = if !tmp.is_null() {
            True
        } else {
            False
        };

        if !tmp.is_null() {
            libc::fclose(tmp);
        }

        exists
    }
}

pub fn fopen_output_safely(name: *const c_char, mode: *const c_char) -> *mut libc::FILE {
    unsafe {
        let fh = open(
            name,
            O_WRONLY | O_CREAT | O_EXCL,
            S_IRUSR | S_IWUSR,
        );

        if fh == -1 {
            return ptr::null_mut();
        }

        let fp = fdopen(fh, mode);
        if fp.is_null() {
            libc::close(fh);
        }

        fp
    }
}

pub fn not_a_standard_file(name: *const c_char) -> Bool {
    unsafe {
        let mut stat_buf: stat = std::mem::zeroed();

        let i = lstat(name, &mut stat_buf);
        if i != 0 {
            return True;
        }

        if (stat_buf.st_mode & S_IFMT) == S_IFREG {
            return False;
        }

        True
    }
}

pub fn count_hard_links(name: *const c_char) -> Int32 {
    unsafe {
        let mut stat_buf: stat = std::mem::zeroed();

        let i = lstat(name, &mut stat_buf);
        if i != 0 {
            return 0;
        }

        (stat_buf.st_nlink - 1) as Int32
    }
}

pub fn save_input_file_meta_info(src_name: *const c_char) {
    unsafe {
        let ret_val = stat(src_name, &mut fileMetaInfo);
        error_if_not_zero(ret_val);
    }
}

pub fn apply_saved_time_info_to_output_file(dst_name: *const c_char) {
    unsafe {
        let mut u_tim_buf = utimbuf {
            actime: fileMetaInfo.st_atime as i64,
            modtime: fileMetaInfo.st_mtime as i64,
        };

        let ret_val = utime(dst_name, &u_tim_buf);
        error_if_not_zero(ret_val);
    }
}

pub fn apply_saved_file_attr_to_output_file(fd: IntNative) {
    unsafe {
        let ret_val = fchmod(fd, fileMetaInfo.st_mode);
        error_if_not_zero(ret_val);

        fchown(fd, fileMetaInfo.st_uid, fileMetaInfo.st_gid);
    }
}

pub fn contains_dubious_chars(_name: *const c_char) -> Bool {
    False
}

pub fn hasSuffix(s: *const c_char, suffix: *const c_char) -> Bool {
    unsafe {
        let s_str = CStr::from_ptr(s).to_str().unwrap_or("");
        let suffix_str = CStr::from_ptr(suffix).to_str().unwrap_or("");

        let ns = s_str.len() as Int32;
        let nx = suffix_str.len() as Int32;

        if ns < nx {
            return False;
        }
        if s_str.ends_with(suffix_str) {
            return True;
        }
        False
    }
}

pub fn map_suffix(name: *mut c_char, oldSuffix: *const c_char, newSuffix: *const c_char) -> Bool {
    unsafe {
        if hasSuffix(name, oldSuffix) == False {
            return False;
        }

        let name_len = libc::strlen(name);
        let old_suffix_len = libc::strlen(oldSuffix);

        *name.add(name_len - old_suffix_len) = 0; // Null-terminate the string

        strcat(name, newSuffix);

        True
    }
}