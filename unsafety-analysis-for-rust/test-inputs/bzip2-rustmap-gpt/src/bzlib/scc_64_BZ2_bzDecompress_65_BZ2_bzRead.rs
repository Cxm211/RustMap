// use crate::bzlib::scc_11_BZ2_bzReadClose::*;
use crate::bzlib::scc_7_unRLE_obuf_to_output_SMALL::*;
use crate::bzlib::scc_8_root_6_unRLE_obuf_to_output_FAST::*;
use crate::global_vars::bzip2_c1::*;

// use crate::global_vars::bzip2_c1::{verbosity, DState, bz_stream};
use std::io::{Error, Read, Write};
use std::ptr;
use std::slice;

// 假设 unRLE_obuf_to_output_SMALL 和 unRLE_obuf_to_output_FAST 函数
// 已经被正确地翻译和实现为 Rust 函数。
pub fn BZ2_bzDecompress(strm: *mut bz_stream_DState) -> i32 {
    // unsafe {
    //     if strm.is_null() { return -2; }
    //     let mut corrupt: u8 = false;
    //     let s = (*strm).state;
    //     if s.is_null() { return -2; }
    //     let s = &mut *s;

    //     if (*s.strm) != strm { return -2; }

    //     loop {
    //         match s.state {
    //             1 => return -1,
    //             2 => {
    //                 let corrupt = if s.smallDecompress {
    //                     un_rle_obuf_to_output_small(s)
    //                 } else {
    //                     unRLE_obuf_to_output_FAST(s)
    //                 };

    //                 if corrupt { return -4; }

    //                 if s.nblock_used == s.save_nblock + 1 && s.state_out_len == 0 {
    //                     s.calculatedBlockCRC = !s.calculatedBlockCRC;
    //                     if s.verbosity >= 3 {
    //                         eprintln!("{{0x{:08x}, 0x{:08x}}}", s.storedBlockCRC, s.calculatedBlockCRC);
    //                     }
    //                     if s.verbosity >= 2 { eprintln!("]"); }
    //                     if s.calculatedBlockCRC != s.storedBlockCRC { return -4; }
    //                     s.calculatedCombinedCRC = s.calculatedCombinedCRC.rotate_left(1) ^ s.calculatedBlockCRC;
    //                     s.state = 14;
    //                     break;
    //                 } else {
    //                     return 0;
    //                 }
    //             },
    //             _ if s.state >= 10 => {
    //                 let r = BZ2_bzDecompress(s);
    //                 if r == 4 {
    //                     if s.verbosity >= 3 {
    //                         eprintln!("\n    combined CRCs: stored = 0x{:08x}, computed = 0x{:08x}", s.storedCombinedCRC, s.calculatedCombinedCRC);
    //                     }
    //                     if s.calculatedCombinedCRC != s.storedCombinedCRC { return -4; }
    //                     return r;
    //                 }
    //                 if s.state != 2 { return r; }
    //             },
    //             _ => {}
    //         }
    //     }
    // }
    0 // If the loop exits normally, return 0.
}


pub fn BZ2_bzRead(bzerror: &mut i32, b: BZFILE, buf: &mut [u8]) -> i32 {
    // if b.is_null() || buf.is_empty() {
    //     *bzerror = -2;
    //     return 0;
    // }

    // let bzf = unsafe { &mut *b };

    // if bzf.writing {
    //     *bzerror = -1;
    //     return 0;
    // }

    // let mut total_read = 0;
    // bzf.strm.avail_out = buf.len() as u32;
    // bzf.strm.next_out = buf.as_mut_ptr() as *mut i8;

    // while bzf.strm.avail_out > 0 {
    //     if bzf.strm.avail_in == 0 {
    //         match read_input_to_buffer(&mut bzf.handle, &mut bzf.buf) {
    //             Ok(n) => {
    //                 bzf.bufN = n as i32;
    //                 bzf.strm.avail_in = n as u32;
    //                 bzf.strm.next_in = bzf.buf.as_mut_ptr() as *mut i8;
    //             },
    //             Err(_) => {
    //                 *bzerror = -6;
    //                 return 0;
    //             }
    //         }
    //     }

    //     let ret = unsafe { BZ2_bzDecompress(&mut bzf.strm as *mut bz_stream) };
    //     if ret != 0 && ret != 4 {
    //         *bzerror = ret;
    //         return 0;
    //     }

    //     if ret == 4 || (bzf.strm.avail_in == 0 && bzf.strm.avail_out > 0) {
    //         *bzerror = if ret == 4 { 4 } else { -7 };
    //         break;
    //     }

    //     total_read += (buf.len() as u32 - bzf.strm.avail_out) as i32;
    // }

    // total_read
    0
}

