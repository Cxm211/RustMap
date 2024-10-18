

use crate::bzlib::BZ_SETERR::bz_seterr;
use crate::bzFile;

pub fn bz2_bz_read_get_unused(
    bzerror: Option<&mut i32>,
    b: Option<&mut bzFile>,
    unused: Option<&mut *mut u8>,
    n_unused: Option<&mut i32>,
) {
    let mut bzf = b;

    if bzf.is_none() {
        bz_seterr(bzerror, bzf, -2);
        return;
    }

    let bzf = bzf.unwrap();

    if bzf.lastErr != 4 {
        bz_seterr(bzerror, Some(bzf), -1);
        return;
    }

    if unused.is_none() || n_unused.is_none() {
        bz_seterr(bzerror, Some(bzf), -2);
        return;
    }

    bz_seterr(bzerror, Some(bzf), 0);

    if let Some(n_unused) = n_unused {
        *n_unused = bzf.strm.avail_in as i32;
    }

    if let Some(unused) = unused {
        *unused = bzf.strm.next_in;
    }
}
