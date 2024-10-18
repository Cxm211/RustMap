

use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzlib::BZ2_bzDecompress::bz2_bz_decompress;
use libc;
use crate::bzFile;
use libc::{fgetc, ungetc, EOF, FILE};

pub fn myfeof(f: *mut FILE) -> bool {
    if f.is_null() {
        return true; // If the file pointer is null, we consider it as EOF or error.
    }

    unsafe {
        let c = fgetc(f);
        if c == EOF {
            return true; // Reached EOF.
        }
        ungetc(c, f); // Put the character back if not EOF.
    }

    false // Not EOF.
}

pub fn bz2_bz_read(
    bzerror: Option<&mut i32>,
    b: Option<&mut bzFile>,
    buf: *mut u8,
    len: i32,
) -> i32 {
    let mut bzf = b;

    bz_seterr(bzerror, bzf, 0);

    if bzf.is_none() || buf.is_null() || len < 0 {
        bz_seterr(bzerror, bzf, -2);
        return 0;
    }

    let bzf = bzf.unwrap();

    if bzf.writing {
        bz_seterr(bzerror, Some(bzf), -1);
        return 0;
    }

    if len == 0 {
        bz_seterr(bzerror, Some(bzf), 0);
        return 0;
    }

    unsafe {
        bzf.strm.avail_out = len as u32;
        bzf.strm.next_out = buf;

        loop {
            if libc::ferror(bzf.handle) != 0 {
                bz_seterr(bzerror, Some(bzf), -6);
                return 0;
            }

            if bzf.strm.avail_in == 0 && !myfeof(bzf.handle) {
                let n = libc::fread(
                    bzf.buf.as_mut_ptr() as *mut libc::c_void,
                    std::mem::size_of::<u8>(),
                    5000,
                    bzf.handle,
                );
                if libc::ferror(bzf.handle) != 0 {
                    bz_seterr(bzerror, Some(bzf), -6);
                    return 0;
                }
                bzf.bufN = n as i32;
                bzf.strm.avail_in = bzf.bufN as u32;
                bzf.strm.next_in = bzf.buf.as_mut_ptr() as *mut u8;
            }

            let ret = bz2_bz_decompress(&mut bzf.strm);

            if ret != 0 && ret != 4 {
                bz_seterr(bzerror, Some(bzf), ret);
                return 0;
            }

            if ret == 0
                && myfeof(bzf.handle)
                && bzf.strm.avail_in == 0
                && bzf.strm.avail_out > 0
            {
                bz_seterr(bzerror, Some(bzf), -7);
                return 0;
            }

            if ret == 4 {
                bz_seterr(bzerror, Some(bzf), 4);
                return len - bzf.strm.avail_out as i32;
            }

            if bzf.strm.avail_out == 0 {
                bz_seterr(bzerror, Some(bzf), 0);
                return len;
            }
        }
    }
}

pub fn bz2_bzread(b: &mut bzFile, buf: &mut [u8], len: i32) -> i32 {
    if b.lastErr == 4 {
        return 0; // Equivalent to returning early if the last error indicates EOF.
    }

    let mut bzerr: i32 = 0;
    let nread = bz2_bz_read(Some(&mut bzerr), Some(b), buf.as_mut_ptr(), len);

    if bzerr == 0 || bzerr == 4 {
        nread
    } else {
        -1 // Return -1 in case of an error other than EOF or success.
    }
}