use std::cmp::Ordering;


type Bool = bool;

pub fn main_gt_u(
    mut i1: u32,      // 第一个索引
    mut i2: u32,      // 第二个索引
    block: &[u8],     // 包含字符的块
    quadrant: &[u16], // 四分区数组
    nblock: u32,      // 块的大小
    budget: &mut i32  // 预算（可能与循环迭代次数有关）
) -> Bool {
    
    let mut k: i32 = nblock as i32 + 8; 
    let mut c1: u8;
    let mut c2: u8;
    let mut s1: u16;
    let mut s2: u16;

    for _ in 0..12 {
        c1 = block[i1 as usize];
        c2 = block[i2 as usize];
        if c1 != c2 {
            return c1 > c2;
        }
        i1 += 1;
        i2 += 1;
    }

    while k >= 0 {
        for _ in 0..8 {
            c1 = block[i1 as usize];
            c2 = block[i2 as usize];
            if c1 != c2 {
                return c1 > c2;
            }
            s1 = quadrant[i1 as usize];
            s2 = quadrant[i2 as usize];
            if s1 != s2 {
                return s1 > s2;
            }
            i1 += 1;
            i2 += 1;

            // 更新索引，如果超出块的大小，则回到块的开头。
            if i1 >= nblock {
                i1 -= nblock;
            }
            if i2 >= nblock {
                i2 -= nblock;
            }
        }
        k -= 8;  // 更新k的值
        *budget -= 1; // 更新预算值。这可能用于外部函数限制这个函数的迭代次数。
    }

    false // 如果函数到达这里，说明两个字符串是相同的。返回false表示它们是相等的。
}



use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn read_dat_file(path: &str) -> Result<(u64, u64, Vec<u8>, Vec<u16>, u64, u64)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut i1: u64 = 0;
    let mut i2: u64 = 0;
    let mut block_values: Vec<u8> = Vec::new();
    let mut quadrant_values: Vec<u16> = Vec::new();
    let mut nblock: u64 = 0;
    let mut budget: u64 = 0;

    for line in reader.lines() {
        let line = line?;
        let inner = line.split(":").nth(1).unwrap_or("").trim();

        match line.split(":").nth(0).unwrap_or("") {
            "i1" => i1 = inner.parse().unwrap(),
            "i2" => i2 = inner.parse().unwrap(),
            "block" => {
                block_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u8::from_str_radix(s.trim(), 16).ok()));
            },
            "quadrant" => {
                quadrant_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u16::from_str_radix(s.trim(), 16).ok()));
            },
            "nblock" => nblock = inner.parse().unwrap(),
            "budget" => budget = inner.parse().unwrap(),
            _ => {},
        }
    }

    if block_values.is_empty() || quadrant_values.is_empty() {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"))
    } else {
        Ok((i1, i2, block_values, quadrant_values, nblock, budget))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (i1, i2, mut block, mut quadrant, nblock, budget) = read_dat_file("/root/crown-rust/bzip2/params_mainGtU_1.dat")?;

        // println!("Before mainGtU:");
        let mut bugdget_as_i32  = budget as i32;
        // println!("bugdget_as_i32{}", bugdget_as_i32);
        let result = main_gt_u(
            i1 as u32,
            i2 as u32,
            &mut block[..],
            &mut quadrant[..],
            nblock as u32,
            &mut bugdget_as_i32,
        );
        // println!("after bugdget_as_i32{}", bugdget_as_i32);
        
        // Add more assertions as necessary, e.g. for block and quadrant
        
        Ok(())
    }
}
