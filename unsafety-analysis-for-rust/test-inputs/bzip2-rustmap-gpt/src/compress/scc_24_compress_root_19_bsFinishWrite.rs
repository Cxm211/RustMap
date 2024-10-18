// 见73
// use crate::global_vars::bzip2_c1::{EState};

// pub fn bs_finish_write(s: &mut EState) {
//     while s.bsLive > 0 {
//         unsafe {
//             // 直接使用 zbits 的原生指针
//             if !s.zbits.is_empty() {
//                 *s.zbits.as_mut_ptr().add(s.numZ as usize) = (s.bsBuff >> 24) as u8;
//                 s.numZ += 1;  // 增加numZ以指向下一个位置
//             }
//         }
//         s.bsBuff <<= 8;
//         s.bsLive -= 8;
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bs_finish_write() {
//         let mut state = EState::default();
//         // 为 zbits 初始化内存
//         state.zbits = vec![0; 1024];
//         bs_finish_write(&mut state);

//         unsafe {
//             // 直接访问 zbits 的内容
//             assert_eq!(state.zbits[0], 0xA5); // 假设第一个位置应该是0xA5

//             // 将 state.numZ 转换为 usize 类型
//             let numZ_index = state.numZ as usize;
//             // 基于state.numZ判断是否正确写入到了内存
//             assert_eq!(state.zbits[numZ_index - 1], 0xA5); // 检查最后一个写入的位置
//         }
//         assert_eq!(state.bsLive, 0);
//     }
// }

