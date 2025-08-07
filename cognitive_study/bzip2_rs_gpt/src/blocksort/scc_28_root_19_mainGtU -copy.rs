use std::cmp::Ordering;

pub fn main_gt_u(
    mut i1: usize,      // 第一个索引
    mut i2: usize,      // 第二个索引
    block: &mut [u8],   // 包含字符的块
    quadrant: &mut [u16], // 四分区数组
    nblock: usize,      // 块的大小
    budget: &mut i32    // 预算（可能与循环迭代次数有关）
) -> bool {

    // 初始化k为nblock加上8，为了后续处理整数倍的8个元素
    let mut k = nblock + 8;

    // 首先，处理前12个元素。比较block中的字符，决定i1和i2哪一个应该在前面。
    for _ in 0..12 {
        match block[i1].cmp(&block[i2]) {
            Ordering::Greater => return true,  // 如果i1处的字符大于i2处的字符，返回true
            Ordering::Less => return false,   // 如果i1处的字符小于i2处的字符，返回false
            _ => {}                           // 相等的情况下，继续比较下一个字符
        }
        i1 += 1;
        i2 += 1;
    }

    // 由于已经处理了12个元素，所以预算需要减少1次（实际上是12/8的预算）
    *budget -= 1;

    // 继续处理block和quadrant的其他元素
    // 为了确保处理的元素数是8的倍数，使用了外部的while循环
    while k >= 8 {
        for _ in 0..8 {
            // 再次比较block中的字符
            match block[i1].cmp(&block[i2]) {
                Ordering::Greater => return true,
                Ordering::Less => return false,
                _ => {}  // 相等的情况下，继续比较下一个字符
            }
            // 比较quadrant中的值
            match quadrant[i1].cmp(&quadrant[i2]) {
                Ordering::Greater => return true,
                Ordering::Less => return false,
                _ => {}  // 相等的情况下，继续比较下一个值
            }
            i1 += 1;
            i2 += 1;

            // 如果索引超出nblock的范围，进行环绕处理
            if i1 >= nblock { i1 -= nblock; }
            if i2 >= nblock { i2 -= nblock; }
        }
        k -= 8;  // 减少8，表示已经处理了8个元素
        *budget -= 1;  // 对于每8个处理的元素，预算减少1
    }

    // 如果上面的所有比较都不能确定i1和i2的顺序，则默认返回false
    false
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_gt_u() {
        let nblock = 98170;
        let mut block: Vec<u8> = vec![0; nblock + 20];
        let mut quadrant: Vec<u16> = vec![0; nblock + 20];
        
        // Original test case
        let mut budget1 = 883445;
        assert_eq!(main_gt_u(90047, 91194, &mut block, &mut quadrant, nblock, &mut budget1), false);
        assert_eq!(budget1, 871172);
        
        // New test case based on the provided scenario
        let mut budget2 = 883290;
        assert_eq!(main_gt_u(53306, 38287, &mut block, &mut quadrant, nblock, &mut budget2), false);
        assert_eq!(budget2, 871017);
    }
}


    



















       
   
    
