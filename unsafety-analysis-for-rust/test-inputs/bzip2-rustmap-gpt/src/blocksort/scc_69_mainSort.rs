use crate::blocksort::scc_68_mainQSort3::*;
use std::io::{self, BufRead, BufReader, Error, Result, Write};
use std::fs::File;
use std::io::stderr;
// use std::fmt::Write;
use std::fmt::Display;
use std::str::FromStr;
use crate::global_vars::sleep_lib::*;

fn read_dat_file(path: &str) -> io::Result<(Vec<u32>, Vec<u8>, Vec<u16>, Vec<u32>, i32, i32, i32)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ptr_values: Vec<u32> = Vec::new();
    let mut block_values: Vec<u8> = Vec::new();
    let mut quadrant_values: Vec<u16> = Vec::new();
    let mut ftab_values: Vec<u32> = Vec::new();
    let mut nblock: i32 = 0;
    let mut verb: i32 = 0;
    let mut budget: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();

        if parts.len() != 2 {
            continue; // Skip lines that do not contain a key-value pair separated by ':'
        }

        let key = parts[0].trim();
        let value = parts[1].trim().trim_matches(|c: char| c == '[' || c == ']').replace("\n", "");

        match key {
            "ptr" => {
                ptr_values = value.split(',')
                    .filter_map(|s| u32::from_str(s.trim()).ok())
                    .collect();
            },
            "block" => {
                block_values = value.split(',')
                    .filter_map(|s| u8::from_str(s.trim()).ok())
                    .collect();
            },
            "quadrant" => {
                quadrant_values = value.split(',')
                    .filter_map(|s| u16::from_str(s.trim()).ok())
                    .collect();
            },
            "ftab" => {
                ftab_values = value.split(',')
                    .filter_map(|s| u32::from_str(s.trim()).ok())
                    .collect();
            },
            "nblock" => {
                nblock = value.parse()
                    .map_err(|e| Error::new(io::ErrorKind::InvalidData, format!("Failed to parse nblock: {}", e)))?;
            },
            "verb" => {
                verb = value.parse()
                    .map_err(|e| Error::new(io::ErrorKind::InvalidData, format!("Failed to parse verb: {}", e)))?;
            },
            "budget" => {
                budget = value.parse()
                    .map_err(|e| Error::new(io::ErrorKind::InvalidData, format!("Failed to parse budget: {}", e)))?;
            },
            _ => {},
        }
    }
    // println!("block_values {}", block_values[0]);

    Ok((ptr_values, block_values, quadrant_values, ftab_values, nblock, verb, budget))
}



fn saveArrayToFile<T>(arr: &Vec<T>, length: usize, filename: &str) -> io::Result<()>
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


const BZ_N_RADIX: usize = 2;
const BZ_N_QSORT: usize = 12;
const BZ_N_SHELL: usize = 18;
const BZ_N_OVERSHOOT: usize = BZ_N_RADIX + BZ_N_QSORT + BZ_N_SHELL + 2;

// 此函数接受向量的可变引用和其他参数，而不是直接操作裸指针。
pub fn main_sort(ptr: &mut Vec<u32>, block: &mut Vec<u8>, quadrant: &mut Vec<u16>, ftab: &mut Vec<u32>, nblock: i32, verb: i32, budget: &mut i32) {
    let mut i: i32;
    let mut j: i32; // usize
    let mut s: u16; 
    let mut old_j: usize;
    let mut old_j_i32: i32;
    let mut k: i32;
    let mut ss: i32;
    let mut sb: i32;
    let mut running_order: Vec<i32> = vec![0; 256];
    let mut bigDone: Vec<u8> = vec![0; 256];
    let mut copyStart: Vec<i32> = vec![0; 256];
    let mut copyEnd: Vec<i32> = vec![0; 256];
    let mut c1: u8;
    let mut num_q_sorted: i32 = 0;
    let mut s: u32;
    // ✅
    // saveArrayToFile(ptr, (nblock + 34) as usize, "69_init_ptr_rust.txt");
    // saveArrayToFile(block, (nblock + 34) as usize, "69_init_block_rust.txt");
    // saveArrayToFile(quadrant, (nblock + 34) as usize, "69_init_quadrant_rust.txt");
    if verb >= 4 {
        eprintln!("        main sort initialise ...\n");
    }

    ftab.resize(65537, 0);
    for i in (0..=65536).rev() {
        ftab[i as usize] = 0;
    }

    j = (block[0] as i32) << 8;
    i = nblock - 1;
    while i >= 3 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as i32) << 8);
        ftab[j as usize] += 1;

        quadrant[(i - 1) as usize] = 0;
        j = (j >> 8) | ((block[(i - 1) as usize] as i32) << 8);
        ftab[j as usize] += 1;

        quadrant[(i - 2) as usize] = 0;
        j = (j >> 8) | ((block[(i - 2) as usize] as i32) << 8);
        ftab[j as usize] += 1;

        quadrant[(i - 3) as usize] = 0;
        j = (j >> 8) | ((block[(i - 3) as usize] as i32) << 8);
        ftab[j as usize] += 1;

        i -= 4;
    }
    // saveArrayToFile(ftab, 65537, "69_after_t2_ftab_rust.txt");
    // saveArrayToFile(quadrant, (nblock + 34) as usize, "69_after_t2_quadrant_rust.txt");

    
    while i >= 0 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as i32) << 8);
        ftab[j as usize] += 1;
        i -= 1;
    }

    // ... (这里应该填充block和quadrant，以保证长度至少为nblock + 34)
    for i in 0..(2 + 12 + 18 + 2) {
        block[(nblock+i) as usize] = block[i as usize];
        quadrant[(nblock+i) as usize] = 0;
    }
    if verb >= 4 {
        eprintln!("        bucket sorting ...\n");
    }
     // 上面貌似无误
    // saveArrayToFile(&block, (nblock + 34) as usize, "69_block_fanxiang_rust.txt");


    // Prefix sum on ftab
    for i in 1..=65536 {
        ftab[i] += ftab[i - 1];
    }
    // 为了准备构建一个16位的数字，其中高8位是之前的字符，低8位是当前字符
    s = (block[0] as u32) << 8;
    i = nblock - 1;

    // Unroll the loop to process four items per iteration
    // 使用计数减4的方式反向遍历块
    while i >= 3 {
        let i_usize = i as usize;
        s = (s >> 8) | ((block[i_usize] as u32) << 8);
        let mut j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = i as u32;
        
        s = (s >> 8) | ((block[i_usize - 1] as u32) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 1) as u32;
        
        s = (s >> 8) | ((block[i_usize - 2] as u32) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 2) as u32;
        
        s = (s >> 8) | ((block[i_usize - 3] as u32) << 8);
        j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = (i - 3) as u32;

        i -= 4;
    }
    // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_fanxiang_rust.txt");


    // Process the remaining elements
    // 处理数据块的最后几个字符（少于4个时）
    while i >= 0 {
        let i_usize = i as usize;
        s = (s >> 8) | ((block[i_usize] as u32) << 8);
        let j = ftab[s as usize] - 1;
        ftab[s as usize] = j;
        ptr[j as usize] = i_usize as u32;
        i -= 1;
    }



    // Shell sort
    for i in 0..=255 {
        bigDone[i] = 0;
        running_order[i] = i as i32;
    }
    // ✅
    // saveArrayToFile(&running_order, 256, "69_ss1_runningOrder_rust.txt");
    // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_rust.txt");

    // Shell sort with gaps
    let mut h = 1;
    while h <= 256 {
        h = 3 * h + 1;
    }
    // println!("🦀️h = {}", h); // 364 一致
    loop {
        h /= 3;
        for i in h..=255 {
            let vv = running_order[i];
            let mut j = i;
            while (ftab[((running_order[(j-h) as usize] + 1) << 8) as usize] - ftab[(running_order[(j-h) as usize] << 8) as usize]) > 
                (ftab[((vv + 1) << 8) as usize] - ftab[(vv << 8) as usize]) {
                running_order[j] = running_order[(j - h) as usize];
                j -= h;
                if j <= (h - 1) {
                    break;
                }
            }
            running_order[j] = vv;
        }
        if h == 1 {
            break;
        }
    }
    
    //这个掩码被设计为关闭第21位。
    // 这意味着如果你有一个i32值，这个掩码与位与操作(&)来确保第21位被设置为0。
    const CLEARMASK: i32 = !(1 << 21); 
    // 被设计来开启第21位。使用这个掩码与位或操作(|)可以将第21位设置为1。
    const SET_MASK: i32 = 1 << 21;
    // 大for loop启动
    let mut num_q_sorted: i32 = 0;
    // 准备写入文件
    let mut lo_file = match File::create("loValues_rust.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create lo_values.txt: {}", e);
            return;
        },
    };
    let mut hi_file = match File::create("hiValues_rust.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create hi_values.txt: {}", e);
            return;
        },
    };
    let mut sb_file = match File::create("sbValues_rust.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create sb_values.txt: {}", e);
            return;
        },
    };
    let mut markBucket_file = match File::create("markBucketValues_rust.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create markBucketValues_rust.txt: {}", e);
            return;
        },
    };
    let mut markBeforeMaskBucket_file = match File::create("markBeforeMaskBucketValues_rust.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create markBeforeMaskBucketValues_rust.txt: {}", e);
            return;
        },
    };

    // ✅
    // saveArrayToFile(&quadrant, (nblock + 34) as usize, "69_quadrant_before_da_loop_rust.txt");
    //✅
    // saveArrayToFile(&running_order, 256, "69_runningOrder_rust.txt");
    // ✅
    // saveArrayToFile(&ftab, 65537, "69_ftab_before_da_loop_rust.txt"); 
    // println!("🦀️ftab[63209]: {}",ftab[63209]); // 97612一致 ✅，但下面的不一致❌
    let mut counter = 0;
    for i in 0..=255 {
        let ss = running_order[i as usize];
        for j in 0..=255 {
            if j != ss { // 进入的52785
                let sb = (ss << 8) + j; //✅ sb相同
                if let Err(e) = writeln!(sb_file, "{}", sb) {
                    eprintln!("Failed to write to sbValues_rust.txt: {}", e);
                    return;
                }
                if counter == 488{
                    // println!("sb = {}", sb); //63209
                    // println!("ftab[sb] = {}", ftab[sb as usize]); // 97612
                    // my_sleep(120);
                }
                if let Err(e) = writeln!(markBeforeMaskBucket_file, "{}", ftab[sb as usize]) {
                    eprintln!("Failed to write to markBeforeMaskBucket_file.txt: {}", e);
                    return;
                }
                if let Err(e) = writeln!(markBucket_file, "{}", ftab[sb as usize] & (SET_MASK) as u32) {
                    eprintln!("Failed to write to markBucket_file.txt: {}", e);
                    return;
                }

                counter+=1;
                if ftab[sb as usize] & (SET_MASK) as u32 == 0 {
                    let lo = ftab[sb as usize] & (CLEARMASK) as u32;
                    let hi = (ftab[sb as usize + 1] & (CLEARMASK) as u32) - 1;
                    // 希尔
                    if let Err(e) = writeln!(lo_file, "{}", lo) {
                        eprintln!("Failed to write to lo_values.txt: {}", e);
                        return;
                    }
                    if let Err(e) = writeln!(hi_file, "{}", hi) {
                        eprintln!("Failed to write to hi_values.txt: {}", e);
                        return;
                    }
                    if hi > lo {
                        if verb >= 4 {
                            eprintln!("        qsort [0x{:x}, 0x{:x}]   done {}   this {}", ss, j, num_q_sorted, hi - lo + 1);
                        }
                        // Replace with the actual quicksort function, if available.
                        main_q_sort3(ptr, block, quadrant, nblock, lo as i32, hi as i32, 2, budget);
                        num_q_sorted += (hi - lo + 1) as i32;
                        if *budget < 0 {
                            return; // You might want to return a Result or specific value here.
                        }
                    }
                }
                ftab[sb as usize] |= (SET_MASK) as u32;
            }
        }
        // let filename = format!("69_hou_ftab_{}_rust.txt", i);
        // // saveArrayToFile(&ftab, 65537, &filename);
        assert!(bigDone[ss as usize] == 0); // Panics if the condition is true. Replace with error handling as needed.



        let mut copyStart: Vec<i32> =  vec![0; 256];
        let mut copyEnd: Vec<i32> = vec![0; 256];
        for j in 0..=255 {
            let base_index = ((j as usize) << 8).wrapping_add(ss as usize);
            copyStart[j as usize] = (ftab[base_index] as i32) & CLEARMASK ;
            copyEnd[j as usize] = ((ftab[base_index + 1] as i32) & CLEARMASK).wrapping_sub(1);
        }
        if i == 155 { // ✅ 进入两个EndStart for-loop之前
            // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_155_init_rust.txt");

            // saveArrayToFile(&copyStart, 256, "69_copyStart_155_init_rust.txt");
            // saveArrayToFile(&copyEnd, 256, "69_copyEnd_155_init_rust.txt");
            // saveArrayToFile(&running_order, 256, "69_runningOrder_155_init_rust.txt");
            // saveArrayToFile(block, (nblock + 34) as usize, "69_block_155_init_rust.txt");
            // saveArrayToFile(&bigDone, 256, "69_bigDone_155_init_rust.txt");
            // ❌重大发现，在进入155前尾出错
            // ss:78头: 15192; 🦀️尾: 15208 🐦尾:15209 
            // println!("copyStart[78]: {}", copyStart[78]);
            // println!("头: {}; 尾: {}, ss:{}",  (ftab[(ss as usize) << 8] & CLEARMASK as u32) as usize, copyStart[ss as usize] as usize, ss);
            // my_sleep(120);
        }
        if i == 156{ // EndStart处理前⚽️
            // saveArrayToFile(&copyStart, 256, "69_copyStart_init_rust.txt");
            // saveArrayToFile(&copyEnd, 256, "69_copyEnd_init_rust.txt");
            // 🔥
            // saveArrayToFile(&bigDone, 256, "69_bigDone_init_rust.txt");
            // 🔥
            // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_init_rust.txt");
        }
        
        // 由于 ftab 的索引需要 usize，我们需要确保表达式的结果是 usize 类型
        // 假定 ss 已经定义在合适的范围内
        // 改copyStart
       //  已经定位出错点155,运行完上面的for-loop ptr🐦🦀️不一致❌
       j = (ftab[(ss as usize) << 8] & CLEARMASK as u32) as i32;
       let mut copy_start_ss = copyStart[ss as usize] as i32;
       
       while j < copy_start_ss {
           let k = if ptr[j as usize] as i32 - 1 < 0 {
               ptr[j as usize] as i32 - 1 + nblock
           } else {
               ptr[j as usize] as i32 - 1
           };
       
           let c1 = block[k as usize] as i32;
           if bigDone[c1 as usize] == 0 {
               ptr[copyStart[c1 as usize] as usize] = k as u32;
               copyStart[c1 as usize] = copyStart[c1 as usize].wrapping_add(1);
               // 如果 c1 等于 ss，更新 copy_start_ss 以反映新的结束条件
               if c1 == ss {
                   copy_start_ss = copyStart[ss as usize] as i32;
               }
           }
       
           // 手动增加 j
           j += 1;
       }
       //  已经定位出错点155,运行完上面的for-loop ptr🐦🦀️不一致❌
       if i == 154 {// 两个EndStart for-loop的分界点,上面的155是进入之前
            // saveArrayToFile(&copyStart, 256,  "69_copyStart_154_zhong_rust.txt");
       }
        if i == 155 {// 两个EndStart for-loop的分界点,上面的155是进入之前
            // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_155_zhong_rust.txt");
            // saveArrayToFile(&copyStart, 256, "69_copyStart_155_zhong_rust.txt");
        }

        
        // 假定 ss 是 i32 类型，因此需要转换为 usize 用于索引
        // 改copyEnd
        j = ((ftab[((ss + 1) as usize ) << 8] & CLEARMASK as u32)) as i32 - 1;
        while j > copyEnd[ss as usize] as i32 {
            let mut k = ptr[j as usize] as i32 - 1; // 假设 ptr 是 Vec<u32> 类型
            if k < 0 {
                k += nblock; // 假设 nblock 是 i32 类型
            }
            let c1 = block[k as usize]; // 假设 block 是 Vec<u8> 类型
            if bigDone[c1 as usize] == 0 { // 假设 bigDone 是 Vec<bool> 类型
                // 这里同时对 copyEnd[c1] 进行了递减，并将 k 转换成 u32 类型赋值
                if let Some(new_end) = copyEnd[c1 as usize].checked_sub(1) {
                    ptr[copyEnd[c1 as usize] as usize] = k as u32;
                    copyEnd[c1 as usize] = new_end;
                } else {
                    // 在这里处理下溢，例如通过错误处理或逻辑调整
                    // 可以选择记录错误、panic 或者其它适当的错误处理方式
                    eprintln!("错误：尝试从0减去1导致下溢。");
                    // 可能需要退出循环或执行其它恢复操作
                    break;
                }                
            }
            if j == 0 { break; } // 在 Rust 中需要显式处理可能的下溢
            j -= 1;
        }

        // Assume AssertH is a macro/function for asserting, we need its Rust equivalent.
        // println!("i: {}", i);
        if i == 155 {
            // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_155_beforeAssert_rust.txt");
        }
        if i == 156{ // EndStart处理后🐒
             // ❌ Difference at index 48: 11458 vs 11455 ❌
            // saveArrayToFile(&copyStart, 256, "69_copyStart_beforeAssert_rust.txt");
            // saveArrayToFile(&copyEnd, 256, "69_copyEnd_beforeAssert_rust.txt");;
            // ✅
            // saveArrayToFile(&quadrant, (nblock + 34) as usize, "69_quadrant_beforeAssert_rust.txt");
            // ✅
            // saveArrayToFile(&bigDone, 256, "69_bigDone_beforeAssert_rust.txt");


            // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_ptr_beforeAssert_rust.txt");
        }
        // assert!(
        //     (copyStart[ss as usize] - 1 == copyEnd[ss as usize])
        //     ||
        //     (copyStart[ss as usize] == 0 && copyEnd[ss as usize] == (nblock - 1) as u32),
        //     "Assertion failed with code 1007"
        // );
        
        // 特别注意 🔥👇
        for j in 0..=255 {
            let sb = (j << 8) + ss;
            ftab[sb as usize] |= (SET_MASK) as u32;
        }
        // 特别注意 🔥👆

        /* 第三步：
        现在[ss]大桶已完成。记录这一事实，并更新象限描述符。
        如果有必要，也要记得更新超出部分的象限。
        "if (i < 255)"测试仅仅是为了跳过最后一个桶的更新，因为更新最后一个桶毫无意义。
        象限数组提供了一种逐渐缓存排序顺序的方法，
        随着它们的出现，可使得在 fullGtU() 中的后续比较更快完成。
        对于重复的块来说，这会产生很大的差异
        （但还不足以避免退回到后备的排序机制，即指数基数排序）。
         */
        if let Some(done) = bigDone.get_mut(ss as usize) {
            *done = 1;
        }

        if i < 255 {
            let bb_start: i32 = (ftab[(ss << 8) as usize] & (CLEARMASK as u32)) as i32;
            let bb_size: i32 = (ftab[((ss + 1) << 8) as usize] & (CLEARMASK as u32)) as i32 - bb_start;
            let mut shifts = 0;

            while (bb_size >> shifts) > 65534 {
                shifts += 1;
            }

            // Safety check for indexing `ptr` and `quadrant`
            if bb_start + bb_size <= (ptr.len()) as i32 {
                for j in (0..bb_size).rev() {
                    if let Some(a2update) = ptr.get((bb_start + j) as usize) {
                        let q_val = (j >> shifts) as u16;
                        quadrant[*a2update as usize] = q_val; // safe as per given context

                        if (*a2update as usize) < BZ_N_OVERSHOOT {
                            quadrant[(*a2update as usize) + nblock as usize] = q_val; // need to check the conversion from i32 to usize
                        }
                    }
                }
            }
            // Assert replacement in Rust, this will panic if the condition is false
            assert!(((bb_size - 1) >> shifts) <= 65535, "Error Code: 1002");
        }
    }
    // ❌
    // saveArrayToFile(&copyEnd, 256, "69_final_copyEnd_rust.txt"); // 一致✅
    // saveArrayToFile(&copyStart, 256, "69_final_copyStart_rust.txt"); // 一致✅
     // 一致 
    // saveArrayToFile(&quadrant, (nblock + 34) as usize, "69_final_quadrant_rust.txt");   
    // saveArrayToFile(ftab, 65536, "69_final_ftab_rust.txt"); // 一致✅
    // 一致✅
    // saveArrayToFile(&block, (nblock + 34) as usize, "69_final_block_rust.txt"); 
    // ❌ Difference at index 0: 97736 vs 8
    // ✅
    // saveArrayToFile(&ptr, (nblock + 34) as usize, "69_final_ptr_rust.txt"); 

}


// 测试函数
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut ptr, mut block, mut quadrant, mut ftab, nblock, verb, mut budget) = read_dat_file("/root/rustmap/bzip2/params_mainSort_1.dat")?;
        // println!("Before mainSort:");
        let lo = 0; // 或者其他默认值
        let hi = ptr.len() - 1; // 或者其他默认值
        /*
            Before mainSort:
                nblock=98170
                len(block)=98204
                len(quadrant)=98204
                len(ftab)=65537
                ptr[0 as usize] = 0
                ptr[98203 as usize] = 0
                budget 883530
         */
        // println!("nblock={}",nblock);
        // println!("len(block)={}",block.len());
        // println!("len(quadrant)={}",quadrant.len());
        // println!("len(ftab)={}",ftab.len());
        // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
        // println!("budget {}", budget);
        
        // 调用 main_sort 函数
        main_sort(&mut ptr, &mut block, &mut quadrant, &mut ftab, nblock as i32, verb, &mut budget);
        // ... 你可以在这里继续你的断言 ...
        Ok(())
    }
}