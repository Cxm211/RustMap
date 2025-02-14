
use crate::bzlib::utils::bz_config_ok;
use crate::bzlib::utils::default_bzalloc;
use crate::bzlib::utils::default_bzfree;


pub fn bz2_bz_decompress_init(strm: &mut bz_stream, verbosity: i32, small: i32) -> i32 {
    // Check if the configuration is okay
    if !bz_config_ok() {
        return -9;
    }

    // Validate parameters
    if strm.is_none() {
        return -2;
    }
    if small != 0 && small != 1 {
        return -2;
    }
    if verbosity < 0 || verbosity > 4 {
        return -2;
    }

    // Set default memory allocation functions if not provided
    if strm.bzalloc.is_none() {
        strm.bzalloc = Some(default_bzalloc);
    }
    if strm.bzfree.is_none() {
        strm.bzfree = Some(default_bzfree);
    }

    // Allocate memory for the decompression state
    let s = match strm.bzalloc.unwrap()(strm.opaque, std::mem::size_of::<DState>(), 1) {
        Some(state) => state,
        None => return -3,
    };

    // Initialize the state
    s.strm = Some(strm);
    strm.state = Some(s);
    s.state = 10;
    s.bs_live = 0;
    s.bs_buff = 0;
    s.calculated_combined_crc = 0;
    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;
    s.small_decompress = small != 0;
    s.ll4 = None;
    s.ll16 = None;
    s.tt = None;
    s.curr_block_no = 0;
    s.verbosity = verbosity;

    0
}
