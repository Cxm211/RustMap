mod utils;
mod myfeof;
mod uInt64_toAscii;
mod globals;

use globals::*;
use utils::*;
use uInt64_toAscii::*;
use crate::bzlib::BZ2_bzWriteOpen::bz2_bz_write_open;
use crate::bzlib::bz2_bzwrite::bz2_bz_write;
use crate::bzlib::BZ2_bzWriteClose::bz2_bz_write_close64;
use myfeof::myfeof;

use libc::{fclose, fflush, fileno, ferror, fread, stderr, stdout, FILE};
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

pub fn compress_stream(stream: *mut FILE, z_stream: *mut FILE) {
   unsafe {
       let mut bzf: Option<*mut bzFile> = None;
       let mut ibuf = [0u8; 5000];
       let mut n_ibuf: Int32;
       let mut nbytes_in_lo32: UInt32 = 0;
       let mut nbytes_in_hi32: UInt32 = 0;
       let mut nbytes_out_lo32: UInt32 = 0;
       let mut nbytes_out_hi32: UInt32 = 0;
       let mut bzerr: Int32 = 0;
       let mut bzerr_dummy: Int32;
       let mut ret: Int32;

       if ferror(stream) != 0 {
           goto_errhandler_io();
       }
       if ferror(z_stream) != 0 {
           goto_errhandler_io();
       }

       bzf = bz2_bz_write_open(
           Some(&mut bzerr),
           z_stream,
           blockSize100k,
           verbosity,
           workFactor,
       );

       if bzerr != 0 || bzf.is_none() {
           goto_errhandler();
       }

       if verbosity >= 2 {
           eprintln!();
       }

       loop {
           if myfeof(stream) == True {
               break;
           }
           n_ibuf = fread(
               ibuf.as_mut_ptr() as *mut c_void,
               std::mem::size_of::<u8>(),
               ibuf.len(),
               stream,
           ) as Int32;

           if ferror(stream) != 0 {
               goto_errhandler_io();
           }
           if n_ibuf > 0 {
               bz2_bz_write(
                   Some(&mut bzerr),
                   bzf.as_mut(),
                   Some(&ibuf[..n_ibuf as usize]),
                   n_ibuf,
               );
               if bzerr != 0 {
                   goto_errhandler();
               }
           }
       }

       bz2_bz_write_close64(
           Some(&mut bzerr),
           bzf,
           0,
           Some(&mut nbytes_in_lo32),
           Some(&mut nbytes_in_hi32),
           Some(&mut nbytes_out_lo32),
           Some(&mut nbytes_out_hi32),
       );

       if bzerr != 0 {
           goto_errhandler();
       }

       if ferror(z_stream) != 0 {
           goto_errhandler_io();
       }

       ret = fflush(z_stream);
       if ret == -1 {
           goto_errhandler_io();
       }

       if z_stream != stdout {
           let fd = fileno(z_stream);
           if fd < 0 {
               goto_errhandler_io();
           }
           apply_saved_file_attr_to_output_file(fd);
           ret = fclose(z_stream);
           outputHandleJustInCase = ptr::null_mut();
           if ret == -1 {
               goto_errhandler_io();
           }
       }

       outputHandleJustInCase = ptr::null_mut();

       if ferror(stream) != 0 {
           goto_errhandler_io();
       }

       ret = fclose(stream);
       if ret == -1 {
           goto_errhandler_io();
       }

       if verbosity >= 1 {
           if nbytes_in_lo32 == 0 && nbytes_in_hi32 == 0 {
               eprintln!(" no data compressed.");
           } else {
               let mut buf_nin = [0u8; 32];
               let mut buf_nout = [0u8; 32];
               let mut nbytes_in = UInt64 { b: [0; 8] };
               let mut nbytes_out = UInt64 { b: [0; 8] };
               let nbytes_in_d: f64;
               let nbytes_out_d: f64;

               u_int64_from_uint32s(&mut nbytes_in, nbytes_in_lo32, nbytes_in_hi32);
               u_int64_from_uint32s(&mut nbytes_out, nbytes_out_lo32, nbytes_out_hi32);

               nbytes_in_d = u_int64_to_double(&nbytes_in);
               nbytes_out_d = u_int64_to_double(&nbytes_out);

               u_int64_to_ascii(buf_nin.as_mut_ptr() as *mut c_char, &nbytes_in);
               u_int64_to_ascii(buf_nout.as_mut_ptr() as *mut c_char, &nbytes_out);

               eprintln!(
                   "{:.3}:1, {:.3} bits/byte, {:.2}% saved, {} in, {} out.",
                   nbytes_in_d / nbytes_out_d,
                   (8.0 * nbytes_out_d) / nbytes_in_d,
                   100.0 * (1.0 - nbytes_out_d / nbytes_in_d),
                   CStr::from_ptr(buf_nin.as_ptr() as *const c_char).to_str().unwrap_or(""),
                   CStr::from_ptr(buf_nout.as_ptr() as *const c_char).to_str().unwrap_or("")
               );
           }
       }

       return;

       // Error handlers
       fn goto_errhandler() {
           unsafe {
               bz2_bz_write_close64(
                   Some(&mut bzerr_dummy),
                   bzf,
                   1,
                   Some(&mut nbytes_in_lo32),
                   Some(&mut nbytes_in_hi32),
                   Some(&mut nbytes_out_lo32),
                   Some(&mut nbytes_out_hi32),
               );

               match bzerr {
                   -9 => config_error(),
                   -3 => out_of_memory(),
                   -6 | _ => goto_errhandler_io(),
               }

               panic(CString::new("compress:end").unwrap().as_ptr());
           }
       }

       fn goto_errhandler_io() {
           io_error();
       }
   }
}