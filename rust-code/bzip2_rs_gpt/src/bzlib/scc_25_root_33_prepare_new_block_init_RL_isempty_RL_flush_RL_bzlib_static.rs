use crate::global_vars::bzip2_c1::{EState};
use crate::bzlib::scc_29_root_22_add_pair_to_block::*;
pub fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.numZ = 0;
    s.state_out_pos = 0;
    s.blockCRC = 0xffffffff; // 4294967295
    for i in 0..256 {
        s.inUse[i] = false;
    }
    s.blockNo += 1;
}

pub fn init_rl(s: &mut EState) {
    s.state_in_ch = 256;
    s.state_in_len = 0;
}

pub fn is_empty_rl(s: &EState) -> bool {
    !(s.state_in_ch < 256 && s.state_in_len > 0)
}

pub fn flush_rl(s: &mut EState) {
    if s.state_in_ch < 256 {
        add_pair_to_block(s);
    }
    init_rl(s);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_new_block() {
        let mut s = EState::default();

        prepare_new_block(&mut s);
        assert_eq!(s.nblock, 0);
        assert_eq!(s.numZ, 0);
        assert_eq!(s.state_out_pos, 0);
        assert_eq!(s.blockCRC, 0xffffffff);
        for &i in &s.inUse {
            assert_eq!(i, false);
        }
        assert_eq!(s.blockNo, 1);
    }

    #[test]
    fn test_init_rl() {
        let mut s = EState::default();

        init_rl(&mut s);
        assert_eq!(s.state_in_ch, 256);
        assert_eq!(s.state_in_len, 0);
    }

    #[test]
    fn test_is_empty_rl() {
        let mut s = EState::default();
        s.state_in_ch = 255;
        s.state_in_len = 1;

        let result = is_empty_rl(&s);
        assert_eq!(result, false);

        s.state_in_ch = 256;
        s.state_in_len = 0;

        let result = is_empty_rl(&s);
        assert_eq!(result, true);
    }
}
















