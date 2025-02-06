use std::fs::File;
use std::io::{self, BufReader, Write, BufRead, Result, Error};
use std::io::prelude::*;
use crate::blocksort::scc_70_fallbackQSort3::*;
use crate::bzlib::scc_62_BZ2_bz__AssertH__fail::*;
use crate::global_vars::bzip2_c1::{rebuild_arr2_from_block, build_block_from_arr2};
use std::path::Path;
use std::mem;
use std::fmt::Display;
use crate::global_vars::sleep_lib::*;
use crate::global_vars::bzip2_c1::*;


fn saveBlockArrayToFile<T>(arr: &Vec<T>, length: usize, filename: &str) -> io::Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;

    write!(file, "[")?;

    for (i, item) in arr.iter().enumerate().take(length) {
        if i < length - 1 {
            write!(file, "{},", item)?;
        } else {
            // 最后一个元素后不加逗号
            write!(file, "{}", item)?;
        }
    }

    write!(file, "]")?;

    Ok(())
}


fn print_int_and_byte_view_of_array(eclass: &[u32], filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::create(filename)?;
    // 🌟 打印 eclass 数组的整数视图 🌟 
    // println!("正在写入 eclass 数组的整数视图到文件...");
    // write!(file, "eclass (int view): [")?;
    // for (i, &val) in eclass.iter().enumerate() {
    //     write!(file, "{}{}", val, if i < eclass.len() - 1 { ", " } else { "" })?;
    // }
    // writeln!(file, "]")?;

    // 🌟 准备 eclass 数组的字节视图 🌟
    // println!("准备 eclass 数组的字节视图...");
    let mut byte_view = Vec::new();
    for &val in eclass.iter() {
        byte_view.extend_from_slice(&val.to_ne_bytes());
    }

    // 🌟 将字节视图写入文件 🌟
    // 场景：当您处理的数据更多地与内存表示、编码、或二进制数据有关时，
    // 字节视图更加有用。例如，在处理编码转换、文件I/O、网络传输或任何涉及字节级操作的场景中，字节视图提供了关于数据如何在内存中表示的深入洞见。
    // println!("正在写入 eclass 数组的字节视图到文件...");
    // write!(file, "eclass (byte view): [")?;
    // for (i, &byte) in byte_view.iter().enumerate() {
    //     write!(file, "{}{}", byte, if i < byte_view.len() - 1 { ", " } else { "" })?;
    // }
    // writeln!(file, "]")?;
    // println!("eclass 数组的整数视图和字节视图已成功写入文件 {}", filename);
    // 返回字节视图
    Ok(byte_view)
}

fn read_dat_file(path: &str) -> Result<(Vec<u32>, Vec<u32>, Vec<u32>, i32, i32)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut fmap_values: Vec<u32> = Vec::new();
    let mut eclass_values: Vec<u32> = Vec::new();
    let mut bhtab_values: Vec<u32> = Vec::new();
    let mut nblock: i32 = 0;
    let mut verb: i32 = 0;

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
            "bhtab" => {
                bhtab_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u32::from_str_radix(s.trim(), 10).ok()));
            },
            "nblock" => nblock = inner.parse().unwrap(),
            "verb" => verb = inner.parse().unwrap(),
            _ => {},
        }
    }

    if fmap_values.is_empty() || eclass_values.is_empty() {
        Err(Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"))
    } else {
        Ok((fmap_values, eclass_values, bhtab_values, nblock, verb))
    }
}

fn save_to_file<T: Display>(data: &Vec<T>, file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;

    write!(file, "vec:[")?;
    for (index, value) in data.iter().enumerate() {
        if index != 0 {
            write!(file, ",")?;
        }
        write!(file, "{}", value)?;
    }
    write!(file, "]")?;

    Ok(())
}
pub fn fallback_sort(fmap: &mut Vec<u32>, eclass: &mut Vec<u32>, bhtab: &mut Vec<u32>, nblock: i32, verb: i32){
    let max_eclass = *eclass.iter().max().unwrap_or(&0) as usize;
    // println!("开始执行 fallback_sort 函数");
    // println!("最大 eclass: {}", max_eclass);

    let mut ftab: Vec<i32> = vec![0; max_eclass + 1];
    let mut ftab_copy: Vec<i32> = vec![0; max_eclass + 1];

    let mut h: i32;
    let mut i: i32;
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;
    let mut r: i32;
    let mut cc: i32;
    let mut cc1: i32;
    let mut n_not_done: i32;
    let mut n_bhtab: i32;

    // println!("开始桶排序...");
    for i in 0..257 {
        ftab[i as usize] = 0;
    }

    // println!("计算 eclass8 数组...");
    // let mut eclass8 = print_int_and_byte_view_of_array(&eclass, "71_int_and_byte_view_rust.txt").unwrap();
    let mut eclass8 = build_block_from_arr2(eclass);
    // println!("更新 ftab 数组...");
    for i in 0..nblock {
        ftab[eclass8[i as usize] as usize] += 1;
    }

    // println!("复制 ftab 到 ftab_copy...");
    for i in 0..256 {
        ftab_copy[i as usize] = ftab[i as usize];
    }
    for i in 1..257 {
        ftab[i as usize] += ftab[(i - 1) as usize];
    }

    // println!("更新 fmap 数组...");
    for i in 0..nblock {
        j = eclass8[i as usize] as i32;
        k = ftab[j as usize] - 1;
        ftab[j as usize] = k;
        fmap[k as usize] = i as u32;
    }

    n_bhtab = 2 + (nblock / 32);
    // println!("初始化 bhtab...");
    for i in 0..n_bhtab {
        bhtab[i as usize] = 0;
    }
    for i in 0..256 {
        bhtab[(ftab[i as usize] as usize) >> 5] |= 1 << (ftab[i as usize] & 31);
    }
    for i in 0..32 {
        bhtab[((nblock + 2 * i) as usize) >> 5] |= 1 << ((nblock + 2 * i) & 31);
        bhtab[((nblock + 2 * i + 1) as usize) >> 5] &= !(1 << ((nblock + 2 * i + 1) & 31));
    }

    h = 1;
    let mut counter1 = 0;
    loop {
        counter1 += 1;
        // println!("进入主循环, 循环计数器 counter1: {}", counter1);
        if verb >= 4 {
            eprintln!("当前排序深度: {}", h);
        }

        j = 0;
        // println!("更新 eclass 数组...");
        for i in 0..nblock {
            if (bhtab[(i as usize) >> 5] & (1u32 << ((i & 31) as u32))) != 0 {
                j = i;
            }
            let mut k: i32 = fmap[i as usize] as i32 - h;
            if k < 0 {
                k += nblock;
            }
            eclass[k as usize] = j as u32;
        }
        eclass8 = build_block_from_arr2(eclass);


        n_not_done = 0;
        r = -1;
        let mut counter_71 = 0;
        loop {
            counter_71 += 1;
            // println!("进入次级循环, 计数器 counter_71: {}", counter_71);
            let mut k = r + 1;
            while (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) != 0) && (k & 0x01f != 0) {
                k += 1;
            }
            if (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) != 0) {
                while bhtab[(k as usize) >> 5] == 0xffffffff {
                    k += 32;
                }
                while (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) != 0) {
                    k += 1;
                }
            }
            l = k - 1;
            if l >= nblock {
                // println!("跳出内部循环");
                break;
            }
            while (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) == 0) && (k & 0x01f != 0) {
                k += 1;
            }
            if (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) == 0) {
                while bhtab[(k as usize) >> 5] == 0x00000000 {
                    k += 32;
                }
                while (bhtab[(k as usize) >> 5] & (1u32 << ((k & 31) as u32)) == 0) {
                    k += 1;
                }
            }
            r = k - 1;
            if r >= nblock {
                // println!("跳出外部循环");
                break;
            }

            if r > l {
                n_not_done += (r - l + 1);
                // println!("在区间 [{}，{}] 之间执行快速排序", l, r);
                fallback_qsort_3(fmap, eclass, l, r);

                let mut cc: i32 = -1;
                for i in l..=r {
                    let cc1 = eclass[fmap[i as usize] as usize];
                    if cc != cc1 as i32 {
                        bhtab[(i as usize) >> 5] |= 1u32 << ((i & 31) as u32);
                        cc = cc1 as i32;
                    }
                }
            }
        }

        if verb >= 4 {
            eprintln!("仍未解决的字符串数: {}", n_not_done);
        }

        h *= 2;
        if h > nblock || n_not_done == 0 {
            // println!("结束主循环");
            break;
        }
    }

    if verb >= 4 {
        eprintln!("重构块中...");
    }
    j = 0;
    // println!("更新 eclass8 数组...");
    for i in 0..nblock {
        while ftab_copy[j as usize] == 0 {
            j += 1;
        }
        ftab_copy[j as usize] -= 1;
        eclass8[fmap[i as usize] as usize] = j as u8;
    }
    *eclass = rebuild_arr2_from_block(&eclass8);
    // println!("fallback_sort 函数执行完毕");
}




    

    

  


 





            


                





#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut fmap,  mut eclass, mut bhtab, nblock, verb) = read_dat_file("/root/rustmap/bzip2/params_fallbackSort_3.dat")?;
        fallback_sort(&mut fmap, &mut eclass, &mut bhtab, nblock, verb); // 修改这里，加上 &
        // let mut file = File::create(Path::new("result.txt"))?;
        // // Write each variable to the file
        // writeln!(file, "fmap:[{}]", fmap.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))?;
        // writeln!(file, "eclass:[{}]", eclass.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))?;
        // writeln!(file, "bhtab:[{}]", bhtab.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))?;
        // writeln!(file, "nblock:{}", nblock)?;
        // writeln!(file, "verb:{}", verb)?;
    
            
        // 调用排序函数

         // 从文件读取第二组参数（预期结果）
        let (expected_fmap, expected_eclass, expected_bhtab, expected_nblock, expected_verb) = read_dat_file("/root/rustmap/bzip2/params_fallbackSort_4.dat")?;

        //  // println!("loSt: {}", loSt);
        //  // println!("hiSt: {}", hiSt);
 
         // 比较loSt和hiSt
         assert_eq!(nblock, expected_nblock, "loSt values do not match");
         assert_eq!(verb, expected_verb, "hiSt values do not match");
 
         // 比较fmap
        //  assert_eq!(fmap, expected_fmap, "fmap values do not match");

        //  assert_eq!(bhtab, expected_bhtab, "bhtab values do not match");
        
         // 比较eclass，假设这两个数组的长度相同
         for i in 0..fmap.len() {
            assert_eq!(fmap[i], expected_fmap[i], "fmap values do not match at index {}", i);
         }
        //  for i in 0..eclass.len() {
        //     assert_eq!(eclass[i], expected_eclass[i], "eclass values do not match at index {}", i);
        //  }
         for i in 0..bhtab.len() {
            assert_eq!(bhtab[i], expected_bhtab[i], "bhtab values do not match at index {}", i);
         }
        // println!("nblock: {}", nblock);
        // // println!("loSt{}",loSt);
        // // println!("hiSt{}",hiSt);
        // Add your assertions here
        Ok(())
    }
}
