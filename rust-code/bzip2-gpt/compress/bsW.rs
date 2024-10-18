use crate::EState;

pub fn bs_need_w(s: &mut EState) {
    let zbits = s.zbits.as_mut().expect("zbits must be initialized before calling bs_need_w");

    while s.bs_live >= 8 {
        zbits[s.num_z as usize] = (s.bs_buff >> 24) as u8;  // UChar equivalent is u8
        s.num_z += 1;
        s.bs_buff <<= 8;
        s.bs_live -= 8;
    }
}

pub fn bs_w(s: &mut EState, n: i32, v: u32) {
    bs_need_w(s);
    s.bs_buff |= v << (32 - s.bs_live - n);
    s.bs_live += n;
}
