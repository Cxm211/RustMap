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




// impl EState {
//     fn prepare_new_block(&mut self) {
//         self.nblock = 0;
//         self.numZ = 0;
//         self.state_out_pos = 0;
//         self.blockCRC = 0xffffffff; // Int32 in C, might be i32 in Rust
//         for i in 0..256 {
//             self.inUse[i] = false;
//         }
//         self.blockNo += 1;
//     }

//     pub fn init_RL(&mut self) {
//         self.state_in_ch = 256; 
//         self.state_in_len = 0;
//     }

//     pub fn isempty_RL(&self) -> bool {
//         !(self.state_in_ch < 256 && self.state_in_len > 0)
//     }
//     pub fn flush_RL(&mut self) {
//         if self.state_in_ch < 256 {
//             // add_pair_to_block(s);  需要函数定义
//             todo!(); // 占位符，表示该部分需要进一步实现
//         }
//         self.init_RL();
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::EState;
//     use assert_eq;

//     #[test]
//     fn test_prepare_new_block() {
//         let mut s = EState::default(); // 我们假设EState有默认实现

//         s.prepare_new_block();
//         assert_eq!(s.nblock, 0);
//         assert_eq!(s.numZ, 0);
//         assert_eq!(s.state_out_pos, 0);
//         assert_eq!(s.blockCRC, 0xffffffff);
//         for &i in &s.inUse {
//             assert_eq!(i, false);
//         }
//         assert_eq!(s.blockNo, 1);
//     }

//     #[test]
//     fn test_init_RL() {
//         let mut s = EState::default();

//         s.init_RL();
//         assert_eq!(s.state_in_ch, 256);
//         assert_eq!(s.state_in_len, 0);
//     }

//     #[test]
//     fn test_isempty_RL() {
//         let mut s = EState::default();
//         s.state_in_ch = 255;
//         s.state_in_len = 1;

//         let result = s.isempty_RL();
//         assert_eq!(result, false);

//         s.state_in_ch = 256;
//         s.state_in_len = 0;

//         let result = s.isempty_RL();
//         assert_eq!(result, true);
//     }
// }
