
mod globals;
mod utils;
mod compress;
mod uncompress;
mod license;
mod usage;
mod redundant;
mod testf;

use globals::*;
use utils::*;
use compress::compress;
use uncompress::uncompress;
use license::license;
use usage::usage;
use redundant::redundant;
use testf::testf;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use libc::{malloc, getenv,strstr};
use libc;
use std::ptr;
use libc::{signal, SIGSEGV, SIGBUS, SIGHUP, SIGTERM, SIGINT};

pub struct Cell {
   pub name: *mut c_char,
   pub link: *mut Cell,
}

impl Cell {
   pub fn new() -> *mut Cell {
       let cell_ptr = my_malloc(std::mem::size_of::<Cell>()) as *mut Cell;
       if !cell_ptr.is_null() {
           unsafe {
               (*cell_ptr).name = ptr::null_mut();
               (*cell_ptr).link = ptr::null_mut();
           }
       }
       cell_ptr
   }
}

pub fn my_malloc(n: Int32) -> *mut c_void {
   unsafe {
       let p = malloc(n as usize);
       if p.is_null() {
           out_of_memory();
       }
       p
   }
}

pub fn snoc_string(root: *mut Cell, name: *const c_char) -> *mut Cell {
   unsafe {
       if root.is_null() {
           let tmp = Cell::new();
           (*tmp).name = my_malloc(5 + libc::strlen(name)) as *mut c_char;
           libc::strcpy((*tmp).name, name);
           tmp
       } else {
           let mut tmp = root;
           while !(*tmp).link.is_null() {
               tmp = (*tmp).link;
           }
           (*tmp).link = snoc_string((*tmp).link, name);
           root
       }
   }
}


/// Helper function to convert a C-style string (`*const c_char`) to a Rust string slice (`&str`).
/// This function is marked as `unsafe` because dereferencing a raw pointer is inherently unsafe.
pub unsafe fn c_str_to_str<'a>(c_str: *const c_char) -> &'a str {
    if c_str.is_null() {
        ""
    } else {
        CStr::from_ptr(c_str).to_str().unwrap_or("<invalid UTF-8>")
    }
}

pub fn add_flags_from_env_var(arg_list: &mut *mut Cell, var_name: *const c_char) {
   unsafe {
       let env_base = getenv(var_name);
       if !env_base.is_null() {
           let mut p = env_base;
           let mut i = 0;
           while *p != 0 {
               p = p.add(i);
               i = 0;

               while libc::isspace(*p as i32) != 0 {
                   p = p.add(1);
               }

               while *p != 0 && libc::isspace(*p as i32) == 0 {
                   i += 1;
               }

               if i > 0 {
                   let k = if i > 1034 - 10 { 1034 - 10 } else { i };
                   for j in 0..k {
                       tmpName[j as usize] = *p.add(j as usize);
                   }
                   tmpName[k as usize] = 0;
                   *arg_list = snoc_string(*arg_list, tmpName.as_ptr() as *const c_char);
               }
           }
       }
   }
}

pub fn is_flag(aa: *mut Cell, s: *const c_char) -> Bool {
   unsafe {
       if libc::strcmp((*aa).name, s) == 0 {
           True
       } else {
           False
       }
   }
}


pub fn main(argc: IntNative, argv: *const *mut c_char) {
   let mut i: Int32 = 0;
   let mut j: Int32 = 0;
   let mut decode: Bool;
   let mut tmp: *mut c_char;
   let mut arg_list: *mut Cell = ptr::null_mut();
   let mut aa: *mut Cell;

   if std::mem::size_of::<Int32>() != 4
       || std::mem::size_of::<UInt32>() != 4
       || std::mem::size_of::<Int16>() != 2
       || std::mem::size_of::<UInt16>() != 2
       || std::mem::size_of::<Char>() != 1
       || std::mem::size_of::<UChar>() != 1
   {
       config_error();
   }

   unsafe {
       outputHandleJustInCase = ptr::null_mut();
   }
   smallMode = False;
   keepInputFiles = False;
   forceOverwrite = False;
   noisy = True;
   verbosity = 0;
   blockSize100k = 9;
   testFailsExist = False;
   unzFailsExist = False;
   numFileNames = 0;
   numFilesProcessed = 0;
   workFactor = 30;
   deleteOutputOnInterrupt = False;
   exitValue = 0;

   unsafe {
       signal(SIGSEGV, my_sigsegv_or_sigbus_catcher as usize);
       signal(SIGBUS, my_sigsegv_or_sigbus_catcher as usize);
   }

   copy_file_name(inName.as_mut_ptr(), b"(none)\0".as_ptr() as *const c_char);
   copy_file_name(outName.as_mut_ptr(), b"(none)\0".as_ptr() as *const c_char);
   copy_file_name(progNameReally.as_mut_ptr(), *argv);

   unsafe {
       progName = progNameReally.as_mut_ptr();
       tmp = progName;
       while *tmp != 0 {
           if *tmp == b'/' as c_char {
               progName = tmp.add(1);
           }
           tmp = tmp.add(1);
       }
   }

   add_flags_from_env_var(&mut arg_list, b"BZIP2\0".as_ptr() as *const c_char);
   add_flags_from_env_var(&mut arg_list, b"BZIP\0".as_ptr() as *const c_char);
   for i in 1..argc {
       unsafe {
           arg_list = snoc_string(arg_list, *argv.add(i as usize));
       }
   }

   longestFileName = 7;
   numFileNames = 0;
   decode = True;
   unsafe {
       aa = arg_list;
       while !aa.is_null() {
           if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
               decode = False;
               continue;
           }
           if (*aa).name.is_null() || (*(*aa).name.offset(0) == b'-' as c_char && decode == True) {
               aa = (*aa).link;
               continue;
           }
           numFileNames += 1;
           let len = libc::strlen((*aa).name) as Int32;
           if longestFileName < len {
               longestFileName = len;
           }
           aa = (*aa).link;
       }
   }

   if numFileNames == 0 {
       srcMode = SM_I2O;
   } else {
       srcMode = SM_F2F;
   }

   opMode = OM_Z;

   unsafe {
       if strstr(progName, b"unzip\0".as_ptr() as *const c_char).is_null() == false
           || strstr(progName, b"UNZIP\0".as_ptr() as *const c_char).is_null() == false
       {
           opMode = OM_UNZ;
       }

       if strstr(progName, b"z2cat\0".as_ptr() as *const c_char).is_null() == false
           || strstr(progName, b"Z2CAT\0".as_ptr() as *const c_char).is_null() == false
           || strstr(progName, b"zcat\0".as_ptr() as *const c_char).is_null() == false
           || strstr(progName, b"ZCAT\0".as_ptr() as *const c_char).is_null() == false
       {
           opMode = OM_UNZ;
           srcMode = if numFileNames == 0 { SM_I2O } else { SM_F2O };
       }
   }
   unsafe {
      aa = arg_list;
      while !aa.is_null() {
         if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
            break;
         }
         if (*aa).name.is_null() == false && *(*aa).name == b'-' as c_char && *(*aa).name.offset(1) != b'-' as c_char {
            let mut j = 1;
            while *(*aa).name.offset(j) != b'\0' as c_char {
               match *(*aa).name.offset(j) {
                  b'c' as i8 => {
                     srcMode = SM_F2O;
                     j += 1;
                  }
                  b'd' as i8 => {
                     opMode = OM_UNZ;
                     j += 1;
                  }
                  b'z' as i8 => {
                     opMode = OM_Z;
                     j += 1;
                  }
                  b'f' as i8 => {
                     forceOverwrite = True;
                     j += 1;
                  }
                  b't' as i8 => {
                     opMode = OM_TEST;
                     j += 1;
                  }
                  b'k' as i8 => {
                     keepInputFiles = True;
                     j += 1;
                  }
                  b's' as i8 => {
                     smallMode = True;
                     j += 1;
                  }
                  b'q' as i8 => {
                     noisy = False;
                     j += 1;
                  }
                  b'1' as i8 => {
                     blockSize100k = 1;
                     j += 1;
                  }
                  b'2' as i8 => {
                     blockSize100k = 2;
                     j += 1;
                  }
                  b'3' as i8 => {
                     blockSize100k = 3;
                     j += 1;
                  }
                  b'4' as i8 => {
                     blockSize100k = 4;
                     j += 1;
                  }
                  b'5' as i8 => {
                     blockSize100k = 5;
                     j += 1;
                  }
                  b'6' as i8 => {
                     blockSize100k = 6;
                     j += 1;
                  }
                  b'7' as i8 => {
                     blockSize100k = 7;
                     j += 1;
                  }
                  b'8' as i8 => {
                     blockSize100k = 8;
                     j += 1;
                  }
                  b'9' as i8 => {
                     blockSize100k = 9;
                     j += 1;
                  }
                  b'V' as i8 | b'L' as i8 => {
                     license();
                     exit(0); // The program terminates here, so no `j += 1`
                  }
                  b'v' as i8 => {
                     verbosity += 1;
                     j += 1;
                  }
                  b'h' as i8 => {
                     usage(progName);
                     exit(0); // The program terminates here, so no `j += 1`
                  }
                  _ => {
                     eprintln!("{}: Bad flag `{}`", c_str_to_str(progName), c_str_to_str((*aa).name));
                     usage(progName);
                     exit(1); // The program terminates here, so no `j += 1`
                  }
               }              
            }
            aa = (*aa).link;
         }
      }
  }
  
  unsafe {
      aa = arg_list;
      while !aa.is_null() {
          if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
              break;
          }
          if is_flag(aa, b"--stdout\0".as_ptr() as *const c_char) == True {
              srcMode = SM_F2O;
          } else if is_flag(aa, b"--decompress\0".as_ptr() as *const c_char) == True {
              opMode = OM_UNZ;
          } else if is_flag(aa, b"--compress\0".as_ptr() as *const c_char) == True {
              opMode = OM_Z;
          } else if is_flag(aa, b"--force\0".as_ptr() as *const c_char) == True {
              forceOverwrite = True;
          } else if is_flag(aa, b"--test\0".as_ptr() as *const c_char) == True {
              opMode = OM_TEST;
          } else if is_flag(aa, b"--keep\0".as_ptr() as *const c_char) == True {
              keepInputFiles = True;
          } else if is_flag(aa, b"--small\0".as_ptr() as *const c_char) == True {
              smallMode = True;
          } else if is_flag(aa, b"--quiet\0".as_ptr() as *const c_char) == True {
              noisy = False;
          } else if is_flag(aa, b"--version\0".as_ptr() as *const c_char) == True || is_flag(aa, b"--license\0".as_ptr() as *const c_char) == True {
              license();
              exit(0);
          } else if is_flag(aa, b"--verbose\0".as_ptr() as *const c_char) == True {
              verbosity += 1;
          } else if is_flag(aa, b"--help\0".as_ptr() as *const c_char) == True {
              usage(progName);
              exit(0);
          } else {
              eprintln!("{}: Bad flag `{}`", c_str_to_str(progName), c_str_to_str((*aa).name));
              usage(progName);
              exit(1);
          }
          aa = (*aa).link;
      }
  }
  
  if verbosity > 4 {
      verbosity = 4;
  }
  if opMode == OM_Z && smallMode == True && blockSize100k > 2 {
      blockSize100k = 2;
  }
  if opMode == OM_TEST && srcMode == SM_F2O {
      eprintln!("{}: -c and -t cannot be used together.", c_str_to_str(progName));
      exit(1);
  }
  if srcMode == SM_F2O && numFileNames == 0 {
      srcMode = SM_I2O;
  }
  if opMode != OM_Z {
      blockSize100k = 0;
  }
  if srcMode == SM_F2F {
      unsafe {
          signal(SIGINT, my_signal_catcher as usize);
          signal(SIGTERM, my_signal_catcher as usize);
          signal(SIGHUP, my_signal_catcher as usize);
      }
  }
  
  unsafe {
      if opMode == OM_Z {
          if srcMode == SM_I2O {
              compress(ptr::null_mut());
          } else {
              let mut decode = True;
              aa = arg_list;
              while !aa.is_null() {
                  if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
                      decode = False;
                      continue;
                  }
                  if !(*aa).name.is_null() && *(*aa).name == b'-' as c_char && decode == True {
                      aa = (*aa).link;
                      continue;
                  }
                  numFilesProcessed += 1;
                  compress((*aa).name);
                  aa = (*aa).link;
              }
          }
      } else if opMode == OM_UNZ {
          unzFailsExist = False;
          if srcMode == SM_I2O {
              uncompress(ptr::null_mut());
          } else {
              let mut decode = True;
              aa = arg_list;
              while !aa.is_null() {
                  if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
                      decode = False;
                      continue;
                  }
                  if !(*aa).name.is_null() && *(*aa).name == b'-' as c_char && decode == True {
                      aa = (*aa).link;
                      continue;
                  }
                  numFilesProcessed += 1;
                  uncompress((*aa).name);
                  aa = (*aa).link;
              }
          }
          if unzFailsExist == True {
              set_exit(2);
              exit(exitValue);
          }
      } else {
          testFailsExist = False;
          if srcMode == SM_I2O {
              testf(ptr::null_mut());
          } else {
              let mut decode = True;
              aa = arg_list;
              while !aa.is_null() {
                  if is_flag(aa, b"--\0".as_ptr() as *const c_char) == True {
                      decode = False;
                      continue;
                  }
                  if !(*aa).name.is_null() && *(*aa).name == b'-' as c_char && decode == True {
                      aa = (*aa).link;
                      continue;
                  }
                  numFilesProcessed += 1;
                  testf((*aa).name);
                  aa = (*aa).link;
              }
          }
          if testFailsExist == True {
              if noisy == True {
                  eprintln!(
                      "\nYou can use the `bzip2recover' program to attempt to recover\n\
                      data from undamaged sections of corrupted files.\n"
                  );
              }
              set_exit(2);
              exit(exitValue);
          }
      }
  
      aa = arg_list;
      while !aa.is_null() {
          let aa2 = (*aa).link;
          if !(*aa).name.is_null() {
              libc::free((*aa).name as *mut c_void);
          }
          libc::free(aa as *mut c_void);
          aa = aa2;
      }
  }
  std::process::exit(exitValue);
}