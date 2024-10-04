

use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzlib::BZ2_bzDecompressEnd::bz2_bz_decompress_end;
use crate::bzFile;

pub fn bz2_bz_read_close(bzerror: Option<&mut i32>, b: Option<&mut bzFile>) {
    let mut bzf = b;

    bz_seterr(bzerror, bzf, 0);
    
    if bzf.is_none() {
        bz_seterr(bzerror, bzf, 0);
        return;
    }

    let bzf = bzf.unwrap();
    
    if bzf.writing {
        bz_seterr(bzerror, Some(bzf), -1);
        return;
    }

    if bzf.initialisedOk {
        bz2_bz_decompress_end(&mut bzf.strm);
    }
    
    unsafe {
        Box::from_raw(bzf);
    }
}
