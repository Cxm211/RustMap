// use crate::global_vars::bzip2_c1::{BZ2_crc32Table, EState};
use crate::global_vars::bzip2_c1::*;
pub fn add_pair_to_block(s: &mut EState) {
    // println!("37_进入函数 add_pair_to_block");
    let ch = s.state_in_ch as u8;
    // println!("37_ch: {}", ch);
    for _ in 0..s.state_in_len {
        let index = (s.blockCRC >> 24) ^ s.state_in_ch;
        // println!("index: {}", index);
        s.blockCRC = (s.blockCRC << 8) ^ unsafe { BZ2_crc32Table[index as usize] };
        // println!("s.blockCRC: {}", s.blockCRC);
    }

    s.inUse[s.state_in_ch as usize] = true;
    // println!("37_s.inUse[s.state_in_ch] 设置为 true");
    
    match s.state_in_len {
        1 | 2 | 3 | _ => {
            // println!("37_s.state_in_len: {}", s.state_in_len);
            for _ in 0..s.state_in_len {
                // 检查 s.block 的长度是否足够以进行下一步操作 💡💡💡
                if s.nblock < s.block.len() as i32 {
                    s.block[s.nblock as usize] = ch;
                    s.nblock += 1;
                    // println!("37_写入 block: {}", ch);
                } else {
                    // 在这里处理 Vec 长度不足的情况
                    // 比如可以扩展 Vec，或者记录错误
                    // println!("37_警告：block 长度不足");
                }
            }
            if s.state_in_len > 3 {
                s.inUse[(s.state_in_len - 4) as usize] = true;
                // println!("设置 s.inUse[s.state_in_len - 4] 为 true");
                // 确保 s.block 的长度至少为 s.nblock + 1，以便安全地添加新元素 💡💡💡
                if s.block.len() > s.nblock as usize {
                    s.block[s.nblock as usize] = (s.state_in_len - 4) as u8;
                    s.nblock += 1;
                    // println!("写入 block: {}", (s.state_in_len - 4));
                } else {
                    // println!("警告：block 长度不足，无法写入新元素");
                }
            }
        }
    }
    // println!("离开函数 add_pair_to_block");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_pair_to_block() {
        let mut s = EState::default();
        // 初始化s.block
        // 当使用Box::into_raw时，返回的是一个指向单一元素的裸指针，而不是数组的裸指针。
        // s.block = Some(Box::into_raw(Box::new([0u8; 100])) as *mut u8); // 更正为指向u8的指针
        
        s.state_in_ch = 'a' as u32;
        s.state_in_len = 2;

        add_pair_to_block(&mut s);

        assert_eq!(s.block.get(0), Some(&('a' as u8)));
        assert_eq!(s.block.get(1), Some(&('a' as u8)));

        // assert_eq!(unsafe { *s.block.expect("Block pointer is None").offset(1) }, 'a' as u8);

        assert!(s.block.len() > 1, "Block vector is too short");
        // 使用 Vec 的索引访问来替代原来的裸指针访问
        assert_eq!(s.block[1], 'a' as u8);
        
        assert_eq!(s.nblock, 2);
        // ... 其他相关的断言

        // 测试完成后释放s.block
        // 
        drop(s.block);

        // unsafe {
        //     Box::from_raw(s.block.expect("Block pointer is None") as *mut [u8; 100]); // 更正释放指针的方式
        // }
    }
}
