

use crate::bzlib::utils::bz_config_ok;
use crate::bzlib::utils::default_bzalloc;
use crate::bzlib::utils::default_bzfree;
use crate::bzlib::utils::prepare_new_block;

pub fn bz2_bz_compress_init(
    strm: &mut bz_stream,
    block_size_100k: i32,
    verbosity: i32,
    mut work_factor: i32,
) -> i32 {
    let n: i32;

    // Check configuration validity
    if !bz_config_ok() {
        return -9;
    }

    // Validate input parameters
    if strm.is_null() || block_size_100k < 1 || block_size_100k > 9 || work_factor < 0 || work_factor > 250 {
        return -2;
    }

    // Default workFactor to 30 if not set
    if work_factor == 0 {
        work_factor = 30;
    }

    // Set default memory allocation and deallocation functions if not provided
    if strm.bzalloc.is_none() {
        strm.bzalloc = Some(default_bzalloc);
    }
    if strm.bzfree.is_none() {
        strm.bzfree = Some(default_bzfree);
    }

    // Allocate memory for EState struct
    let s_ptr = (strm.bzalloc.unwrap())(strm.opaque, std::mem::size_of::<EState>() as i32, 1);
    if s_ptr.is_null() {
        return -3;
    }
    let s: &mut EState = unsafe { &mut *(s_ptr as *mut EState) };
    s.strm = Some(strm);

    // Initialize pointers to null
    s.arr1 = None;
    s.arr2 = None;
    s.ftab = None;

    // Allocate memory for arrays
    n = 100000 * block_size_100k;
    s.arr1 = (strm.bzalloc.unwrap())(strm.opaque, n * std::mem::size_of::<u32>() as i32, 1);
    s.arr2 = (strm.bzalloc.unwrap())(strm.opaque, (n + 34) * std::mem::size_of::<u32>() as i32, 1);
    s.ftab = (strm.bzalloc.unwrap())(strm.opaque, 65537 * std::mem::size_of::<u32>() as i32, 1);

    // Check if allocations failed
    if s.arr1.is_none() || s.arr2.is_none() || s.ftab.is_none() {
        if let Some(arr1) = s.arr1 {
            (strm.bzfree.unwrap())(strm.opaque, arr1);
        }
        if let Some(arr2) = s.arr2 {
            (strm.bzfree.unwrap())(strm.opaque, arr2);
        }
        if let Some(ftab) = s.ftab {
            (strm.bzfree.unwrap())(strm.opaque, ftab);
        }
        (strm.bzfree.unwrap())(strm.opaque, s_ptr);
        return -3;
    }

    // Initialize EState fields
    s.block_no = 0;
    s.state = 2;
    s.mode = 2;
    s.combined_crc = 0;
    s.block_size_100k = block_size_100k;
    s.nblock_max = 100000 * block_size_100k - 19;
    s.verbosity = verbosity;
    s.work_factor = work_factor;

    // Set block and mtfv pointers
    s.block = s.arr2.as_ref().unwrap().as_mut_ptr() as *mut u8;
    s.mtfv = s.arr1.as_ref().unwrap().as_mut_ptr() as *mut u16;
    s.zbits = None;
    s.ptr = s.arr1.as_ref().unwrap().as_mut_ptr() as *mut u32;

    // Initialize stream state
    strm.state = Some(s);
    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;

    // Initialize run-length encoding and prepare new block
    init_rl(s);
    prepare_new_block(s);

    0 
    // Return success
}
