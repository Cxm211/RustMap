mod utils;
mod myfeof;
mod uInt64_toAscii;
mod globals;

use globals::*;
use utils::*;
use uInt64_toAscii::*;
use crate::bzlib::BZ2_bzReadOpen::bz2_bz_read_open;
use crate::bzlib::bz2_bzread::bz2_bz_read;
use crate::bzlib::BZ2_bzReadGetUnused::bz2_bz_read_get_unused;
use crate::bzlib::BZ2_bzReadClose::bz2_bz_read_close;
use myfeof::myfeof;

use libc::{fclose, fileno, ferror, fread, fwrite, rewind, stderr, stdout, FILE};
use std::os::raw::{c_void, c_char};
use std::ptr;


pub fn uncompress_stream(z_stream: *mut FILE, stream: *mut FILE) -> Bool {
    unsafe {
        let mut bzf: Option<*mut bzFile> = None;
        let mut bzerr: Int32 = 0;
        let mut bzerr_dummy: Int32;
        let mut ret: Int32;
        let mut nread: Int32;
        let mut stream_no: Int32 = 0;
        let mut i: Int32;
        let mut obuf = [0u8; 5000];
        let mut unused = [0u8; 5000];
        let mut n_unused: Int32 = 0;
        let mut unused_tmp: *mut UChar = ptr::null_mut();

        if ferror(stream) != 0 {
            goto_errhandler_io();
        }
        if ferror(z_stream) != 0 {
            goto_errhandler_io();
        }

        loop {
            bzf = bz2_bz_read_open(
                Some(&mut bzerr),
                z_stream,
                verbosity,
                smallMode as i32,
                Some(&unused[..n_unused as usize]),
                n_unused,
            );

            if bzf.is_none() || bzerr != 0 {
                goto_errhandler();
            }

            stream_no += 1;

            while bzerr == 0 {
                nread = bz2_bz_read(Some(&mut bzerr), bzf.unwrap(), &mut obuf, 5000);
                if bzerr == -5 {
                    goto_trycat();
                }
                if (bzerr == 0 || bzerr == 4) && nread > 0 {
                    fwrite(obuf.as_ptr() as *const c_void, 1, nread as usize, stream);
                }
                if ferror(stream) != 0 {
                    goto_errhandler_io();
                }
            }

            if bzerr != 4 {
                goto_errhandler();
            }

            bz2_bz_read_get_unused(Some(&mut bzerr), bzf.unwrap(), &mut unused_tmp as *mut _, &mut n_unused);
            if bzerr != 0 {
                panic(CString::new("decompress:bzReadGetUnused").unwrap().as_ptr());
            }

            for i in 0..n_unused {
                unused[i as usize] = *unused_tmp.add(i as usize);
            }

            bz2_bz_read_close(Some(&mut bzerr), bzf);
            if bzerr != 0 {
                panic(CString::new("decompress:bzReadGetUnused").unwrap().as_ptr());
            }

            if n_unused == 0 && myfeof(z_stream) == True {
                break;
            }
        }

        // Directly implementing the closeok logic here
        if ferror(z_stream) != 0 {
            goto_errhandler_io();
        }
        if stream != stdout {
            let fd = fileno(stream);
            if fd < 0 {
                goto_errhandler_io();
            }
            apply_saved_file_attr_to_output_file(fd);
        }
        ret = fclose(z_stream);
        if ret == -1 {
            goto_errhandler_io();
        }

        if ferror(stream) != 0 {
            goto_errhandler_io();
        }
        ret = libc::fflush(stream);
        if ret != 0 {
            goto_errhandler_io();
        }
        if stream != stdout {
            ret = fclose(stream);
            outputHandleJustInCase = ptr::null_mut();
            if ret == -1 {
                goto_errhandler_io();
            }
        }
        outputHandleJustInCase = ptr::null_mut();

        if verbosity >= 2 {
            eprintln!("\n    ");
        }

        return True;

        // Error handlers
        fn goto_trycat() {
            unsafe {
                if forceOverwrite == True {
                    rewind(z_stream);
                    loop {
                        if myfeof(z_stream) == True {
                            break;
                        }
                        let nread = fread(
                            obuf.as_mut_ptr() as *mut c_void,
                            std::mem::size_of::<UChar>(),
                            obuf.len(),
                            z_stream,
                        ) as Int32;

                        if ferror(z_stream) != 0 {
                            goto_errhandler_io();
                        }
                        if nread > 0 {
                            fwrite(obuf.as_ptr() as *const c_void, 1, nread as usize, stream);
                        }
                        if ferror(stream) != 0 {
                            goto_errhandler_io();
                        }
                    }

                    // Jump to the closeok code directly, instead of calling a function
                    if ferror(z_stream) != 0 {
                        goto_errhandler_io();
                    }
                    if stream != stdout {
                        let fd = fileno(stream);
                        if fd < 0 {
                            goto_errhandler_io();
                        }
                        apply_saved_file_attr_to_output_file(fd);
                    }
                    ret = fclose(z_stream);
                    if ret == -1 {
                        goto_errhandler_io();
                    }

                    if ferror(stream) != 0 {
                        goto_errhandler_io();
                    }
                    ret = libc::fflush(stream);
                    if ret != 0 {
                        goto_errhandler_io();
                    }
                    if stream != stdout {
                        ret = fclose(stream);
                        outputHandleJustInCase = ptr::null_mut();
                        if ret == -1 {
                            goto_errhandler_io();
                        }
                    }
                    outputHandleJustInCase = ptr::null_mut();

                    if verbosity >= 2 {
                        eprintln!("\n    ");
                    }

                    return True;
                } else {
                    goto_errhandler();
                }
            }
        }

        fn goto_errhandler() {
            unsafe {
                bz2_bz_read_close(Some(&mut bzerr_dummy), bzf);
                match bzerr {
                    -9 => config_error(),
                    -6 => goto_errhandler_io(),
                    -4 => crc_error(),
                    -3 => out_of_memory(),
                    -7 => compressed_stream_eof(),
                    -5 => {
                        if z_stream != libc::stdin {
                            fclose(z_stream);
                        }
                        if stream != stdout {
                            fclose(stream);
                        }
                        if stream_no == 1 {
                            return False;
                        } else {
                            if noisy == True {
                                eprintln!(
                                    "\n{}: {}: trailing garbage after EOF ignored",
                                    CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                                    CStr::from_ptr(inName).to_str().unwrap_or("<invalid>")
                                );
                            }
                            return True;
                        }
                    }
                    _ => panic(CString::new("decompress:unexpected error").unwrap().as_ptr()),
                }
            }
        }

        fn goto_errhandler_io() {
            io_error();
        }
    }
}
