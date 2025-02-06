use crate::global_vars::bzip2_c1::*;
use crate::global_vars::sleep_lib::*;
pub fn make_maps_e(s: &mut EState) {
    s.nInUse = 0;
    for i in 0..256 {
        if s.inUse[i as usize] {
            s.unseqToSeq[i as usize] = s.nInUse as UChar;
            s.nInUse += 1;
        }
    }
}

#[test]
fn test_make_maps_e() {
    let mut s = EState::default();
    // 初始化：让偶数的值设为true
    for i in 0..256 {
        s.inUse[i] = i % 2 == 0;
    }

    make_maps_e(&mut s);

    // 验证 nInUse 的值应为128
    assert_eq!(s.nInUse, 128);
    for i in 0..256 {
        if i % 2 == 0 {
            assert_eq!(usize::from(s.unseqToSeq[i]), i / 2 as usize);
        }
    }
}

fn convert_to_vec(ptr_option: Option<*mut u32>, num_elements: usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(num_elements);
    if let Some(ptr) = ptr_option {
        unsafe {
            // 从 ptr 指向的内存地址复制 num_elements 个 u32 元素到 vec 中
            vec.extend_from_slice(std::slice::from_raw_parts(ptr, num_elements));
        }
    }
    vec
}


pub fn generate_mtf_values(s: &mut EState) {
    // println!("开始 generate_mtf_values 函数");

    let mut yy = [0u8; 256];
    let mut z_pend:i32;
    let mut j: i32;
    let mut wr:i32;
    let EOB:i32; 

    // println!("调用 make_maps_e");
    make_maps_e(s);
    EOB = s.nInUse + 1;

    for i in 0..=EOB {
        s.mtfFreq[i as usize] = 0;
    }
    wr = 0;
    z_pend = 0;
    // println!("初始化 yy 数组, s.nInUse: {}", s.nInUse);
    for i in 0..s.nInUse as usize {
        yy[i] = i as u8;
    }
    // println!("yy success");

    // let ptr = &s.ptr;
    // // println!("尝试解引用 block");
    // let block = &s.block;


    // println!("处理主循环");
    for i in 0..s.nblock as usize {
        let ll_i: u8;
        j = (s.ptr[i] as isize - 1) as i32;
        // println!("计算后的 j 值: {}", j);
        if j < 0 {
            j += s.nblock as i32;
            // println!("调整后的 j 值: {}", j);
        }
        ll_i = s.unseqToSeq[s.block[j as usize] as usize];
        if yy[0] == ll_i {
            z_pend += 1;
        } else {
            if z_pend > 0 {
                z_pend -= 1;
                while z_pend >= 0 {
                    let value = if z_pend & 1 != 0 { 1 } else { 0 };
                    if wr < s.mtfv.len() as i32{
                        s.mtfv[wr as usize] = value as u16;
                        s.ptr = rebuild_arr1_from_mtfv(&s.mtfv);
                    } else {
                        // println!("错误：索引超出 mtfv 的范围。wr = {}, mtfv 长度 = {}", wr, s.mtfv.len());
                    }
                    wr += 1;
                    s.mtfFreq[value as usize] += 1;

                    if z_pend < 2 {
                        break;
                    }
                    z_pend = (z_pend - 2) / 2;
                }
                z_pend = 0;
            }
            {
                let mut tempValue: u8 = yy[1];
                yy[1] = yy[0];
                let mut currentPointer: usize = 1;
                let targetValue: u8 = ll_i;
            
                while targetValue != tempValue {
                    let mut tempSwap: u8;
                    currentPointer += 1;
                    tempSwap = tempValue;
                    tempValue = yy[currentPointer];
                    yy[currentPointer] = tempSwap;
                }
                
                yy[0] = tempValue;
                j = currentPointer as i32;  // 在 Rust 中索引直接用 usize 表示
                s.mtfv[wr as usize] = (j + 1) as u16;
                s.ptr = rebuild_arr1_from_mtfv(&s.mtfv);
                wr += 1;
                s.mtfFreq[(j+1) as usize] += 1;
            
                // println!("mtfv[0]: {}", s.mtfv[0]);
                // println!("上述代码：将最近访问的元素移至数组的前端，来优化数据的存取效率");
                // let mut rtmp:u8 = yy[1];
                // yy[1] = yy[0];
            
                // let mut ryy_j_index: u8 = 1; // 使用索引代替指针
                // let rll_i = ll_i;
            
                // while rll_i != rtmp {
                //     // if ryy_j_index + 1 >= yy.len() {
                //     //     break; // 防止数组越界
                //     // }
                //     let rtmp2 = rtmp;
                //     ryy_j_index += 1;
                //     rtmp = yy[ryy_j_index as usize];
                //     yy[ryy_j_index as usize] = rtmp2;
                // }
            
                // yy[0] = rtmp;
                // // ❌❌
                // j = (ryy_j_index - 1) as i32; // 通过索引计算 j 的值
                // s.mtfv[wr as usize] = (j + 1) as u16;
                // s.ptr = rebuild_arr1_from_mtfv(&s.mtfv);
                // wr += 1;
                // s.mtfFreq[(j + 1) as usize] += 1;

                // if wr < s.mtfv.len() as i32 && (j + 1) < s.mtfFreq.len() {
                  
                    // let mut temp_value = yy[1];
                    // yy[1] = yy[0];
                    // let mut current_index = 1usize; // 使用 usize 类型，因为它是数组索引
                    // let target_value = ll_i;
                
                    // while target_value != temp_value {
                    //     current_index += 1;
                    //     let temp_swap = temp_value;
                    //     temp_value = yy[current_index];
                    //     yy[current_index] = temp_swap;
                    // }
                
                    // yy[0] = temp_value;
                    // let j = current_index as i32; // 将 usize 类型转换为 i32
                    // mtfv[*wr as usize] = (j + 1) as usize; // 将 i32 类型转换为 usize
                    // *wr += 1;
                    // s.mtf_freq[(j + 1) as usize] += 1; // 同样，进行类型转换
                
            }
        }
    }

    if z_pend > 0 {
        z_pend -= 1;
        while true {
            let value = if z_pend & 1 != 0 { 1 } else { 0 };
            if wr < s.mtfv.len() as i32{
                s.mtfv[wr as usize] = value as u16;
                s.ptr = rebuild_arr1_from_mtfv(&s.mtfv);
            } else {
                // println!("错误：索引超出 mtfv 的范围。wr = {}, mtfv 长度 = {}", wr, s.mtfv.len());
            }
            wr += 1;
            s.mtfFreq[value as usize] += 1;

            if z_pend < 2 {
                break;
            }
            z_pend = (z_pend - 2) / 2;
        }
    }

    // ❌发现
      s.mtfv[wr as usize] = EOB as u16;
      s.ptr = rebuild_arr1_from_mtfv(&s.mtfv);
      wr += 1;
    // 检查 EOB 是否在 mtfFreq 的范围内
    if EOB < s.mtfFreq.len() as i32{
        s.mtfFreq[EOB as usize] += 1;
    } else {
        // println!("Error: EOB is out of range for mtfFreq");
    }
    s.nMTF = wr as i32;
}







        


        







        








    



