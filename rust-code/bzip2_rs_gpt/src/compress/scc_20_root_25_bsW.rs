use crate::global_vars::bzip2_c1::{EState};

pub fn bs_w(s: &mut EState, n: i32, v: u32) {
    while s.bsLive >= 8 {
        unsafe {
            //TODO：和zbits相关的代码都是手动改（）
            //NOTE: paper中
            // 直接使用 zbits 的原生指针
            if !s.zbits.is_empty() {
                *s.zbits.as_mut_ptr().add(s.numZ as usize) = (s.bsBuff >> 24) as u8;
            }
        }
        s.numZ += 1;
        s.bsBuff <<= 8;
        s.bsLive -= 8;
    }

    s.bsBuff |= v << (32 - s.bsLive - n);
    s.bsLive += n;
}