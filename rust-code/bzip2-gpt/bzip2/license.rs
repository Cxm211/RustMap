use crate::bzip2::BZ2_bz__AssertH__fail::BZ2_bzlibVersion;

use std::ffi::CString;
use libc::stderr;

pub fn license() {
    unsafe {
        eprintln!(
            "bzip2, a block-sorting file compressor.  Version {}.\n\
             \n\
             Copyright (C) 1996-2019 by Julian Seward.\n\
             \n\
             This program is free software; you can redistribute it and/or modify\n\
             it under the terms set out in the LICENSE file, which is included\n\
             in the bzip2 source distribution.\n\
             \n\
             This program is distributed in the hope that it will be useful,\n\
             but WITHOUT ANY WARRANTY; without even the implied warranty of\n\
             MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\
             LICENSE file for more details.\n\
             \n",
            CString::from_raw(BZ2_bzlibVersion() as *mut c_char)
                .to_str()
                .unwrap_or("<invalid>")
        );
    }
}