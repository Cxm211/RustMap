use crate::bzFile;


pub fn bz_seterr(bzerror: Option<&mut i32>, bzf: Option<&mut bzFile>, eee: i32) {
    if let Some(bzerr) = bzerror {
        *bzerr = eee;
    }
    if let Some(bzf_instance) = bzf {
        bzf_instance.lastErr = eee;
    }
}
