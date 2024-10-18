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

use libc::{fclose, ferror, stderr, FILE};
use std::os::raw::{c_void, c_char};
use std::ptr;

pub fn test_stream(z_stream: *mut FILE) -> Bool {
   unsafe {
       let mut bzf: Option<*mut bzFile> = None;
       let mut bzerr: Int32 = 0;
       let mut bzerr_dummy: Int32;
       let mut ret: Int32;
       let mut stream_no: Int32 = 0;
       let mut i: Int32;
       let mut obuf = [0u8; 5000];
       let mut unused = [0u8; 5000];
       let mut n_unused: Int32 = 0;
       let mut unused_tmp: *mut UChar = ptr::null_mut();

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
               bz2_bz_read(Some(&mut bzerr), bzf.unwrap(), &mut obuf, 5000);
               if bzerr == -5 {
                   goto_errhandler();
               }
           }

           if bzerr != 4 {
               goto_errhandler();
           }

           bz2_bz_read_get_unused(Some(&mut bzerr), bzf.unwrap(), &mut unused_tmp as *mut _, &mut n_unused);
           if bzerr != 0 {
               panic(CString::new("test:bzReadGetUnused").unwrap().as_ptr());
           }

           for i in 0..n_unused {
               unused[i as usize] = *unused_tmp.add(i as usize);
           }

           bz2_bz_read_close(Some(&mut bzerr), bzf);
           if bzerr != 0 {
               panic(CString::new("test:bzReadGetUnused").unwrap().as_ptr());
           }

           if n_unused == 0 && myfeof(z_stream) == True {
               break;
           }
       }

       if ferror(z_stream) != 0 {
           goto_errhandler_io();
       }
       ret = fclose(z_stream);
       if ret == -1 {
           goto_errhandler_io();
       }

       if verbosity >= 2 {
           eprintln!("\n    ");
       }

       return True;

       // Error handlers
       fn goto_errhandler() {
           unsafe {
               bz2_bz_read_close(Some(&mut bzerr_dummy), bzf);
               if verbosity == 0 {
                   eprint!(
                       "{}: {}: ",
                       CStr::from_ptr(progName).to_str().unwrap_or("<invalid>"),
                       CStr::from_ptr(inName).to_str().unwrap_or("<invalid>")
                   );
               }

               match bzerr {
                   -9 => config_error(),
                   -6 => goto_errhandler_io(),
                   -4 => {
                       eprintln!("data integrity (CRC) error in data");
                       return False;
                   }
                   -3 => out_of_memory(),
                   -7 => {
                       eprintln!("file ends unexpectedly");
                       return False;
                   }
                   -5 => {
                       if z_stream != libc::stdin {
                           fclose(z_stream);
                       }
                       if stream_no == 1 {
                           eprintln!("bad magic number (file not created by bzip2)");
                           return False;
                       } else {
                           if noisy == True {
                               eprintln!("trailing garbage after EOF ignored");
                           }
                           return True;
                       }
                   }
                   _ => panic(CString::new("test:unexpected error").unwrap().as_ptr()),
               }
           }
       }

       fn goto_errhandler_io() {
           io_error();
       }
   }
}