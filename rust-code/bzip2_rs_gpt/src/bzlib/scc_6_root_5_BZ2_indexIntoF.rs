

pub fn bz2_index_into_f(indx: i32, cftab: &[i32; 257]) -> i32 {
    // 定义搜索范围的开始和结束
    let mut nb: i32 = 0;    // 明确标记为 i32 类型
    let mut na: i32 = 256;  // 明确标记为 i32 类型

    // 使用二分查找在cftab中查找indx
    while na - nb != 1 {
        // 计算中点
        let mid = (nb + na) / 2; // 使用除法替代右移操作来避免潜在的溢出

        // 根据中点位置更新搜索范围
        if indx >= cftab[mid as usize] { // 明确注释usize并进行类型转换
            nb = mid;
        } else {
            na = mid;
        }
    }

    // 返回找到的位置
    nb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bz2_index_into_f() {
        let mut cftab = [0; 257];
        for (i, value) in cftab.iter_mut().enumerate() {
            *value = i as i32 * 10;
        }

        assert_eq!(bz2_index_into_f(50, &cftab), 5);
        assert_eq!(bz2_index_into_f(150, &cftab), 15);
        assert_eq!(bz2_index_into_f(2550, &cftab), 255);
    }
}
