use std::ptr;
use crate::bzlib::scc_36_root_27_default_bzfree_38_bz_config_ok_37_default_bzalloc::*;
use crate::global_vars::bzip2_c1::{BZ2_rNums, DState, bz_stream_DState,  bzFile};
pub fn bz2_bzread_open(bzerror: &mut i32, f: &mut std::fs::File, verbosity: i32, small: i32, unused: Option<&[u8]>) -> Option<Box<bzFile>> {
    let mut bzf = Box::new(bzFile::default());

    // Initialize error code to 0.
    *bzerror = 0;
    bzf.lastErr = 0;

    // Validate arguments.
    if small != 0 && small != 1
        || verbosity < 0 || verbosity > 4
        || unused.is_some() && (unused.unwrap().len() > 5000 || unused.unwrap().is_empty())
    {
        *bzerror = -2;
        bzf.lastErr = -2;
        return None;
    }

    // Check for file errors.
    // NOTE: This is a simple error check. Rust's `File` type provides better error handling.
    if f.metadata().is_err() {
        *bzerror = -6;
        bzf.lastErr = -6;
        return None;
    }

    // Initialize the bzFile structure.
    bzf.initialisedOk = false;
    bzf.handle = f;
    bzf.bufN = 0;
    bzf.writing = false;
    bzf.strm.bzalloc = None;
    bzf.strm.bzfree = None;
    bzf.strm.opaque = std::ptr::null_mut();

    // Copy unused data to the buffer.
    if let Some(data) = unused {
        for &byte in data.iter() {
            bzf.buf[bzf.bufN as usize] = byte;
            bzf.bufN += 1;
        }
    }

    // Initialize the decompression stream.
    let ret = bz2_bzdecompress_init(&mut bzf.strmD, verbosity, small);
    if ret != 0 {
        *bzerror = ret;
        bzf.lastErr = ret;
        return None;
    }

    bzf.strm.avail_in = bzf.bufN as _;  // TODO: Adjust based on the type of `avail_in`.
    bzf.strm.next_in = bzf.buf.as_mut_ptr() as *mut i8;


    bzf.initialisedOk = true;
    Some(bzf)
}

pub fn bz2_bzdecompress_init(strm: &mut bz_stream_DState, verbosity: i32, small: i32) -> i32 {
    // Check if bzip2 configuration is correct.
    if !bz_config_ok() {
        return -9;
    }

    // Validate arguments.
    if small != 0 && small != 1 {
        return -2;
    }
    if verbosity < 0 || verbosity > 4 {
        return -2;
    }

    // Use default allocation and free functions if not provided.
    if strm.bzalloc.is_none() {
        strm.bzalloc = Some(default_bzalloc as fn(*mut std::ffi::c_void, i32, i32) -> *mut std::ffi::c_void);
     }     
    if strm.bzfree.is_none() {
        strm.bzfree = Some(default_bzfree);
    }

    // Allocate memory for DState structure.
    let s = strm.bzalloc.expect("bzalloc function not set")(strm.opaque, std::mem::size_of::<DState>() as i32, 1);
    if s.is_null() {
        return -3;
    }
    let s_ref = unsafe { &mut *(s as *mut DState) };  // Ensure type safety with DState.

    // Initialize DState and strm structures.
    s_ref.strm = strm;
    strm.state = s_ref;
    s_ref.state = 10;
    s_ref.bsLive = 0;
    s_ref.bsBuff = 0;
    s_ref.calculatedCombinedCRC = 0;
    strm.total_in_lo32 = 0;
    strm.total_in_hi32 = 0;
    strm.total_out_lo32 = 0;
    strm.total_out_hi32 = 0;
    s_ref.smallDecompress = small != 0;
    s_ref.ll4 = std::ptr::null_mut();
    s_ref.ll16 = std::ptr::null_mut();
    s_ref.tt = std::ptr::null_mut();    
    s_ref.currBlockNo = 0;
    s_ref.verbosity = verbosity;

    0
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::path::Path;

    #[test]
    fn test_bz2_bzread_open() {
        // 1. Prepare a sample bz2 compressed file
        let test_file_path = Path::new("test_data.bz2");
        let test_data = b"Hello, world!"; // This should be actual bz2 compressed data
        let mut file = File::create(&test_file_path).expect("Failed to create test file");
        file.write_all(test_data).expect("Failed to write to test file");
        
        // 2. Try to open the bz2 file
        let mut f = File::open(&test_file_path).expect("Failed to open test file for reading");
        let mut bzerror: i32 = 0;
        let result = bz2_bzread_open(&mut bzerror, &mut f, 0, 0, None);

        // 3. Check the result
        assert!(result.is_some(), "Failed to open bz2 file with bz2_bzread_open");
        let bzf = result.unwrap();
        assert_eq!(bzf.lastErr, 0, "Expected lastErr to be 0, got {}", bzf.lastErr);

        // 4. Cleanup
        std::fs::remove_file(&test_file_path).expect("Failed to delete test file");
    }
}
