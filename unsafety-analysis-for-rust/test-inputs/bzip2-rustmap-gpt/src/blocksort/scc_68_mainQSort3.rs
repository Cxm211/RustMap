// 引入必要的模块
use crate::blocksort::scc_29_mainSimpleSort::*;
use crate::blocksort::scc_30_root_20_mmed3::*;
use crate::bzlib::scc_62_BZ2_bz__AssertH__fail::*;
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Error, ErrorKind};
use std::num::ParseIntError;

// 注意：对于mmed3和BZ2_bz__AssertH__fail函数，我们需要更多信息
fn read_dat_file(path: &str) -> Result<(Vec<u32>, Vec<u8>, Vec<u16>, u32, u32, u32, u32, Option<u32>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ptr_values: Vec<u32> = Vec::new();
    let mut block_values: Vec<u8> = Vec::new();
    let mut quadrant_values: Vec<u16> = Vec::new();
    let mut nblock: u32 = 0;
    let mut lo: u32 = 0;
    let mut hi: u32 = 0;
    let mut d: u32 = 0;
    let mut budget: Option<u32> = None;

    for line in reader.lines() {
        let line = line?;
        let inner = line.split(":").nth(1).unwrap_or("").trim();

        match line.split(":").nth(0).unwrap_or("") {
            "ptr" => {
                ptr_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| s.trim().parse::<u32>().ok()));
            },
            "block" => {
                block_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| s.trim().parse::<u8>().ok()));
            },
            "quadrant" => {
                quadrant_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| s.trim().parse::<u16>().ok()));
            },
            "nblock" => nblock = inner.parse().unwrap(),
            "loSt" => lo = inner.parse().unwrap(),
            "hiSt" => hi = inner.parse().unwrap(),
            "dSt" => d = inner.parse().unwrap(),
            "budget" => budget = Some(inner.parse().map_err(|e: ParseIntError| Error::new(ErrorKind::InvalidData, e.to_string()))?),
            _ => {},
        }
    }

    if ptr_values.is_empty() || block_values.is_empty() || quadrant_values.is_empty() {
        Err(Error::new(ErrorKind::InvalidData, "数据行数不足"))
    } else {
        Ok((ptr_values, block_values, quadrant_values, nblock, lo, hi, d, budget))
    }
}



// 事前声明这些类型以提供清晰性
type Int32 = i32; // 为了清晰起见，明确这是i32
type UInt32 = u32; // 同上，这是u32
type UChar = u8; // 这是u8
type UInt16 = u16; // 这是u16

pub fn main_q_sort3(
    ptr: &mut [UInt32], 
    block: &[UChar], 
    quadrant: &[UInt16], 
    nblock: Int32, 
    mut lo_st: Int32, 
    mut hi_st: Int32, 
    d_st: Int32, 
    budget: &mut Int32 
) {
    const STACK_SIZE: usize = 100;
    let mut sp = 0;
    let mut stack_lo: [Int32; STACK_SIZE] = [0; STACK_SIZE];
    let mut stack_hi: [Int32; STACK_SIZE] = [0; STACK_SIZE];
    let mut stack_d: [Int32; STACK_SIZE] = [0; STACK_SIZE];
    let mut next_lo: [Int32; 3] = [0; 3];
    let mut next_hi: [Int32; 3] = [0; 3];
    let mut next_d: [Int32; 3] = [0; 3];

    stack_lo[sp] = lo_st;
    stack_hi[sp] = hi_st;
    stack_d[sp] = d_st;
    sp += 1;

    while sp > 0 {
        if sp >= STACK_SIZE - 2 {
            bz2_bz_assert_h_fail(1001);
        }
        sp -= 1;
        let lo = stack_lo[sp];
        let hi = stack_hi[sp];
        let d = stack_d[sp];

        if hi - lo < 20 || d > 14 {
            main_simple_sort(ptr, block, quadrant, nblock, lo, hi, d, budget);
            if *budget < 0 {
                return;
            }
            continue;
        }

        let med = mmed3(
            block[ptr[lo as usize] as usize + d as usize],
            block[ptr[hi as usize] as usize + d as usize],
            block[ptr[((lo + hi) >> 1) as usize] as usize + d as usize]
        ) as Int32;

        let mut un_lo = lo;
        let mut lt_lo = lo;
        let mut un_hi = hi;
        let mut gt_hi = hi;

        while un_lo <= un_hi {
            while un_lo <= un_hi {
                let n = block[ptr[un_lo as usize] as usize + d as usize] as Int32 - med;
                if n == 0 {
                    ptr.swap(un_lo as usize, lt_lo as usize);
                    lt_lo += 1;
                    un_lo += 1;
                    continue;
                }
                if n > 0 {
                    break;
                }
                un_lo += 1;
            }
            while un_lo <= un_hi {
                let n = block[ptr[un_hi as usize] as usize + d as usize] as Int32 - med;
                if n == 0 {
                    ptr.swap(un_hi as usize, gt_hi as usize);
                    gt_hi -= 1;
                    un_hi -= 1;
                    continue;
                }
                if n < 0 {
                    break;
                }
                un_hi -= 1;
            }
            if un_lo <= un_hi {
                ptr.swap(un_lo as usize, un_hi as usize);
                un_lo += 1;
                un_hi -= 1;
            }
        }

        if gt_hi < lt_lo {
            stack_lo[sp] = lo;
            stack_hi[sp] = hi;
            stack_d[sp] = d + 1;
            sp += 1;
            continue;
        }

        let n = std::cmp::min(lt_lo - lo, un_lo - lt_lo);
        let m = std::cmp::min(hi - gt_hi, gt_hi - un_hi);

        for i in 0..n as usize {
            ptr.swap(lo as usize + i, un_lo as usize - n as usize + i);
        }

        for i in 0..m as usize {
            ptr.swap(un_lo as usize + i, hi as usize - m as usize + 1 + i);
        }

        next_lo[0] = lo;
        next_hi[0] = lo + un_lo - lt_lo - 1;
        next_d[0] = d;
        
        next_lo[1] = hi - (gt_hi - un_hi) + 1;
        next_hi[1] = hi;
        next_d[1] = d;
        
        next_lo[2] = lo + un_lo - lt_lo;
        next_hi[2] = hi - (gt_hi - un_hi);
        next_d[2] = d + 1;

        let mut sequences = [
            (next_lo[0], next_hi[0], next_d[0]),
            (next_lo[1], next_hi[1], next_d[1]),
            (next_lo[2], next_hi[2], next_d[2]),
        ];
        
        sequences.sort_by(|a, b| (b.1 - b.0).cmp(&(a.1 - a.0)));

        for (l, h, d_val) in &sequences {
            stack_lo[sp] = *l;
            stack_hi[sp] = *h;
            stack_d[sp] = *d_val;
            sp += 1;
        }
    }
}



// 测试函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut ptr, block, mut quadrant, nblock, lo, hi, d, budget_option) = read_dat_file("/root/rustmap/bzip2/params_mainQSort3_1.dat")?;

        let mut budget = match budget_option {
            Some(value) => value as Int32,
            None => 0,  // Or any default value you see fit
        };
        // // println!("Before mainQSort3:");
        // // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
    
        // Call the main_q_sort3 function with the data read
        main_q_sort3(&mut ptr, &block, &mut quadrant, nblock as Int32, lo as Int32, hi as Int32, d as Int32, &mut budget);
      
        // // println!("After mainQSort3:");
        // // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
        // ... you can continue with your assertions here ...
        Ok(())
    }
}
