// 导入必要的Rust库
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


// void test_mainGtU(void) {
//     UInt32 nblock = 98170;
//     UChar block[nblock + 20];  
//     UInt16 quadrant[nblock + 20];
//     Int32 budget = 883445;
    
//     // 使用循环来初始化block和quadrant数组
//     for (UInt32 i = 0; i < nblock + 20; i++) {
//         block[i] = 0;
//         quadrant[i] = 0;
//     }

//     // 基于你给的例子：
//     assert(mainGtU(90047, 91194, block, quadrant, nblock, &budget) == FALSE);
//        if(budget != 871172) {
//         printf("Expected budget: 871172, but got: %d\n", budget);
//     }
//     assert(budget == 871172);  // 根据函数的逻辑，budget值应该递减1

//     // 在此处，你可以添加更多的断言来测试其他情况
// }






// 定义数据类型
// type UInt32 = u32;
// type Int32 = i32;
// type UChar = u8;
// type UInt16 = u16;
// type Bool = bool;

// pub fn main_gt_u(mut i1: UInt32, // 第一个索引
//              mut i2: UInt32, // 第二个索引
//              block: &[UChar], // 包含字符的块
//              quadrant: &[UInt16], // 四分区数组
//              nblock: UInt32, // 块的大小
//              budget: &mut Int32) // 预算（可能与循环迭代次数有关）
//              -> Bool {
//     let mut k = nblock as Int32 + 8;

//     loop {
//         for _ in 0..8 {
//             // 注意：这里有可能发生数组越界的风险，需要额外的检查
//             if i1 as usize >= block.len() || i2 as usize >= block.len() {
//                 return false;
//             }
//             let c1 = block[i1 as usize];
//             let c2 = block[i2 as usize];

//             if c1 != c2 {
//                 return c1 > c2;
//             }

//             let s1 = quadrant[i1 as usize];
//             let s2 = quadrant[i2 as usize];

//             if s1 != s2 {
//                 return s1 > s2;
//             }

//             i1 += 1;
//             i2 += 1;

//             // 更新索引，如果超出块的大小，则回到块的开头。
//             if i1 >= nblock {
//                 i1 -= nblock;
//             }
//             if i2 >= nblock {
//                 i2 -= nblock;
//             }
//             // // println!("Comparing i1: {}, i2: {}", i1, i2); // 打印比较的索引
//             // // println!("Characters c1: {}, c2: {}", c1, c2); // 打印比较的字符
//             // // println!("Quadrants s1: {}, s2: {}", s1, s2); // 打印四分区值
//         }
//         k -= 8;
//         *budget -= 1; // 更新预算值

//         if k < 0 {
//             break;
//         }
//     }
//     false
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_main_gt_u() {
//         let block = b"abracadabra";
//         let quadrant: [UInt16; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//         let nblock = 11;
//         let mut budget = 100;

//         // 测试两个相同的索引
//         assert_eq!(main_gt_u(0, 0, block, &quadrant, nblock, &mut budget), false);
       
//         // 测试具有不同字符的两个索引     
//         assert_eq!(main_gt_u(0, 3, block, &quadrant, nblock, &mut budget), false);
   
//         // 测试两个索引，其中一个索引会导致越界
//         assert_eq!(main_gt_u(10, 0, block, &quadrant, nblock, &mut budget), true);
    
//     }
// }
