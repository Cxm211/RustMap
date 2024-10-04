

use crate::bzlib::BZ2_bzCompress::bz2_bz_compress;
use crate::bzlib::BZ2_bzCompressEnd::bz2_bz_compress_end;
use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzFile;
use libc::{ferror, fflush, fwrite, c_void};

pub fn bz2_bz_write_close64(
    bzerror: Option<&mut i32>,
    b: Option<&mut bzFile>,
    abandon: i32,
    nbytes_in_lo32: Option<&mut u32>,
    nbytes_in_hi32: Option<&mut u32>,
    nbytes_out_lo32: Option<&mut u32>,
    nbytes_out_hi32: Option<&mut u32>,
) {
    let mut bzf = match b {
        Some(bz) => bz,
        None => {
            bz_seterr(bzerror, None, 0);
            return;
        }
    };

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

    if let Some(lo32) = nbytes_in_lo32 {
        *lo32 = 0;
    }
    if let Some(hi32) = nbytes_in_hi32 {
        *hi32 = 0;
    }
    if let Some(lo32) = nbytes_out_lo32 {
        *lo32 = 0;
    }
    if let Some(hi32) = nbytes_out_hi32 {
        *hi32 = 0;
    }

    if abandon == 0 && bzf.lastErr == 0 {
        loop {
            bzf.strm.avail_out = 5000;
            bzf.strm.next_out = bzf.buf.as_mut_ptr() as *mut u8;

            let ret = bz2_bz_compress(&mut bzf.strm, 2);
            if ret != 3 && ret != 4 {
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

            if ret == 4 {
                break;
            }
        }
    }

    unsafe {
        if abandon == 0 && ferror(bzf.handle) == 0 {
            fflush(bzf.handle);
            if ferror(bzf.handle) != 0 {
                bz_seterr(bzerror, Some(bzf), -6);
                return;
            }
        }
    }

    if let Some(lo32) = nbytes_in_lo32 {
        *lo32 = bzf.strm.total_in_lo32;
    }
    if let Some(hi32) = nbytes_in_hi32 {
        *hi32 = bzf.strm.total_in_hi32;
    }
    if let Some(lo32) = nbytes_out_lo32 {
        *lo32 = bzf.strm.total_out_lo32;
    }
    if let Some(hi32) = nbytes_out_hi32 {
        *hi32 = bzf.strm.total_out_hi32;
    }

    bz_seterr(bzerror, Some(bzf), 0);
    bz2_bz_compress_end(&mut bzf.strm);
    drop(bzf);
}

pub fn bz2_bz_write_close(
    bzerror: Option<&mut i32>,
    b: Option<&mut bzFile>,
    abandon: i32,
    nbytes_in: Option<&mut u32>,
    nbytes_out: Option<&mut u32>,
) {
    bz2_bz_write_close64(
        bzerror,
        b,
        abandon,
        nbytes_in,
        None,         // Corresponds to `nbytes_in_hi32`
        nbytes_out,
        None,         // Corresponds to `nbytes_out_hi32`
    );
}
