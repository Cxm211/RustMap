
use crate::bzlib::BZ2_bzReadClose::bz2_bz_read_close;
use crate::bzlib::BZ2_bzWriteClose::bz2_bz_write_close;

pub fn bz2_bzclose(b: Option<&mut bzFile>) {
    let mut bzerr: i32 = 0;

    if b.is_none() {
        return;
    }

    let bzf = b.unwrap();
    let fp = bzf.handle;

    if bzf.writing {
        bz2_bz_write_close(Some(&mut bzerr), Some(bzf), 0, None, None);
        if bzerr != 0 {
            bz2_bz_write_close(None, Some(bzf), 1, None, None);
        }
    } else {
        bz2_bz_read_close(Some(&mut bzerr), Some(bzf));
    }

    if !matches!(fp, Some(stdin) | Some(stdout)) {
        if let Some(handle) = fp {
            unsafe {
                libc::fclose(handle);
            }
        }
    }
}
