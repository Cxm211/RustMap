use crate::blocksort::scc_68_mainQSort3::*;
use std::io::{self, BufRead, BufReader, Error, Result, Write};
use std::fs::File;
use std::io::stderr;
use crate::global_vars::sleep_lib::*;
use std::fmt::Display;
use std::process;


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


fn save_array_to_file<T: Display>(arr: &[T], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    write!(file, "[")?;

    for (i, item) in arr.iter().enumerate() {
        write!(file, "{}{}", item, if i < arr.len() - 1 { "," } else { "" })?;
    }

    write!(file, "]")?;

    Ok(())
}

fn print_255<T: std::fmt::Display>(slice255: &[T], message: &str) {
    print!("{}[", message);
    for i in 0..=255 {
        print!("{}", slice255[i]);
        if i != 255 {
            print!(", ");
        }
    }
    // println!("]");
    // my_sleep(5);
}

fn print_65537<T: std::fmt::Display>(slice255: &[T], message: &str) {
    print!("{}[", message);
    for i in 0..=65536 {
        print!("{}", slice255[i]);
        if i != 65536 {
            print!(", ");
        }
    }
    // println!("]");
    // my_sleep(5);
}


fn print_255_copy_i32(ftab: &[i32;256], message: &str) {
    print!("{}[", message);
    for i in 0..=255 {
        print!("{}", ftab[i]);
        if i != 255 {
            print!(", ");
        }
    }
    // println!("]");
    // my_sleep(5);
}

pub fn main_sort(
    ptr: &mut [u32],
    block: &mut [u8],
    quadrant: &mut [u16],
    ftab: &mut [u64],
    nblock: i32,
    verb: i32,
    budget: &mut i32
) {
    let mut i: i32;
    let mut j: usize;
    let mut old_j: usize;
    let mut old_j_i32: i32;
    let mut k: i32;
    let mut ss: i32;
    let mut sb: i32;
    let mut running_order: [i32; 256] = [0; 256];
    let mut big_done: [bool; 256] = [false; 256];
    let mut copy_start: [i32; 256] = [0; 256];
    let mut copy_end: [i32; 256] = [0; 256];
    let mut c1: u8;
    let mut num_q_sorted: i32 = 0;
    let mut s: u32;

    if verb >= 4 {
        eprintln!("        main sort initialise ...");
    }
    // 一：🦀️初始化ftab数组为零
    for i in (0..=65535).rev() {
        ftab[i as usize] = 0; // 添加了usize类型转换
    }
    j = (block[0] as u16 as usize) << 8;
    let mut i = nblock as usize - 1;  // 将i改为usize类型
    // 二：用循环展开处理块，更新ftab数组，统计每个两字节对应的出现次数
    // println!("j: {}; i: {}", j, i); //j: 63232; i: 98169
    // save_array_to_file(block, "before2_block_rust.txt");
    // save_array_to_file(quadrant, "before2_quadrant_rust.txt");
    // save_array_to_file(ftab, "before2_ftab_rust.txt");
    // println!("🉑️准备title2,统计每个双字节对应出现次数");
    my_sleep(12);
    let values = vec![1015, 57091, 57311];
    let diff_point: bool = i == 98169 && values.contains(&j);
    let values2 = vec![4060];
    let diff_point2: bool = i == 98169 && values2.contains(&j);
    while i >= 3 {
        quadrant[i] = 0;
        j = (j >> 8) | (((block[i] as u16) << 8) as usize);
        ftab[j as usize] += 1;
        
        quadrant[i - 1] = 0;
        j = (j >> 8) | (((block[i - 1] as u16) << 8) as usize);
        ftab[j as usize] += 1;

        quadrant[i - 2] = 0;
        j = (j >> 8) | (((block[i - 2] as u16) << 8) as usize);
        ftab[j as usize] += 1;
    
        quadrant[i - 3] = 0;
        j = (j >> 8) | (((block[i - 3] as u16) << 8) as usize);
        ftab[j as usize] += 1;
    
        // 使用format!宏来创建包含变量i的文件名
        let filename = format!("ftab_{}_rust.txt", i);
        // println!("filename: {}",filename);
        // // save_array_to_file(ftab, &filename);
        if i >= 4 {
            i -= 4;
        } else {
            break;
        }
    }
    // save_array_to_file(ftab, "ftab_title2_rust.txt");
    // println!("~~~~~~~~~~~~~~~~~~~~~~ ftab_title_2_rust.txt ~~~~~~~~~~~~~~~~~~~~~~");
    // ❌发现不一致❌
    // std::process::exit(0);

    
    // my_sleep(5);


    // 三：处理残余的块元素
    // 对于那些不满足4的倍数的元素，单独处理，并继续更新ftab
    // 假设 i 的类型为 isize 或其他有符号整数类型
    while i >= 0 {
        quadrant[i as usize] = 0;
        j = (j >> 8) | ((block[i as usize] as u16) << 8) as usize;
        ftab[j] += 1;
        if i == 0 {
            break;
        }
        i -= 1;
    }

    // println!("标题三添加前：{}\n睡5秒",nblock); // 98170
    my_sleep(5);
    for i in 0..(2 + 12 + 18 + 2) {
        let idx = nblock as usize + i;  // 这里添加了一个类型转换
        block[idx] = block[i];
        quadrant[idx] = 0;
    }
    
    // save_array_to_file(&block, "/root/crown-rust/bzip2/block_rust.txt");
    // println!("save block_rust array to file and sleep 12s........................");
    if verb >= 4 {
        eprintln!("        bucket sorting ...");
    }
    for i in 1..=65536 {
        ftab[i] += ftab[i - 1];
    }
    // my_sleep(12);
    let mut s = (block[0] as u16) << 8;
    let mut i = nblock as usize - 1;
   // 1. 计算一个新的索引s。
   // 2. 使用s作为ftab的索引，并减少对应的值。
   // 3. 使用ftab[s]的新值更新ptr数组。

   

    // my_sleep(12);
    // 标题五: 循环展开处理block的每四个元素
    // 此时需要确保s,j,ftab,ptr和C中代码一致
    // println!("s: {}; j: {}", s, j);
    // save_array_to_file(ftab, "ftab_qian_t5_rust.txt");
    // save_array_to_file(ptr, "/root/crown-rust/ptr_before_title5.txt");
    // println!("~~~~~~~~~~~~~~~~~~~~  前title5 ~~~~~~~~~~~~~~~~~~~~");
    while i >= 3 {
        s = (s >> 8) | ((block[i] as u16) << 8);
        j = ftab[s as usize] as usize - 1;  // 为了和ptr的索引匹配，将j定义为usize类型
        ftab[s as usize] = j as u64;       // 使用as进行类型转换
        ptr[j] = i as u32;                 // 使用as进行类型转换
        s = (s >> 8) | ((block[i-1] as u16) << 8);
        j = ftab[s as usize] as usize - 1;
        ftab[s as usize] = j as u64;
        ptr[j] = (i - 1) as u32;
        s = (s >> 8) | ((block[i-2] as u16) << 8);
        j = ftab[s as usize] as usize - 1;
        ftab[s as usize] = j as u64;
        ptr[j] = (i - 2) as u32;
        s = (s >> 8) | ((block[i-3] as u16) << 8);
        j = ftab[s as usize] as usize - 1;
        ftab[s as usize] = j as u64;
        ptr[j] = (i - 3) as u32;
        // if i == 0 || i == 1 || i == 2 || i == 3 {
        //     break;
        // }
        // if i > 3 {
        i -= 4;
        // }
    }
    // save_array_to_file(ptr, "/root/crown-rust/ptr_finish_title5.txt");
    my_sleep(12);
    // println!("~~~~~~~~~~~~~~~~~~~~  后title5 ptr ~~~~~~~~~~~~~~~~~~~~");



    // 标题六: 处理block的剩余元素
    while i >= 0 {
        s = (s >> 8) | ((block[i] as u16) << 8);
        let temp = ftab[s as usize];
        j = (temp as usize) - 1; // 将u32转为usize
        ftab[s as usize] = j as u64; // 将usize转为u32
        if i == 156 && s == 48{
            // println!("标题6 s: {}",  ftab[s as usize]);
        }   
        ptr[j] = i as u32; // 将usize转为u32
        if i == 1 {
            break;
        }
        if i > 1{
            i -= 1;
        }
    }
    // save_array_to_file(ptr, "/root/crown-rust/d_qian_156_ptr.txt");
    // 97613
    my_sleep(5);

    // 标题七: 初始化bigDone和runningOrder数组
    for i in 0..=255 {
        big_done[i] = false;
        running_order[i] = i as i32;
    }
    {
        let mut h = 1;
        while h <= 256 {
            h = 3 * h + 1;
        }
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
    }
    // let mut running_order = vec![0; 256];  // 假设你已经有一个名为running_order的向量
    print_255_copy_i32(&running_order,"runningOrder停一下");
    
    // 标题八：遍历runningOrder数组，生成索引sb来访问ftab
    for i in 0..=255 {
        ss = running_order[i as usize];
        // println!("标题八 ss: {}; i:{}",ss, i);
        for j in 0..=255 {
            if j != ss {
                sb = (ss << 8) + j;
                if (ftab[sb as usize] & (1 << 21)) == 0 {
                    let lo = (ftab[sb as usize] & (!((1 << 21)))) as i64 ;
                    // // println!("sb: {}", ftab[(sb + 1) as usize]);
                    // // println!("ftab[(sb + 1) as usize]: {}", ftab[(sb + 1) as usize]);
                    // // println!("Masked value: {}", ftab[(sb + 1) as usize] & (!((1 << 21))));
                    let hi = (ftab[(sb + 1) as usize] & (!((1 << 21)))) as i64 - 1;

                    if hi > lo {
                        if verb >= 4 {
                            eprintln!("        qsort [0x{}, 0x{}]   done {}   this {}", ss, j, num_q_sorted, (hi as i32) - (lo as i32) + 1);
                        }
                        if i == 156 {
                            // save_array_to_file(ptr, "/root/crown-rust/d_qian_156_ptr.txt");
                            // println!("main_q_sort3前进行打印 .........."); // ❌发现报错处❌
                        }
    // 🔥🔥🔥🔥🔥@@@@@@@@@@@@@@@@@@@@@@@  子标题5：调用mainQSort3进行排序 @@@@@@@@@@@@@@@@@@@@@@@🔥🔥🔥🔥🔥  
                        main_q_sort3(ptr, block, quadrant, nblock, lo as i32, hi as i32, 2, budget);
                        num_q_sorted += (hi as i32) - (lo as i32) + 1;
                        if *budget < 0 {
                            return;
                        }
                    }
                }
                if sb == 63280{
                    // println!("前:{}",ftab[sb as usize]); // 97613❌  97614✅
                 }
                ftab[sb as usize] |= (1 << 21);
                if sb == 63280{
                   // println!("后:{}",ftab[sb as usize]); // 2194765❌ 2194766✅
                //    my_sleep(48);
                }
                if i == 156 {
                    saveBlockArrayToFile(ftab, (nblock+34) as usize, "69_after_t8_ftab_c.txt");
                }
            }
        }   
        // println!("🦀️标题8 ss:{}; i:{}",ss, i);
        // my_sleep(30);
        assert!(!big_done[ss as usize]);
        
        for j in 0..=255 {
            copy_start[j] = ((ftab[((j << 8) + ss as usize) as usize] & (!((1 << 21))) as u64))as i64 as i32;
            // if i == 156 {
                // let array = [
                //     1386, 2778, 2916, 3413, 4013, 4276, 4370, 4858, 4886, 4930, 5065, 5696, 5799, 5972,
                //     5994, 6017, 6042, 6055, 6078, 6093, 6105, 6124, 6177, 6190, 6255, 6276, 6286, 6346,
                //     6515, 6859, 8079, 8087, 8095, 8205, 8252, 8301, 9004, 9015, 9027, 9046, 9129, 9278, 
                //     9352, 9668, 9952, 10513, 10844, 11410, 11452, 11542, 11990, 12117, 12232, 12306, 12339, 
                //     12380, 12430, 12505, 12835, 12964, 12994, 13042, 13086, 13121, 13222, 13230, 13431, 13477, 
                //     13572, 13643, 13676, 14616, 14691, 14853, 14933, 14987, 15024, 15111, 15195, 15287, 15319, 
                //     15412, 15710, 15753, 15893, 16418, 17615, 17629, 17684, 17749, 17771, 17784, 17913, 18020, 
                //     18110, 18123, 18136, 18187, 21498, 22105, 23570, 24920, 29571, 30528, 31315, 32725, 35784, 
                //     35836, 36125, 37824, 39095, 42044, 44889, 45984, 46550, 50195, 52942, 56496, 57548, 57935, 
                //     58480, 58766, 59375, 59410, 59468, 59515, 59528, 59572, 59637, 59647, 59660, 59670, 59728, 
                //     59741, 59749, 59806, 59870, 59883, 59892, 59912, 59922, 59971, 61749, 63494, 65653, 71795, 
                //     72652, 72786, 78427, 78704, 80761, 80776, 80853, 84807, 84988, 86021, 86034, 86047, 86092, 
                //     86237, 86382, 86428, 87042, 87051, 87078, 87148, 87171, 87233, 87250, 87284, 87338, 87447, 
                //     88284, 88300, 88313, 88348, 88386, 88398, 88983, 89247, 89260, 89283, 89734, 89749, 89814,
                //     90995, 91014, 91077, 91147, 91159, 91165, 91334, 91762, 91776, 91898, 91915, 92308, 92380, 
                //     92408, 92661, 93551, 93561, 93574, 93592, 93612, 93664, 93712, 93725, 93789, 93831, 93849, 
                //     93868, 93883, 93900, 94180, 94199, 94248, 94288, 94303, 94323, 94334, 95030, 95042, 95261,
                //     95429, 95438, 95447, 95471, 96828, 96957, 96995, 97009, 97057, 97063, 97065, 97075, 97086, 
                //     97098, 97116, 97184, 97197, 97209, 97442, 97529, 97552, 97561, 97610, 97614, 97642, 97651, 
                //     97659, 97680, 97694, 97733, 97794, 97898
                // ];
            //     if copy_start[j] != array[j] {

            //         // print_65537(&ftab, "🦀️ftab (i==156)");
            //         // println!("CS前内ss部部分: {}", ss);  // 48
            //         // println!("CS前内部部分:: {}", ((j << 8) + ss as usize) as usize);  // 63280
            //         // println!("CS前半部分: {}", ftab[((j << 8) + ss as usize) as usize]); // 2194765
            //         // println!("CS后半部分: {}", !((1 << 21))); //  -2097153
            //         // println!("copy_start[{}] is different from array[{}]. Expected {}, found {}", j, j, array[j], copy_start[j]);
            //         my_sleep(20);  // 睡2秒
            //     } 
            // }
            // 检查 copy_start[j] 是否与 array[j] 一样
            copy_end[j] = (((ftab[((j << 8) + ss as usize + 1) as usize] & (!((1 << 21))) as u64))as i64 - 1) as i32;
        }
        if i == 156 {
            print_255(&copy_start, "🦀️首次copy_start ✅ 和C一致");
            print_255(&copy_end, "🦀️首次copy_end ✅ 和C一致"); //TODO: missmatch
            my_sleep(12);
        }
        // 第一个：🔥copyStart for-loop🔥
        // for j in (ftab[(ss << 8) as usize] & (!((1 << 21))) as u64) as i32..copy_start[ss as usize] {
        let initial_j = (ftab[(ss << 8) as usize] & (!((1 << 21))) as u64) as i32;
        let end_condition = copy_start[ss as usize];
        for j in initial_j..end_condition {
            k = (ptr[j as usize] as i32) - 1;
            if k < 0 {
                k += nblock;
            }
            c1 = block[k as usize];
            if !big_done[c1 as usize] {
                ptr[copy_start[c1 as usize] as usize] = k as u32;
                copy_start[c1 as usize] += 1;
            }
        }
        if i == 156 {
            print_255_copy_i32(&copy_start, "一for后：156-🦀️copy_start ");
            print_255_copy_i32(&copy_end, "一for后：156-🦀️copy_end ");
        }



        
        // C的循环是从初始值递减到copyEnd[ss] + 1。 ❌Rust的循环是从初始值递增到copy_end[ss as usize]
        // 第二个：🔥copyEnd for-loop🔥
        // for j in (copy_end[ss as usize] + 1..=(ftab[((ss + 1) << 8) as usize] & (!((1 << 21)))) as i32 - 1).rev() {
        let start_index = (ftab[((ss + 1) << 8) as usize] & (!((1 << 21)))) as i32 - 1;
        let end_index = copy_end[ss as usize]; //❌+1 ✅没有
        if i == 156 {
            // println!("第二个CS, CE for loop: start_index: {}; end_index: {}, sleep:12", start_index, end_index);       
            my_sleep(12)
        }

        // 正常的如下：
        // for j in (end_index..=start_index).rev() {
        //     k = (ptr[j as usize] as i32) - 1;
        //     if k < 0 {
        //         k += nblock;
        //     }
        //     c1 = block[k as usize];
        //     if !big_done[c1 as usize] {
        //         ptr[copy_end[c1 as usize] as usize] = k as u32;
        //         copy_end[c1 as usize] -= 1;
        //     }
        // }
        // print_255(block, "🦀️block");
        // if i == 156 {
        //     // save_array_to_file(block, "/root/crown-rust/d_156_block_rust.txt");
        //     // println!("Before CE 关键第二forLoop save d_156_block_rust.txt ..........");
        //     // save_array_to_file(ptr, "/root/crown-rust/d_156_ptr_rust.txt");
        //     // println!("Before CE 关键第二forLoop save d_156_ptr_rust.txt ..........");
        //     my_sleep(12);
        // }
        for j in (end_index..=start_index).rev() {
            k = (ptr[j as usize] as i32) - 1;
            if k < 0 {
                k += nblock;
            }
            c1 = block[k as usize];
            if i == 156 {
                // println!("(sleep 1s) c1: {}", c1);
                my_sleep(1);
                if !big_done[c1 as usize] {
                    ptr[copy_end[c1 as usize] as usize] = k as u32;
                    copy_end[c1 as usize] -= 1;
                    // // println!("i == 156: big_done[{} as usize]: {}", c1, big_done[c1 as usize] );                    
                }else {
                    // println!("❌copy_end[{}] {}; ", c1, copy_end[c1 as usize]);   
                }
                // my_sleep(15);
            }else{
                if !big_done[c1 as usize] {
                    ptr[copy_end[c1 as usize] as usize] = k as u32;
                    copy_end[c1 as usize] -= 1;
                }
            }
        }
        // // println!("For first condition:");
        // // println!("Left Side: {}", copy_start[ss as usize] - 1);
        // // println!("Right Side: {}", copy_end[ss as usize]);
    
        // // println!("For second condition:");
        // // println!("Left Side (first part): {}", copy_start[ss as usize]);
        // // println!("Right Side (first part): {}", 0);
        // // println!("Left Side (second part): {}", copy_end[ss as usize]);
        // // println!("Right Side (second part): {}", nblock - 1);
        // print!("3: copy_start: [");
        // print!("3: copy_end: [");
        // println!("标题9: i: {}; ss {}", i, ss);
        if i == 156 {
            print_255_copy_i32(&copy_start, "两for后：156-🦀️copy_start ");
            print_255_copy_i32(&copy_end, "两for后：156-🦀️copy_end ");
        }
        assert!(copy_start[ss as usize] - 1 == copy_end[ss as usize] || (copy_start[ss as usize] == 0 && copy_end[ss as usize] == nblock - 1));

        for j in 0..=255 {
            ftab[((j << 8) + ss) as usize] |= (1 << 21);
        }
        print_63280_i156(ftab,  "标题9: 🦀️：遍历从0到255的整数，将其第21位设置为1: ftab");
        big_done[ss as usize] = true;

        if i < 255 {
            let bb_start = ftab[(ss << 8) as usize] & (!((1 << 21)));
            let mut bb_size = ((ftab[((ss + 1) << 8) as usize] & (!((1 << 21)))) - bb_start) as i64;
            let mut shifts = 0;

            while (bb_size >> shifts) > 65534 {
                shifts += 1;
            }

            for j in (0..bb_size).rev() {
                let a2update = ptr[(bb_start + (j as u64)) as usize] as i32;
                let q_val = (j >> shifts) as u16;
                quadrant[a2update as usize] = q_val;
                if a2update < (2 + 12 + 18 + 2) {
                    quadrant[(a2update + nblock) as usize] = q_val;
                }
            }
            if bb_size == 0 {
                // println!("特别注意：bb_size 的值为 0，导致减法溢出！");
            } else {
                assert!(((bb_size - 1) >> shifts) <= 65535, "错误：断言失败，可能的原因是 ((bb_size - 1) >> shifts) 的值大于 65535");
            }
            
            assert!(((bb_size - 1) >> shifts) <= 65535);
        }
    }

    if verb >= 4 {
        eprintln!("        {} pointers, {} sorted, {} scanned", nblock, num_q_sorted, nblock - num_q_sorted);
    
    }
}


fn read_dat_file(path: &str) -> Result<(Vec<u32>, Vec<u8>, Vec<u16>, Vec<u64>, i32, i32)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut ptr_values: Vec<u32> = Vec::new();
    let mut block_values: Vec<u8> = Vec::new();
    let mut quadrant_values: Vec<u16> = Vec::new();
    let mut ftab_values: Vec<u64> = Vec::new();
    let mut nblock: i32 = 0;
    let mut budget: i32 = 0;

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
            "ftab" => {
                ftab_values.extend(inner[1..inner.len()-1].split(",").filter_map(|s| s.trim().parse::<u64>().ok()));
            },
            "nblock" => nblock = inner.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to parse nblock: {}", e)))?,
            "budget" => budget = inner.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to parse budget: {}", e)))?,
            
            _ => {},
        }
    }
    if ptr_values.is_empty() || block_values.is_empty() || quadrant_values.is_empty() || ftab_values.is_empty() {
        return Err(Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"));
    } else {
        return Ok((ptr_values, block_values, quadrant_values, ftab_values, nblock, budget));
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut ptr, mut block, mut quadrant, mut ftab, nblock, mut budget) = read_dat_file("/root/crown-rust/bzip2/params_mainSort_1.dat")?;
        // println!("Before mainSort:");
        let lo = 0; // 或者其他默认值
        let hi = ptr.len() - 1; // 或者其他默认值

        // println!("nblock={}",nblock);
        // println!("len(block)={}",block.len());
        // println!("len(quadrant)={}",quadrant.len());
        // println!("len(ftab)={}",ftab.len());
        // println!("ptr[{} as usize] = {}", lo, ptr[lo as usize]);
        // println!("ptr[{} as usize] = {}", hi, ptr[hi as usize]);
        // println!("budget {}", budget);
        
        // 调用 main_sort 函数
        main_sort(&mut ptr, &mut block[..], &mut quadrant, &mut ftab, nblock, 0, &mut budget);
        // ... 你可以在这里继续你的断言 ...
        Ok(())
    }
}