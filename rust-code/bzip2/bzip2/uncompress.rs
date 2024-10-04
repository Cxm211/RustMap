mod utils;
mod myfeof;
mod uInt64_toAscii;
mod uncompressStream;
mod globals;

use libc;
use globals::*;
use utils::*;
use uInt64_toAscii::*;
use uncompressStream::uncompress_stream;;
use libc::{strcat,fclose, fileno, ferror, fflush, fopen, remove, stderr, stdout, FILE};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

pub fn uncompress(name: *mut c_char) {
    unsafe {
        let mut in_str: *mut FILE = ptr::null_mut();
        let mut out_str: *mut FILE = ptr::null_mut();
        let mut n: Int32;
        let mut i: Int32;
        let mut magic_number_ok: Bool;
        let mut cant_guess: Bool;
        let mut stat_buf = std::mem::zeroed::<libc::stat>();

        deleteOutputOnInterrupt = False;

        if name.is_null() && srcMode != SM_I2O {
            panic(CString::new("uncompress: bad modes\n").unwrap().as_ptr());
        }

        cant_guess = False;
        match srcMode {
            SM_I2O => {
                copy_file_name(inName.as_mut_ptr(), b"(stdin)\0".as_ptr() as *const c_char);
                copy_file_name(outName.as_mut_ptr(), b"(stdout)\0".as_ptr() as *const c_char);
            }
            SM_F2F => {
                copy_file_name(inName.as_mut_ptr(), name);
                copy_file_name(outName.as_mut_ptr(), name);
                for i in 0..4 {
                    if map_suffix(outName.as_mut_ptr(), Z_SUFFIX[i], UNZ_SUFFIX[i]) == True {
                        goto_zzz();
                        return;
                    }
                }
                cant_guess = True;
                strcat(outName.as_mut_ptr(), b".out\0".as_ptr() as *const c_char);
            }
            SM_F2O => {
                copy_file_name(inName.as_mut_ptr(), name);
                copy_file_name(outName.as_mut_ptr(), b"(stdout)\0".as_ptr() as *const c_char);
            }
            _ => {}
        }

        fn goto_zzz() {
            unsafe {
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
                        CStr::from_ptr(libc::strerror(*libc::__errno_location()))
                            .to_str()
                            .unwrap_or("<invalid>")
                    );
                    set_exit(1);
                    return;
                }

                if (srcMode == SM_F2F || srcMode == SM_F2O)
                    && libc::stat(inName.as_ptr(), &mut stat_buf) == 0
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

                if cant_guess == True && noisy == True {
                    eprintln!(
                        "{}: Can't guess original name for {} -- using {}",
                        CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"),
                        CStr::from_ptr(outName.as_ptr()).to_str().unwrap_or("<invalid>")
                    );
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
                        if libc::isatty(fileno(libc::stdin)) != 0 {
                            eprintln!(
                                "{}: I won't read compressed data from a terminal.",
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
                    SM_F2F => {
                        in_str = fopen(inName.as_ptr(), b"rb\0".as_ptr() as *const c_char);
                        out_str = fopen_output_safely(outName.as_mut_ptr(), b"wb\0".as_ptr() as *const c_char);
                        if out_str.is_null() {
                            eprintln!(
                                "{}: Can't create output file {}: {}.",
                                CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                                CStr::from_ptr(outName.as_ptr()).to_str().unwrap_or("<invalid>"),
                                CStr::from_ptr(libc::strerror(*libc::__errno_location()))
                                    .to_str()
                                    .unwrap_or("<invalid>")
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
                                CStr::from_ptr(libc::strerror(*libc::__errno_location()))
                                    .to_str()
                                    .unwrap_or("<invalid>")
                            );
                            if !out_str.is_null() {
                                fclose(out_str);
                            }
                            set_exit(1);
                            return;
                        }
                    }
                    _ => {
                        panic(CString::new("uncompress: bad srcMode").unwrap().as_ptr());
                    }
                }

                if verbosity >= 1 {
                    eprint!("  {}: ", CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>"));
                    pad(inName.as_mut_ptr());
                    fflush(stderr);
                }

                outputHandleJustInCase = out_str;
                deleteOutputOnInterrupt = True;
                magic_number_ok = uncompress_stream(in_str, out_str);
                outputHandleJustInCase = ptr::null_mut();

                if magic_number_ok == True {
                    if srcMode == SM_F2F {
                        apply_saved_time_info_to_output_file(outName.as_mut_ptr());
                        deleteOutputOnInterrupt = False;
                        if keepInputFiles == False {
                            let ret_val = remove(inName.as_ptr());
                            error_if_not_zero(ret_val);
                        }
                    }
                } else {
                    unzFailsExist = True;
                    deleteOutputOnInterrupt = False;
                    if srcMode == SM_F2F {
                        let ret_val = remove(outName.as_ptr());
                        error_if_not_zero(ret_val);
                    }
                }

                deleteOutputOnInterrupt = False;

                if magic_number_ok == True {
                    if verbosity >= 1 {
                        eprintln!("done");
                    }
                } else {
                    set_exit(2);
                    if verbosity >= 1 {
                        eprintln!("not a bzip2 file.");
                    } else {
                        eprintln!(
                            "{}: {} is not a bzip2 file.",
                            CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                            CStr::from_ptr(inName.as_ptr()).to_str().unwrap_or("<invalid>")
                        );
                    }
                }
            }
        }

        goto_zzz();
    }
}
