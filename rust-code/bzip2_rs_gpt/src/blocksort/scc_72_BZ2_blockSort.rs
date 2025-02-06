use std::fs::File;
use std::io::{self, BufReader, Write, BufRead, Result, Error};
use std::io::prelude::*;
use crate::blocksort::scc_71_fallbackSort::*;
use crate::blocksort::scc_69_mainSort::*;
use std::path::Path;
use std::mem;
use std::fmt::Display;
use crate::global_vars::bzip2_c1::{bz_stream, EState, rebuild_arr1_from_mtfv, rebuild_arr2_from_block, build_mtfv_from_arr1, build_block_from_arr2};
use crate::global_vars::sleep_lib::*;
fn fill_2d_array<T: std::str::FromStr>(arr: &mut [[T; 258]; 6], data: &str) 
where <T as std::str::FromStr>::Err: std::fmt::Debug {
    // 先按换行符分割数据，再处理每一行
    let lines: Vec<&str> = data.split('[').filter(|&line| !line.trim().is_empty()).collect();
    for (i, line) in lines.iter().enumerate().take(6) {
        for (j, num) in line.split(',').filter_map(|s| s.trim().matches(char::is_numeric).next()).enumerate().take(258) {
            arr[i][j] = num.parse().expect("Failed to parse number");
        }
    }
}


fn fill_2d_array_len_pack(arr: &mut [[u32; 4]; 258], data: &str) {
    for (i, line) in data.split_whitespace().enumerate() {
        for (j, num) in line.split(',').enumerate() {
            arr[i][j] = num.parse().expect("Failed to parse number");
        }
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

pub fn block_sort(s: &mut EState) {
    let verb = s.verbosity;
    let nblock = s.nblock;
    // println!("🔍 开始 block_sort, nblock: {}, verbosity: {}", nblock, verb);
    // convert and clone
    let mut arr1 = rebuild_arr1_from_mtfv(&s.mtfv);
    let mut arr1_from_ptr: Vec<u32> = s.ptr.clone();
    assert!(arr1 == arr1_from_ptr, "🚨 rebuild_arr1_from_mtfv assertion failed");

    let mut arr2 = rebuild_arr2_from_block(&s.block);
    if nblock < 10000 {
        // println!("🔄 使用 fallback_sort");
        fallback_sort(&mut arr1, &mut arr2, &mut s.ftab, s.nblock, s.verbosity);
    //    EState 
    //   1. 数组pointer 变成 arr1，确定长度（程序分析）
    //   2. 多个同时指向同一个内存，vec 拷贝方式(synchronzie不同copy的内容); 
    // 保证正确性：对于一般单线程程序（写paper）

        // u32 to u16
        s.mtfv = build_mtfv_from_arr1(&mut arr1);
        s.ptr = arr1.clone(); // same type
        // 两块不同的内存，一个点改，其他相关的点也需要改        
        s.block = build_block_from_arr2(&mut arr2);
    } else {
        let mut i = nblock + (2 + 12 + 18 + 2);
        if i & 1 != 0 {
            i += 1;
        }
        let mut quadrant = vec![0u16; i as usize];

        let mut wfact = s.workFactor.clamp(1, 100);
        let budget_init = nblock * ((wfact - 1) / 3);
        let mut budget = budget_init;

        // println!("🔃 进入 main_sort");
        let old_mtfv = s.mtfv.clone();
        main_sort(&mut s.ptr, &mut s.block, &mut quadrant, &mut s.ftab, s.nblock, s.verbosity, &mut budget);
        s.mtfv = build_mtfv_from_arr1(&s.ptr);
        if s.mtfv == old_mtfv {
            // println!("🐊🐊🐊🐊🐊🐊🐊");
            // my_sleep(120);
        }else{
            // println!("🐸🐸🐸🐸🐸🐸🐸");
            // my_sleep(120);
        }
        // println!("72_after_mainSort: {:?}", s.ptr);
        if verb >= 3 {
            eprintln!("📊 budget_init: {}, budget: {}, nblock: {}", budget_init, budget, nblock);
        }

        if budget < 0 {
            if verb >= 2 {
                eprintln!("❗️ 过于重复，使用 fallback_sort");
            }
            fallback_sort(&mut arr1, &mut arr2, &mut s.ftab, s.nblock, s.verbosity);
            s.mtfv = build_mtfv_from_arr1(&arr1);
            s.block = build_block_from_arr2(&arr2);
        }
    }

    s.origPtr = -1;
    for i in 0..nblock {
        if s.ptr[i as usize] == 0 {
            s.origPtr = i;
            break;
        }
    }

    assert!(s.origPtr != -1, "🚨 BZ2 block sort assertion failed");

    // println!("🏁 block_sort 完成，origPtr: {}", s.origPtr);
    // println!("ptr_final_gmtf🐶: {:?}", s.ptr);
}


fn read_dat_file(path: &str) -> std::result::Result<(bz_stream, EState), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //1,2,3
    let (mut avail_in, mut total_in_hi32, mut total_in_lo32): (u32, u32, u32) = (0, 0, 0);
    let (mut avail_out, mut total_out_lo32, mut total_out_hi32): (u32, u32, u32) = (0, 0, 0);
    let (mut mode, mut state, mut avail_in_expect): (i32, i32, u32) = (0, 0, 0);
    //4
    let (mut arr1_values, mut arr2_values, mut ftab): (Vec<u32>, Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new(), Vec::new());
    let origPtr: i32 = 0;
    //5,6,7,8,9
    let (mut ptr, mut block, mut mtfv, mut zbits): (Vec<u32>, Vec<u8>, Vec<u16>, Vec<u8>) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let (mut workFactor, mut state_in_ch, mut state_in_len, mut rNToGo, mut rTPos): (u32, u32, i32, i32, i32) = (0, 0, 0, 0, 0);
    let (mut nblock, mut nblockMAX, mut numZ, mut state_out_pos, mut nInUse): (i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0);
    let (mut mtfFreq_value, mut selector_value, mut selectorMtf_value): (Vec<i32>, Vec<u8>, Vec<u8>) = (Vec::new(), Vec::new(), Vec::new());
    let (mut inUse, mut unseqToSeq): (Vec<u32>, Vec<u8>) = (Vec::new(), Vec::new());
    //10.
    let (mut bsBuff, mut bsLive, mut blockCRC, mut combinedCRC, mut verbosity,mut  blockNo,mut  blockSize100k, mut nMTF): (u32, i32, u32, u32, i32, i32, i32, i32) = (0, 0, 0, 0, 0, 0, 0, 0);
    //11.
    // let (mut len, mut code, mut rfreq, mut len_pack): (Vec<u8>, Vec<i32>, Vec<i32>, Vec<u32>) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut len: [[u8; 258]; 6] = [[0; 258]; 6];
    let mut code: [[i32; 258]; 6] = [[0; 258]; 6];
    let mut rfreq: [[i32; 258]; 6] = [[0; 258]; 6];
    

    
    for line in reader.lines() {
        let line = line?; // 处理可能的错误
        let inner = line.split(":").nth(1).unwrap_or("").trim();
        match line.split(":").nth(0).unwrap_or("") {
            // 1.
            "state->avail_in" => {
                avail_in = inner.parse::<u32>().unwrap_or(0);
            },
            "state->total_in_hi32" => {
                total_in_hi32 = inner.parse::<u32>().unwrap_or(0);
            },
            "state->total_in_lo32" => {
                total_in_lo32 = inner.parse::<u32>().unwrap_or(0);
            },
            // 2.
            "state->avail_out" => {
                avail_out = inner.parse::<u32>().unwrap_or(0);
            },
            "state->total_out_hi32" => {
                total_out_hi32 = inner.parse::<u32>().unwrap_or(0);
            },
            "state->total_out_lo32" => {
                total_out_lo32 = inner.parse::<u32>().unwrap_or(0);
            },
            // 3.
            "state->mode" => {
                mode = inner.parse::<i32>().unwrap_or(0);
            },
            "state->state" => {
                state = inner.parse::<i32>().unwrap_or(0);
            },
            "state->avail_in_expect" => {
                avail_in_expect = inner.parse::<u32>().unwrap_or(0);
            },
            // 4.
            "state->arr1_values" => {
                arr1_values = inner.split(",").filter_map(|s| s.trim().parse::<u32>().ok()).collect();
            },
            "state->arr2_values" => {
                arr2_values = inner.split(",").filter_map(|s| s.trim().parse::<u32>().ok()).collect();
            },
            "state->ftab" => {
                ftab = inner.split(",").filter_map(|s| s.trim().parse::<u32>().ok()).collect();
            },
            // 5.
            "state->ptr" => {
                ptr = inner.split(",").filter_map(|s| s.trim().parse::<u32>().ok()).collect();
            },
            "state->block" => {
                block = inner.split(",").filter_map(|s| s.trim().parse::<u8>().ok()).collect();
            },
            "state->mtfv" => {
                mtfv = inner.split(",").filter_map(|s| s.trim().parse::<u16>().ok()).collect();
            },
            "state->zbits" => {
                zbits = inner.split(",").filter_map(|s| s.trim().parse::<u8>().ok()).collect();
            },
            // 6.
            "state->workFactor" => {
                workFactor = inner.parse::<u32>().unwrap_or(0);
            },
            "state->state_in_ch" => {
                workFactor = inner.parse::<u32>().unwrap_or(0);
            },
            "state->state_in_len" => {
                workFactor = inner.parse::<u32>().unwrap_or(0);
            },
            "state->rNToGo" => {
                rNToGo = inner.parse::<i32>().unwrap_or(0);
            },
            "state->rTPos" => {
                rTPos = inner.parse::<i32>().unwrap_or(0);
            },
            // 7.
            "state->nblock" => {
                nblock = inner.parse::<i32>().unwrap_or(0);
            },
            "state->nblockMAX" => {
                nblockMAX = inner.parse::<i32>().unwrap_or(0);
            },
            "state->numZ" => {
                numZ = inner.parse::<i32>().unwrap_or(0);
            },
            "state->state_out_pos" => {
                state_out_pos = inner.parse::<i32>().unwrap_or(0);
            },
            "state->nInUse" => {
                nInUse = inner.parse::<i32>().unwrap_or(0);
            },
            // 8.
            "state->mtfFreq" => {
                mtfFreq_value = inner.split(",").filter_map(|s| s.trim().parse::<i32>().ok()).collect();
            },
            "state->selector" => {
                selector_value = inner.split(",").filter_map(|s| s.trim().parse::<u8>().ok()).collect();
            },
            "state->selectorMtf" => {
                selectorMtf_value = inner.split(",").filter_map(|s| s.trim().parse::<u8>().ok()).collect();
            },

            

            // 9.
            "state->inUse" => {
                inUse.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u32::from_str_radix(s.trim(), 10).ok()));
            },
            "state->unseqToSeq" => {
                unseqToSeq.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u8::from_str_radix(s.trim(), 10).ok()));
            },
            // 10.
            "state->bsBuff" => {
                bsBuff = inner.parse::<u32>().unwrap_or(0);
            },
            "state->bsLive" => {
                bsLive = inner.parse::<i32>().unwrap_or(0);
            },
            "state->blockCRC" => {
                blockCRC = inner.parse::<u32>().unwrap_or(0);
            },
            "state->combinedCRC" => {
                combinedCRC = inner.parse::<u32>().unwrap_or(0);
            },
            "state->verbosity" => {
                verbosity = inner.parse::<i32>().unwrap_or(0);
            },
            "state->blockNo" => {
                blockNo = inner.parse::<i32>().unwrap_or(0);
            },
            "state->blockSize100k" => {
                blockSize100k = inner.parse::<i32>().unwrap_or(0);
            },
            "state->nMTF" => {
                nMTF = inner.parse::<i32>().unwrap_or(0);
            },
            
            // 11.
            // let (mut len,mut  code, mut rfreq, mut len_pack): (u8, i32, i32, u32) = (0, 0, 0, 0);
            "state->len" => {
                fill_2d_array(&mut len, inner);
            },
            "state->code" => {
                fill_2d_array(&mut code, inner);
            },
            "state->rfreq" => {
                fill_2d_array(&mut rfreq, inner);
            },
            // "state->len_pack" => {
            //     len_pack.extend(inner[1..inner.len()-1].split(",").filter_map(|s| u32::from_str_radix(s.trim(), 10).ok()));
            // },
            _ => {},
        }       
    }
    let mut e_state = EState::default();
     // 创建 bz_stream 结构体实例
     let mut bz_strm = bz_stream {
        next_in: std::ptr::null_mut(),
        avail_in,
        total_in_lo32,
        total_in_hi32,

        next_out: std::ptr::null_mut(),
        avail_out,
        total_out_lo32,
        total_out_hi32,

        state: &mut e_state as *mut EState,  // 指向 e_state

        bzalloc: None,
        bzfree: None,
        opaque: std::ptr::null_mut(),
    };

    let mut mtf_freq_array: [i32; 258] = [0; 258];
    for (i, &value) in mtfFreq_value.iter().enumerate().take(258) {
        mtf_freq_array[i] = value;
    }

    let mut selector_array: [u8; 18002] = [0; 18002];
    for (i, &value) in selector_value.iter().enumerate().take(18002) {
        selector_array[i] = value;
    }

    let mut selector_mtf_array: [u8; 18002] = [0; 18002];
    for (i, &value) in selectorMtf_value.iter().enumerate().take(18002) {
        selector_mtf_array[i] = value;
    }

    // 最后四个值
    // 转换 len 为 [[u8; 258]; 6]
    // let mut len_array: [[u8; 258]; 6] = [[0; 258]; 6];
    // for (i, chunk) in len.chunks(258).enumerate().take(6) {
    //     for (j, &value) in chunk.iter().enumerate().take(258) {
    //         len_array[i][j] = value;
    //     }
    // }
    // // 转换 code 为 [[i32; 258]; 6]
    // let mut code_array: [[i32; 258]; 6] = [[0; 258]; 6];
    // for (i, chunk) in code.chunks(258).enumerate().take(6) {
    //     for (j, &value) in chunk.iter().enumerate().take(258) {
    //         code_array[i][j] = value;
    //     }
    // }
    // // 转换 rfreq 为 [[i32; 258]; 6]
    // let mut rfreq_array: [[i32; 258]; 6] = [[0; 258]; 6];
    // for (i, chunk) in rfreq.chunks(258).enumerate().take(6) {
    //     for (j, &value) in chunk.iter().enumerate().take(258) {
    //         rfreq_array[i][j] = value;
    //     }
    // }
    // // 转换 len_pack 为 [[u32; 4]; 258]
    // let mut len_pack_array: [[u32; 4]; 258] = [[0; 4]; 258];
    // for (i, chunk) in len_pack.chunks(4).enumerate().take(258) {
    //     for (j, &value) in chunk.iter().enumerate().take(4) {
    //         len_pack_array[i][j] = value;
    //     }
    // }
    // 现在可以完全初始化 EState 实例
    e_state = EState {
        strm: Some(&mut bz_strm as *mut bz_stream),
        mode,
        state,
        avail_in_expect,

        // arr1: Some(arr1_values.as_ptr() as *mut u32),
        // arr2: Some(arr2_values.as_ptr() as *mut u32),
        // ftab: Some(ftab.as_ptr() as *mut u32),
        // arr1:arr1_values,
        // arr2:arr2_values,
        ftab,
        
        origPtr,

        ptr,
        block,
        mtfv,
        // ptr: Some(ptr.as_ptr() as *mut u32),
        // block: Some(block.as_ptr() as *mut u8),
        // mtfv: Some(mtfv.as_ptr() as *mut u16),
        // zbits: Some(zbits.as_ptr() as *mut u8),
        zbits: zbits.clone(),

        workFactor: workFactor as i32,
        state_in_ch,
        state_in_len,
        rNToGo,
        rTPos,

        nblock,
        nblockMAX,
        numZ,
        state_out_pos,
        nInUse,

        inUse: [false; 256], // 可以根据需要更新
        unseqToSeq: [0; 256], // 可以根据需要更新

        bsBuff,
        bsLive,
        blockCRC,
        combinedCRC,
        verbosity,
        blockNo,
        blockSize100k,
        nMTF,

        mtfFreq: mtf_freq_array,
        selector: selector_array,
        selectorMtf: selector_mtf_array,
        len,
        code,
        rfreq,
        len_pack: [[0; 4]; 258]
        // len: [[0; 258]; 6], // 可以根据需要更新
        // code: [[0; 258]; 6], // 可以根据需要更新
        // rfreq: [[0; 258]; 6], // 可以根据需要更新

        // len_pack: [[0; 4]; 258], // 可以根据需要更新
    };

    // 返回初始化的结构体
    Ok((bz_strm, e_state))

    // if inUse.is_empty() || unseqToSeq.is_empty() {
    //     Err(Error::new(std::io::ErrorKind::InvalidData, "数据行数不足"))
    // } else {

    //     // println!("inUse{:?}",inUse);
    //     Ok((inUse, unseqToSeq))
    // }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::global_vars::bzip2_c1::EState;
    #[test]
    fn test_read_dat_file() -> Result<()> {
        let (mut bzStream, mut estate) =  read_dat_file("/root/rustmap/bzip2/72_before_blockSort_EState_c.txt")?;
        block_sort(&mut estate);
        // // println!("loSt{}",loSt);
        // // println!("hiSt{}",hiSt);
        // Add your assertions here
        Ok(())
    }
}
