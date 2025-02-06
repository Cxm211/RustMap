



use crate::global_vars::bzip2_c1::{EState};

pub fn bz2_bs_init_write(estate: &mut EState) {
    estate.bsLive = 0;
    estate.bsBuff = 0;
}


