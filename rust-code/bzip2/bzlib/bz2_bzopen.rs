

use std::fs::File;
use std::io::{self, Read, Write};
use std::ffi::CStr;
use std::os::unix::io::FromRawFd;
use crate::bzlib::BZ2_bzWriteOpen::bz2_bz_write_open;
use crate::bzlib::BZ2_bzReadOpen::bz2_bz_read_open;


pub fn bzopen_or_bzdopen(
    path: Option<&CStr>,
    fd: Option<i32>,
    mode: &CStr,
    open_mode: i32,
) -> Option<Box<BzFile>> {
    let mut bzerr = 0;
    let mut unused = [0u8; 5000];
    let mut block_size_100k = 9;
    let mut writing = false;
    let mut mode_str = String::new();
    let mut verbosity = 0;
    let mut work_factor = 30;
    let mut small_mode = 0;
    let mut n_unused = 0;

    // Handle mode string
    if mode.to_bytes().is_empty() {
        return None;
    }

    for &byte in mode.to_bytes() {
        match byte as char {
            'r' => writing = false,
            'w' => writing = true,
            's' => small_mode = 1,
            c if c.is_digit(10) => block_size_100k = c as i32 - '0' as i32,
            _ => {}
        }
    }

    mode_str.push_str(if writing { "w" } else { "r" });
    mode_str.push('b');

    // Open file based on open_mode
    let file = if open_mode == 0 {
        if path.is_none() || path.unwrap().to_bytes().is_empty() {
            if writing {
                Box::new(io::stdout()) as Box<dyn Write>
            } else {
                Box::new(io::stdin()) as Box<dyn Read>
            }
        } else {
            match File::open(path.unwrap().to_str().unwrap()) {
                Ok(f) => Box::new(f) as Box<dyn Read>,
                Err(_) => return None,
            }
        }
    } else {
        if let Some(fd) = fd {
            unsafe {
                let f = File::from_raw_fd(fd);
                Box::new(f) as Box<dyn Read>
            }
        } else {
            return None;
        }
    };

    if writing {
        // Ensure block size is within the allowed range
        block_size_100k = block_size_100k.clamp(1, 9);
        let bzfp = bz2_bz_write_open(
            &mut bzerr,
            file,
            block_size_100k,
            verbosity,
            work_factor,
        );

        if bzfp.is_none() && file != Box::new(io::stdout()) as Box<dyn Write> {
            drop(file);
        }

        bzfp
    } else {
        let bzfp = bz2_bz_read_open(
            &mut bzerr,
            file,
            verbosity,
            small_mode,
            &mut unused,
            n_unused,
        );

        if bzfp.is_none() && file != Box::new(io::stdin()) as Box<dyn Read> {
            drop(file);
        }

        bzfp
    }
}


pub fn bz2_bzopen(path: Option<&str>, mode: &str) -> Option<Box<BzFile>> {
    // Convert path and mode to CStr equivalents if they are present
    let c_path = path.map(|p| std::ffi::CString::new(p).ok()).flatten();
    let c_mode = std::ffi::CString::new(mode).ok()?;

    // Call bzopen_or_bzdopen with path, fd set to -1 (meaning no file descriptor), mode, and open_mode set to 0
    bzopen_or_bzdopen(c_path.as_ref(), None, &c_mode, 0)
}

pub fn bz2_bzdopen(fd: i32, mode: &str) -> Option<Box<BzFile>> {
    // Convert the mode to a C-compatible string (CString)
    let c_mode = std::ffi::CString::new(mode).ok()?;

    // Call bzopen_or_bzdopen with None for the path, fd as the file descriptor, mode, and open_mode set to 1
    bzopen_or_bzdopen(None, Some(fd), &c_mode, 1)
}
