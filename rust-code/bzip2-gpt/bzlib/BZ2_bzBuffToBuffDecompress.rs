mod bzlib::BZ2_bzDecompress;
mod bzlib::BZ2_bzDecompressEnd;
mod bzlib::BZ2_bzDecompressInit;

use BZ2_bzDecompress::bz2_bz_decompress;
use BZ2_bzDecompressEnd::bz2_bz_decompress_end;
use BZ2_bzDecompressInit::bz2_bz_decompress_init;


pub fn bz2_bz_buff_to_buff_decompress(
    dest: &mut [u8],
    dest_len: &mut u32,
    source: &[u8],
    source_len: u32,
    small: i32,
    verbosity: i32,
) -> i32 {
    // Validate input parameters
    if dest.is_empty()
        || dest_len.is_null()
        || source.is_empty()
        || (small != 0 && small != 1)
        || verbosity < 0
        || verbosity > 4
    {
        return -2;
    }

    // Initialize bz_stream
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

    // Decompression initialization
    let ret = bz2_bz_decompress_init(&mut strm, verbosity, small);
    if ret != 0 {
        return ret;
    }

    // Decompress
    let ret = bz2_bz_decompress(&mut strm);
    if ret == 0 {
        if strm.avail_out > 0 {
            bz2_bz_decompress_end(&mut strm);
            return -7; // Output overflow or EOF
        } else {
            bz2_bz_decompress_end(&mut strm);
            return -8; // Output overflow
        }
    } else if ret != 4 {
        bz2_bz_decompress_end(&mut strm);
        return ret;
    }

    // Update destination length and finalize
    *dest_len -= strm.avail_out;
    bz2_bz_decompress_end(&mut strm);

    0 // Success
}
