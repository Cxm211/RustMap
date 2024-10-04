
use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzlib::BZ2_bzCompress::bz2_bz_compress;
use crate::bzFile;
use libc::{ferror, fwrite, c_void};

pub fn bz2_bz_write(bzerror: Option<&mut i32>, b: Option<&mut bzFile>, buf: Option<&[u8]>, len: i32) {
    let mut bzf = match b {
        Some(bz) => bz,
        None => {
            bz_seterr(bzerror, None, -2);
            return;
        }
    };

    bz_seterr(bzerror, Some(bzf), 0);

    if buf.is_none() || len < 0 {
        bz_seterr(bzerror, Some(bzf), -2);
        return;
    }

    if !bzf.writing {
        bz_seterr(bzerror, Some(bzf), -1);
        return;
    }

    unsafe {
        if ferror(bzf.handle) != 0 {
            bz_seterr(bzerror, Some(bzf), -6);
            return;
        }
    }

    if len == 0 {
        bz_seterr(bzerror, Some(bzf), 0);
        return;
    }

    bzf.strm.avail_in = len as u32;
    bzf.strm.next_in = buf.unwrap().as_ptr() as *mut u8; // Dereference and convert to raw pointer

    loop {
        bzf.strm.avail_out = 5000;
        bzf.strm.next_out = bzf.buf.as_mut_ptr() as *mut u8; // Convert to raw pointer
        
        let ret = bz2_bz_compress(&mut bzf.strm, 0);
        if ret != 1 {
            bz_seterr(bzerror, Some(bzf), ret);
            return;
        }

        if bzf.strm.avail_out < 5000 {
            let n = (5000 - bzf.strm.avail_out) as usize;
            unsafe {
                let n2 = fwrite(
                    bzf.buf.as_ptr() as *const c_void,
                    1,
                    n,
                    bzf.handle,
                );

                if n != n2 || ferror(bzf.handle) != 0 {
                    bz_seterr(bzerror, Some(bzf), -6);
                    return;
                }
            }
        }

        if bzf.strm.avail_in == 0 {
            bz_seterr(bzerror, Some(bzf), 0);
            return;
        }
    }
}

pub fn bz2_bzwrite(b: &mut bzFile, buf: &[u8], len: i32) -> i32 {
    let mut bzerr: i32 = 0;

    bz2_bz_write(Some(&mut bzerr), Some(b), Some(buf), len);

    if bzerr == 0 {
        len
    } else {
        -1
    }
}