use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::fmt;
use crate::blocksort::scc_28_root_19_mainGtU::*;
static INCS: [i32; 14] = [1, 4, 13, 40, 121, 364, 1093, 3280,
                   9841, 29524, 88573, 265720,
                   797161, 2391484];

pub fn main_simple_sort(
                    ptr: &mut [u32],      // 待排序数组
                    block: &[u8],         // 包含字符的块
                    quadrant: &[u16],     // 四分区数组
                    nblock: i32,          // 块的大小
                    mut lo: i32,          // 排序开始的索引
                    mut hi: i32,          // 排序结束的索引
                    d: i32,               // 索引的偏移量
                    budget: &mut i32      // 预算（可能与循环迭代次数有关）
                ) {
                    let mut i: i32;
                    let mut j: i32;
                    let mut h: i32;
                    let big_n: i32 = hi - lo + 1;
                    if big_n < 2 {
                        return;
                    }
                
                    let mut hp: i32 = 0;
                    // 这里，我们假设incs是一个全局的数组
                    while INCS[hp as usize] < big_n {
                        hp += 1;
                    }
                    hp -= 1;
                
                    while hp >= 0 {
                        h = INCS[hp as usize]; 
                        i = lo + h;
                
                        loop {
                            if i > hi {
                                break;
                            }
                            let v: u32 = ptr[i as usize];
                
                            j = i;
                            while main_gt_u(ptr[(j-h) as usize] + d as u32, v + d as u32, block, quadrant, nblock as u32, budget) {
                                ptr[j as usize] = ptr[(j-h) as usize];
                                j -= h;
                                if j <= lo + h - 1 {
                                    break;
                                }
                            }
                            ptr[j as usize] = v;
                            i += 1;
                
                            if i > hi { break; }
                            let v = ptr[i as usize];
                            j = i;
                            while main_gt_u(ptr[(j-h) as usize] + d as u32, v + d as u32, block, quadrant, nblock as u32, budget) {
                                ptr[j as usize] = ptr[(j-h) as usize];
                                j -= h;
                                if j <= lo + h - 1 {
                                    break;
                                }
                            }
                            ptr[j as usize] = v;
                            i += 1;
                
                            if i > hi { break; }
                            let v = ptr[i as usize];
                            j = i;
                            while main_gt_u(ptr[(j-h) as usize] + d as u32, v + d as u32, block, quadrant, nblock as u32, budget) {
                                ptr[j as usize] = ptr[(j-h) as usize];
                                j -= h;
                                if j <= lo + h - 1 {
                                    break;
                                }
                            }
                            ptr[j as usize] = v;
                            i += 1;
                
                            if *budget < 0 { return; }
                        }
                
                        hp -= 1;
                    }
                
            // 在函数结尾处打印
    // // println!("After main_simple_sort:");
    // // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
    // // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);

    }
    



fn read_dat_file(path: &str) -> Result<(Vec<u32>, Vec<u8>, Vec<u16>, u32, u32, u32, u32, u32)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ptr_values: Vec<u32> = Vec::new();
    let mut block_values: Vec<u8> = Vec::new();
    let mut quadrant_values: Vec<u16> = Vec::new();
    let mut nblock: u32 = 0;
    let mut lo: u32 = 0;
    let mut hi: u32 = 0;
    let mut d: u32 = 0;
    let mut budget: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let inner = line.split(":").nth(1).unwrap_or("").trim();

        match line.split(":").nth(0).unwrap_or("") {
            "ptr" => {
                ptr_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| s.trim().parse::<u32>().ok()));
            },
            "block" => {
                block_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u8::from_str_radix(s.trim(), 16).ok()));
            },
            "quadrant" => {
                quadrant_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u16::from_str_radix(s.trim(), 16).ok()));
            },
            "nblock" => nblock = inner.parse().unwrap(),
            "lo" => lo = inner.parse().unwrap(),
            "hi" => hi = inner.parse().unwrap(),
            "d" => d = inner.parse().unwrap(),
            "budget" => budget = inner.parse().unwrap(),
            _ => {},
        }
    }


    if ptr_values.is_empty() || block_values.is_empty() || quadrant_values.is_empty() {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"))
    } else {
        Ok((ptr_values, block_values, quadrant_values, nblock, lo, hi, d, budget))
    }
}





mod tests {
    use super::*;
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut ptr, mut block, mut quadrant, nblock, lo, hi, d, budget) = read_dat_file("/root/crown-rust/bzip2/params_mainSimpleSort_1.dat")?;

        // assert_eq!(nblock, 98170);
        // assert_eq!(lo, 59890);
        // assert_eq!(hi, 59891);
        // assert_eq!(d, 2);
        // assert_eq!(budget, 883530);

        // assert_eq!(ptr[lo as usize], 19913);  // 假设此处是ptr[lo]的正确值
        // assert_eq!(ptr[hi as usize], 19782);  // 假设此处是ptr[hi]的正确值

        let nblock_i32 = nblock as i32;
        let lo_i32 = lo as i32;
        let hi_i32 = hi as i32;
        let d_i32 = d as i32;
        let mut budget_i32 = budget as i32;

        // // println!("Before main_simple_sort:");
        // // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
    

        main_simple_sort(&mut ptr, &mut block, &mut quadrant, nblock_i32, lo_i32, hi_i32, d_i32, &mut budget_i32);
        // // println!("After main_simple_sort:");
        // // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
    
    

        // 对于block和quadrant你可以添加其他的assert来验证数据

        Ok(())
    }
}

/*
pub fn main_simple_sort(
    ptr: &mut [u32],       // 待排序数组
    block: &mut [u8],      // 包含字符的块
    quadrant: &mut [u16],  // 四分区数组
    nblock: usize,         // 块的大小
    lo: i32,               // 排序开始的索引
    hi: i32,               // 排序结束的索引
    d: i32,                // 索引的偏移量
    budget: &mut i32       // 预算（可能与循环迭代次数有关）
)
*/