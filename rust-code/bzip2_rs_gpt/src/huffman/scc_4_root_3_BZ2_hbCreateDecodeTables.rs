


pub fn bz2_hb_create_decode_tables(limit: &mut [i32; 23], // 明确数字类型为i32
    base: &mut [i32; 23],  // 明确数字类型为i32
    perm: &mut Vec<i32>,   // 明确数字类型为i32
    length: &Vec<u8>,      // 使用u8来匹配UChar
    min_len: i32,          // 明确数字类型为i32
    max_len: i32,          // 明确数字类型为i32
    alpha_size: i32) {     // 明确数字类型为i32

let mut pp = 0;

for i in min_len..=max_len {
for j in 0..alpha_size {
if length[j as usize] == i as u8 { // 类型转换
perm[pp as usize] = j; // 类型转换
pp += 1;
}
}
}

for i in 0..23 {
base[i as usize] = 0; // 类型转换
limit[i as usize] = 0; // 类型转换
}

for i in 0..alpha_size as usize {
    base[(length[i] as usize + 1) as usize] += 1;
}

for i in 1..23 {
base[i as usize] += base[(i - 1) as usize]; // 类型转换
}

let mut vec = 0;
for i in min_len..=max_len {
    vec += base[(i + 1) as usize] - base[i as usize];
    limit[i as usize] = vec - 1;
    vec <<= 1;
}


for i in (min_len + 1)..=max_len {
base[i as usize] = ((limit[(i - 1) as usize] + 1) << 1) - base[i as usize]; // 类型转换
}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bz2_hb_create_decode_tables() {
        // 设置测试输入
        let mut limit: [i32; 23] = [0; 23];
        let mut base: [i32; 23] = [0; 23];
        let mut perm: Vec<i32> = vec![0; 23];
        let length: Vec<u8> = vec![2, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8];
        let min_len = 2;
        let max_len = 8;
        let alpha_size = 23;

        bz2_hb_create_decode_tables(&mut limit, &mut base, &mut perm, &length, min_len, max_len, alpha_size);

        // 用预期的输出来检查函数的结果
        let expected_perm: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22];
        let expected_base: [i32; 23] =  [0, 0, 0, 1, 5, 19, 51, 119, 259, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23];
        let expected_limit: [i32; 23] = [0, 0, 0, 3, 13, 31, 67, 139, 281, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(perm, expected_perm);
        assert_eq!(base, expected_base);
        assert_eq!(limit, expected_limit);
    }
}
