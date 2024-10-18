
use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzlib::BZ2_bzCompressInit::bz2_bz_compress_init;
use libc::{ferror, malloc, free, c_void};
use crate::bzFile;
use libc::FILE;
use std::ptr;

pub fn bz2_bz_write_open(
    bzerror: Option<&mut i32>,
    f: *mut FILE,
    block_size_100k: i32,
    verbosity: i32,
    work_factor: i32,
) -> Option<*mut bzFile> {
    let mut ret: i32;
    let mut bzf: *mut bzFile = std::ptr::null_mut();

    bz_seterr(bzerror, bzf, 0);

    if f.is_null()
        || (block_size_100k < 1 || block_size_100k > 9)
        || (work_factor < 0 || work_factor > 250)
        || (verbosity < 0 || verbosity > 4)
    {
        bz_seterr(bzerror, bzf, -2);
        return None;
    }

    if unsafe { ferror(f) } != 0 {
        bz_seterr(bzerror, bzf, -6);
        return None;
    }

    bzf = unsafe { malloc(std::mem::size_of::<bzFile>()) as *mut bzFile };
    if bzf.is_null() {
        bz_seterr(bzerror, bzf, -3);
        return None;
    }

    bz_seterr(bzerror, bzf, 0);
    unsafe {
        (*bzf).initialisedOk = false;
        (*bzf).bufN = 0;
        (*bzf).handle = f;
        (*bzf).writing = true;
        (*bzf).strm.bzalloc = None;
        (*bzf).strm.bzfree = None;
        (*bzf).strm.opaque = ptr::null_mut(); 
    }

    let adjusted_work_factor = if work_factor == 0 { 30 } else { work_factor };
    ret = unsafe {
        bz2_bz_compress_init(
            &mut (*bzf).strm,
            block_size_100k,
            verbosity,
            adjusted_work_factor,
        )
    };
    if ret != 0 {
        bz_seterr(bzerror, bzf, ret);
        unsafe { free(bzf as *mut c_void) };
        return None;
    }

    unsafe {
        (*bzf).strm.avail_in = 0;
        (*bzf).initialisedOk = true;
    }

    Some(bzf)
}
