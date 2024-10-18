mod bzlib::BZ2_bzCompress;
mod bzlib::BZ2_bzCompressEnd;
mod bzlib::BZ2_bzCompressInit;

use BZ2_bzCompress::bz2_bz_compress;
use BZ2_bzCompressEnd::bz2_bz_compress_end;
use BZ2_bzCompressInit::bz2_bz_compress_init;

pub fn bz2_bz_buff_to_buff_compress(
    dest: &mut [u8],
    dest_len: &mut u32,
    source: &[u8],
    source_len: u32,
    block_size_100k: i32,
    verbosity: i32,
    work_factor: i32,
) -> i32 {
    if dest.is_empty()
        || dest_len.is_null()
        || source.is_empty()
        || block_size_100k < 1
        || block_size_100k > 9
        || verbosity < 0
        || verbosity > 4
        || work_factor < 0
        || work_factor > 250
    {
        return -2;
    }

    let work_factor = if work_factor == 0 { 30 } else { work_factor };

    let mut strm = bz_stream {
        next_in: source.as_ptr() as *mut _,
        avail_in: source_len,
        next_out: dest.as_mut_ptr(),
        avail_out: *dest_len,
        bzalloc: None,
        bzfree: None,
        opaque: std::ptr::null_mut(),
        ..Default::default()
    };

    let ret = bz2_bz_compress_init(&mut strm, block_size_100k, verbosity, work_factor);
    if ret != 0 {
        return ret;
    }

    let ret = bz2_bz_compress(&mut strm, 2);
    if ret == 3 {
        bz2_bz_compress_end(&mut strm);
        return -8; // Output overflow
    }

    if ret != 4 {
        bz2_bz_compress_end(&mut strm);
        return ret;
    }

    *dest_len -= strm.avail_out;

    bz2_bz_compress_end(&mut strm);
    0
}
