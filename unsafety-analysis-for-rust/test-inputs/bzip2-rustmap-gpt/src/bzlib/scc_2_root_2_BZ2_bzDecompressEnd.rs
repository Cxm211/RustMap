// use crate::global_vars::bzip2_c1::{DState, bz_stream};


// BZ2_bzDecompressEnd (
//       bz_stream *strm
//    )

// BZ2_bzDecompressEnd ( bz_stream *strm )

type Int32 = i32;
type UInt32 = u32;
type UInt16 = u16;
type UChar = u8;
type Bool = bool;

use std::mem::zeroed;

pub fn setup_mismatched_bz_stream() -> *mut bz_stream {
    let strm: *mut bz_stream = Box::into_raw(Box::new(bz_stream {
        state: Box::into_raw(Box::new(DState {
            strm: std::ptr::null_mut(), // intentionally setting to null for mismatch
            ..unsafe { zeroed() }
        })),
        ..unsafe { zeroed() }
    }));

    unsafe {
        (*(*strm).state).strm = strm;
    }

    strm
}

pub fn setup_valid_bz_stream() -> *mut bz_stream {
    let strm: *mut bz_stream = Box::into_raw(Box::new(bz_stream {
        state: Box::into_raw(Box::new(DState {
            strm: std::ptr::null_mut(), // will set properly after bz_stream is initialized
            ..unsafe { zeroed() }
        })),
        ..unsafe { zeroed() }
    }));

    unsafe {
        (*(*strm).state).strm = strm;
    }

    strm
}

pub struct DState {
  pub  strm: *mut bz_stream,   // Reference to associated bz_stream

  pub  state: Int32,          // The current state

  pub  state_out_ch: UChar,   // Output character state
  pub  state_out_len: Int32,  // Length of output state
  pub  blockRandomised: Bool, // Whether the block is randomised
  pub  rNToGo: Int32,         // RLE repeat count
  pub  rTPos: Int32,          // RLE value last output

  pub  bsBuff: UInt32,        // Bitstream buffer
   pub bsLive: Int32,         // Number of bits left in bsBuff

 pub   blockSize100k: Int32,  // Block size, units of 100k
  pub  smallDecompress: Bool, // Use the slower, smaller decompression routine
  pub  currBlockNo: Int32,    // The current block number
  pub  verbosity: Int32,      // Verbosity level

 pub   origPtr: Int32,        // Pointer to original data
 pub   tPos: UInt32,          // Current position in buffer
 pub   k0: Int32,             // Position in sequence for MTF decoding
pub    unzftab: [Int32; 256], // Frequency table of byte values (used for MTF)
 pub   nblock_used: Int32,    // Number of blocks used
 pub   cftab: [Int32; 257],   // Cumulative frequency table
 pub   cftabCopy: [Int32; 257], // A copy of the cftab array

 pub   tt: *mut UInt32,       // Pointer to another array of unknown purpose
 pub   ll16: *mut UInt16,     // Pointer to another array of unknown purpose
 pub   ll4: *mut UChar,       // Pointer to another array of unknown purpose

  pub  storedBlockCRC: UInt32,  // CRC value of stored block
  pub  storedCombinedCRC: UInt32, // Combined CRC value
  pub  calculatedBlockCRC: UInt32, // Calculated CRC value of block
  pub  calculatedCombinedCRC: UInt32, // Calculated combined CRC value

  pub  nInUse: Int32,           // Number of bytes in use
  pub  inUse: [Bool; 256],      // Flags indicating which values are in use
  pub  inUse16: [Bool; 16],     // Flags for 16-bit values
  pub  seqToUnseq: [UChar; 256], // MTF values

 pub   mtfa: [UChar; 4096],      // Another array for MTF values
  pub  mtfbase: [Int32; 256 / 16], // Base values for MTF encoding

   pub selector: [UChar; 2 + (900000 / 50)], // Selector values
  pub  selectorMtf: [UChar; 2 + (900000 / 50)], // MTF'd selector values

  pub  len: [[UChar; 258]; 6],  // Lengths of Huffman code tables

  pub  limit: [[Int32; 258]; 6], // Huffman coding table limit values
  pub  base: [[Int32; 258]; 6],  // Huffman coding table base values
  pub  perm: [[Int32; 258]; 6],  // Huffman coding table permutation values
  pub  minLens: [Int32; 6],      // Minimum lengths of Huffman coding tables

    // State-saving variables (used for saving and restoring state)
 pub   save_i: Int32,
 pub   save_j: Int32,
 pub   save_t: Int32,
 pub   save_alphaSize: Int32,
 pub   save_nGroups: Int32,
 pub   save_nSelectors: Int32,
 pub   save_EOB: Int32,
 pub   save_groupNo: Int32,
 pub   save_groupPos: Int32,
 pub   save_nextSym: Int32,
 pub   save_nblockMAX: Int32,
 pub   save_nblock: Int32,
 pub   save_es: Int32,
 pub   save_N: Int32,
 pub   save_curr: Int32,
 pub   save_zt: Int32,
 pub   save_zn: Int32,
 pub   save_zvec: Int32,
 pub   save_zj: Int32,
  pub  save_gSel: Int32,
pub    save_gMinlen: Int32,
 pub   save_gLimit: *mut Int32,
pub    save_gBase: *mut Int32,
 pub   save_gPerm: *mut Int32,
}
pub struct bz_stream {
 pub   next_in: *mut i8, // char in C is i8 in Rust.
pub   avail_in: u32,
 pub   total_in_lo32: u32,
  pub  total_in_hi32: u32,

 pub   next_out: *mut i8,
  pub  avail_out: u32,
  pub  total_out_lo32: u32,
  pub  total_out_hi32: u32,

 pub   state: *mut DState, // I'm assuming DState here based on your earlier code.
    
 pub   bzalloc: Option<fn(*mut std::ffi::c_void, Int32, Int32) -> *mut std::ffi::c_void>,
 pub   bzfree: Option<fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
 pub   opaque: *mut std::ffi::c_void,
}


pub fn bz2_bz_decompress_end(strm: *mut bz_stream) -> Int32 {
    if strm.is_null() {
        return -2;
    }

    let strm_ref = unsafe { &mut *strm }; // Dereference the pointer to get a mutable reference

    if strm_ref.state.is_null() {
        return -2;
    }

    let s = unsafe { &mut *strm_ref.state }; // Dereference the state pointer

    if !s.tt.is_null() {
        if let Some(bzfree) = strm_ref.bzfree {
            (bzfree)(strm_ref.opaque, s.tt as *mut _);
        }
    }
    if !s.ll16.is_null() {
        if let Some(bzfree) = strm_ref.bzfree {
            (bzfree)(strm_ref.opaque, s.ll16 as *mut _);
        }
    }
    if !s.ll4.is_null() {
        if let Some(bzfree) = strm_ref.bzfree {
            (bzfree)(strm_ref.opaque, s.ll4 as *mut _);
        }
    }

    if let Some(bzfree) = strm_ref.bzfree {
        (bzfree)(strm_ref.opaque, strm_ref.state as *mut _);
    }

    strm_ref.state = std::ptr::null_mut();
    return 0;
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor; // 用于在内存中模拟文件I/O
    #[test]
    fn test_normal_decompress_end() {
        // Setup
        let strm = setup_valid_bz_stream();
        // Call the function
        let result = bz2_bz_decompress_end(strm);
        // Verify the result
        assert_eq!(result, 0);
        // Check if resources are freed
        assert!(unsafe { (*strm).state.is_null() });
    }

    #[test]
    fn test_null_bz_stream() {
        // Setup
        let strm: *mut bz_stream = std::ptr::null_mut();
        // Call the function
        let result = bz2_bz_decompress_end(strm);
        // Verify the result
        assert_eq!(result, -2);
    }
}