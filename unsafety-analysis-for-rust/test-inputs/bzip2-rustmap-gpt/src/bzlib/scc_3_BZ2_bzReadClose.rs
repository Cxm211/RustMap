// use crate::huffman::scc_12_root_6_BZ2_hbCreateDecodeTables::*;
// use crate::global_vars::bzip2_c1::{bzFile};
use crate::bzlib::scc_2_root_2_BZ2_bzDecompressEnd::*;


// BZ2_bzReadClose (
//       int* bzerror,
//       BZFILE* b
//    )

// BZ2_bzReadClose ( int *bzerror, BZFILE *b )

extern crate libc; // For using FILE and related C functions
use std::ptr;

// You provided most of the type definitions, so I won't reiterate them here. 

pub struct bzFile {
 pub   handle: *mut libc::FILE,
 pub   buf: [libc::c_char; 5000], // char in C is c_char in Rust's libc crate.
 pub   bufN: i32,
 pub   writing: bool,
 pub    strm: bz_stream,
 pub   lastErr: i32,
 pub   initialisedOk: bool,
}

type BZFILE = *mut bzFile; // Using a raw pointer to represent the opaque BZFILE

pub fn BZ2_bzReadClose(bzerror: &mut Option<i32>, b: BZFILE) {
    if let Some(bzf) = unsafe { b.as_mut() } { // Dereference the BZFILE pointer safely
        // Based on the C code, setting lastErr and bzerror here
        bzf.lastErr = 0;
        if let Some(err) = bzerror {
            *err = 0;
        }

        if bzf.writing {
            bzf.lastErr = -1;
            if let Some(err) = bzerror {
                *err = -1;
            }
            return;
        }

        if bzf.initialisedOk {
            // Call the bz2_bz_decompress_end function
            bz2_bz_decompress_end(&mut bzf.strm);
        }

        // Free memory (You might want to use a specific deallocating function here)
        unsafe {
            ptr::drop_in_place(b);
        }
    } else {
        // bzf is None
        if let Some(err) = bzerror {
            *err = 0;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use libc;
    #[test]
    fn test_bz2_bz_read_close() {
        // 创建一个dummy bzFile结构
        let dummy_file = unsafe {libc::tmpfile()};
        let dummy_stream = bz_stream {
            next_in: ptr::null_mut(),
            avail_in: 0,
            total_in_lo32: 0,
            total_in_hi32: 0,
            next_out: ptr::null_mut(),
            avail_out: 0,
            total_out_lo32: 0,
            total_out_hi32: 0,
            state: ptr::null_mut(),
            bzalloc: None,
            bzfree: None,
            opaque: ptr::null_mut(),
        };

        let dummy_bzfile = bzFile {
            handle: dummy_file,
            buf: [0; 5000],
            bufN: 0,
            writing: false, // 确保它是读取模式
            strm: dummy_stream,
            lastErr: 0,
            initialisedOk: true, // 模拟一个已经初始化的文件流
        };

        let mut bzerror: Option<i32> = Some(0);
        let b = Box::into_raw(Box::new(dummy_bzfile));

        BZ2_bzReadClose(&mut bzerror, b);

        // 验证bzerror是否为0，表示没有错误
        assert_eq!(bzerror, Some(0));

        // 你可能还需要更多的测试来处理各种可能的场景和边缘情况
    }
}
