pub fn bz2_hb_assign_codes(
    code: &mut [i32],    // Equivalent to `Int32*`
    length: &[u8],       // Equivalent to `UChar*` (in Rust, it's `[u8]`)
    min_len: i32,
    max_len: i32,
    alpha_size: i32,
) {
    let mut vec: i32 = 0;
    
    // Loop through the range of lengths from min_len to max_len
    for n in min_len..=max_len {
        for i in 0..alpha_size {
            if length[i as usize] == n as u8 {
                code[i as usize] = vec;
                vec += 1;
            }
        }
        vec <<= 1;  // Left shift to prepare for the next length
    }
}
