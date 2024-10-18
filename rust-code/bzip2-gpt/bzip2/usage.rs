use crate::bzip2::BZ2_bz__AssertH__fail::BZ2_bzlibVersion;
use std::ffi::{CStr, CString};
use libc::stderr;
use std::os::raw::c_char;

pub fn usage(full_prog_name: *const c_char) {
    unsafe {
        eprintln!(
            "bzip2, a block-sorting file compressor.  Version {}.\n\
             \n\
             usage: {} [flags and input files in any order]\n\
             \n\
             -h --help           print this message\n\
             -d --decompress     force decompression\n\
             -z --compress       force compression\n\
             -k --keep           keep (don't delete) input files\n\
             -f --force          overwrite existing output files\n\
             -t --test           test compressed file integrity\n\
             -c --stdout         output to standard out\n\
             -q --quiet          suppress noncritical error messages\n\
             -v --verbose        be verbose (a 2nd -v gives more)\n\
             -L --license        display software version & license\n\
             -V --version        display software version & license\n\
             -s --small          use less memory (at most 2500k)\n\
             -1 .. -9            set block size to 100k .. 900k\n\
             --fast              alias for -1\n\
             --best              alias for -9\n\
             \n\
             If invoked as `bzip2', default action is to compress.\n\
             as `bunzip2',  default action is to decompress.\n\
             as `bzcat', default action is to decompress to stdout.\n\
             \n\
             If no file names are given, bzip2 compresses or decompresses\n\
             from standard input to standard output.  You can combine\n\
             short flags, so `-v -4' means the same as -v4 or -4v, &c.\n\
             \n",
            CString::from_raw(BZ2_bzlibVersion() as *mut c_char)
                .to_str()
                .unwrap_or("<invalid>"),
            CStr::from_ptr(full_prog_name)
                .to_str()
                .unwrap_or("<invalid>")
        );
    }
}
