use crate::blocksort::scc_31_root_21_fallbackSimpleSort::*;
use std::fs::File;
use std::io::{BufReader, Result, Error};
use std::io::prelude::*;
use std::time::Instant;

fn read_dat_file(path: &str) -> Result<(Vec<u32>, Vec<u32>, i32, i32)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut fmap_values: Vec<u32> = Vec::new();
    let mut eclass_values: Vec<u32> = Vec::new();
    let mut loSt: i32 = 0;
    let mut hiSt: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let inner = line.split(":").nth(1).unwrap_or("").trim();

        match line.split(":").nth(0).unwrap_or("") {
            "fmap" => {
                fmap_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u32::from_str_radix(s.trim(), 10).ok()));
            },
            "eclass" => {
                eclass_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u32::from_str_radix(s.trim(), 10).ok()));
            },
            "loSt" => loSt = inner.parse().unwrap(),
            "hiSt" => hiSt = inner.parse().unwrap(),
            _ => {},
        }
    }

    if fmap_values.is_empty() || eclass_values.is_empty() {
        Err(Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"))
    } else {
        Ok((fmap_values, eclass_values, loSt, hiSt))
    }
}


pub fn fallback_qsort_3(fmap: &mut Vec<u32>, eclass: &Vec<u32>, lo_st: i32, hi_st: i32) {
    let mut stack_lo = [0i32; 100];
    let mut stack_hi = [0i32; 100];
    let mut r = 0u32;

    let mut sp = 0usize;
    stack_lo[sp] = lo_st;
    stack_hi[sp] = hi_st;
    sp += 1;
    let mut counter_70 = 0;
    while sp > 0 {
        counter_70 += 1;
        assert!(sp < 100 - 1, "Stack overflow in fallback_qsort_3");
        sp -= 1;
        let mut lo = stack_lo[sp];
        let mut hi = stack_hi[sp];

        if hi - lo < 10 {
            // println!("counter_70: {}", counter_70);
            fallback_simple_sort(fmap, eclass, lo, hi);
            continue;
        }

        r = (r.wrapping_mul(7621).wrapping_add(1)) % 32768;
        let r3 = r % 3;
        let med = match r3 {
            0 => eclass[fmap[lo as usize] as usize],
            1 => eclass[fmap[((lo + hi) >> 1) as usize] as usize],
            _ => eclass[fmap[hi as usize] as usize],
        };

        let mut un_lo = lo;
        let mut un_hi = hi;
        let mut lt_lo = lo;
        let mut gt_hi = hi;

        loop {
            while un_lo <= un_hi {
                let n = eclass[fmap[un_lo as usize] as usize] as i32 - med as i32;
                if n == 0 {
                    fmap.swap(un_lo as usize, lt_lo as usize);
                    lt_lo += 1;
                    un_lo += 1;
                } else if n > 0 {
                    break;
                } else {
                    un_lo += 1;
                }
            }

            while un_lo <= un_hi {
                let n = eclass[fmap[un_hi as usize] as usize] as i32 - med as i32;
                if n == 0 {
                    fmap.swap(un_hi as usize, gt_hi as usize);
                    gt_hi -= 1;
                    un_hi -= 1;
                } else if n < 0 {
                    break;
                } else {
                    un_hi -= 1;
                }
            }

            if un_lo > un_hi {
                break;
            }

            fmap.swap(un_lo as usize, un_hi as usize);
            un_lo += 1;
            un_hi -= 1;
        }

        if gt_hi < lt_lo {
            continue;
        }

        let n = ((lt_lo - lo).min(un_lo - lt_lo)) as usize;
        for i in 0..n {
            fmap.swap((lo + i as i32) as usize, (un_lo - n as i32 + i as i32) as usize);
        }

        let m = ((hi - gt_hi).min(gt_hi - un_hi)) as usize;
        for i in 0..m {
            fmap.swap((un_lo + i as i32) as usize, (hi - m as i32 + 1 + i as i32) as usize);
        }

        let n = lo + un_lo - lt_lo - 1;
        let m = hi - (gt_hi - un_hi) + 1;

        if n - lo > hi - m {
            stack_lo[sp] = lo;
            stack_hi[sp] = n;
            sp += 1;
            stack_lo[sp] = m;
            stack_hi[sp] = hi;
            sp += 1;
        } else {
            stack_lo[sp] = m;
            stack_hi[sp] = hi;
            sp += 1;
            stack_lo[sp] = lo;
            stack_hi[sp] = n;
            sp += 1;
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_dat_file() -> Result<()> {
        let start = Instant::now();
        let (mut fmap, eclass, loSt, hiSt) = read_dat_file("/root/crown-rust/bzip2/params_fallbackQSort3_1.dat").unwrap();
        let duration = start.elapsed();
        // println!("read_dat_file took: {:?}", duration);
        let start = Instant::now();
        fallback_qsort_3(&mut fmap, &eclass, loSt, hiSt);
        let duration = start.elapsed();
        // println!("fallback_qsort_3 took: {:?}", duration);
            
        // 调用排序函数
        // let (mut fmap, eclass, loSt, hiSt) = read_dat_file("/root/crown-rust/bzip2/params_fallbackQSort3_2.dat")?;

         // 从文件读取第二组参数（预期结果）
         let (expected_fmap, expected_eclass, expected_loSt, expected_hiSt) = read_dat_file("/root/crown-rust/bzip2/params_fallbackQSort3_2.dat")?;

         // println!("loSt: {}", loSt);
         // println!("hiSt: {}", hiSt);
 
         // 比较loSt和hiSt
         assert_eq!(loSt, expected_loSt, "loSt values do not match");
         assert_eq!(hiSt, expected_hiSt, "hiSt values do not match");
 
         // 比较fmap
         assert_eq!(fmap, expected_fmap, "fmap values do not match");
 
         // 比较eclass，假设这两个数组的长度相同
         for i in 0..eclass.len() {
             assert_eq!(eclass[i], expected_eclass[i], "eclass values do not match at index {}", i);
         }

        // println!("loSt{}",loSt);
        // println!("hiSt{}",hiSt);
        // Add your assertions here
        Ok(())
    }
}
