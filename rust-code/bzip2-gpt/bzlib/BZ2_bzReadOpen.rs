

use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzlib::BZ2_bzDecompressInit::bz2_bz_decompress_init;
use crate::bzFile;
use libc::{FILE, ferror};
use crate::bz_stream;

impl bzFile {
    pub fn new() -> Self {
        bzFile {
            handle: std::ptr::null_mut(), // Assuming no handle is set initially
            buf: [0; 5000],               // Initialize the buffer to zeroes
            bufN: 0,                      // Default bufN to 0
            writing: false,               // Default writing to false
            strm: bz_stream::default(),   // Default bz_stream
            lastErr: 0,                   // Default last error to 0
            initialisedOk: false,         // Default initialisedOk to false
        }
    }
}

impl Default for bz_stream {
    fn default() -> Self {
        bz_stream {
            next_in: std::ptr::null_mut(),
            avail_in: 0,
            total_in_lo32: 0,
            total_in_hi32: 0,
            next_out: std::ptr::null_mut(),
            avail_out: 0,
            total_out_lo32: 0,
            total_out_hi32: 0,
            state: std::ptr::null_mut(),
            bzalloc: None,
            bzfree: None,
            opaque: std::ptr::null_mut(),
        }
    }
}


pub fn bz2_bz_read_open(
    bzerror: Option<&mut i32>,
    f: *mut FILE,
    verbosity: i32,
    small: i32,
    unused: Option<&mut [u8]>,
    n_unused: i32,
) -> Option<&'static mut bzFile> {
    let mut bzf: Option<&mut bzFile> = None;

    bz_seterr(bzerror, bzf, 0);

    if f.is_null()
        || (small != 0 && small != 1)
        || (verbosity < 0 || verbosity > 4)
        || (unused.is_none() && n_unused != 0)
        || (unused.is_some() && (n_unused < 0 || n_unused > 5000))
    {
        bz_seterr(bzerror, bzf, -2);
        return None;
    }

    unsafe {
        if ferror(f) != 0 {
            bz_seterr(bzerror, bzf, -6);
            return None;
        }
    }


    let bzf_ref = Box::leak(Box::new(bzFile::new()));
    bzf = Some(unsafe { &mut *bzf_ref });

    bz_seterr(bzerror, bzf, 0);

    if let Some(bzf) = bzf {
        bzf.initialisedOk = false;
        bzf.handle = f;

        bzf.bufN = 0;
        bzf.writing = false;
        bzf.strm.bzalloc = None;
        bzf.strm.bzfree = None;
        bzf.strm.opaque = std::ptr::null_mut();

        // Copy unused bytes into buffer
        if let Some(unused_buf) = unused {
            for &byte in unused_buf.iter().take(n_unused as usize) {
                bzf.buf[bzf.bufN as usize] = byte as i8;
                bzf.bufN += 1;
            }
        }

        let ret = bz2_bz_decompress_init(&mut bzf.strm, verbosity, small);
        if ret != 0 {
            bz_seterr(bzerror, Some(bzf), ret);
            unsafe {
                Box::from_raw(bzf_ref);
            }
            return None;
        }

        bzf.strm.avail_in = bzf.bufN as u32;
        bzf.strm.next_in = bzf.buf.as_mut_ptr() as *mut u8;


        bzf.initialisedOk = true;
    }

    bzf
}
