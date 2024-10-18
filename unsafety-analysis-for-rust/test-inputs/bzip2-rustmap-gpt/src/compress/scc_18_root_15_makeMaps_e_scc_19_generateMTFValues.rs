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



// // 判断出ptr的长度应该是nblock + 34
// pub fn generate_mtf_values(s: &mut EState) {
//     // println!("开始 generate_mtf_values 函数");

//     let mut yy = [0u8; 256];
//     let mut z_pend = 0;
//     let mut wr = 0;
//     let eob = s.nInUse + 1;

//     // println!("调用 make_maps_e");
//     make_maps_e(s);
//     // let mut ptr_after_mmaps = unsafe { Vec::from_raw_parts(s.ptr.unwrap(), (s.nblock + 34) as usize, s.nblock as usize) };
//     // // println!("ptr_after_mmaps: {:?}",ptr_after_mmaps);
//     for i in 0..=eob as usize {
//         s.mtfFreq[i] = 0;
//     }

//     // println!("初始化 yy 数组, s.nInUse: {}", s.nInUse);
//     for i in 0..s.nInUse as usize {
//         yy[i] = i as u8;
//     }

//     // println!("yy success");
//     // // println!(",s.ptr: {}",s.ptr);
//     //TODO: first time ptr
//     // let ptr_vec = convert_to_vec(s.ptr, (100000 * s.blockSize100k) as usize);
//     // let mut ptr_vec = unsafe { Vec::from_raw_parts(s.ptr.unwrap(), (s.nblock + 34) as usize, s.nblock as usize) };
//     let mut ptr_vec = &s.ptr;
//     // println!("26_ptr: {:?}",ptr_vec);
//     // // println!("ptr_vec[0]: {}",ptr_vec[0]);
//     // my_sleep(1000);
//     // println!("处理主循环");
//     for i in 0..s.nblock as usize {
//         // println!("循环迭代: {}", i);
//         // println!("尝试解引用 ptr");
        
//         // let ptr = unsafe { 
//         //     match s.ptr.as_ref() {
//         //         Some(ptr_ref) => ptr_ref,
//         //         None => {
//         //             // println!("错误：ptr 是空指针");
//         //             return;
//         //         }
//         //     }
//         // };
//         // 💡💡💡
//         let ptr = &s.ptr;
//         // println!("尝试解引用 block");
//         // let block = unsafe { 
//         //     match s.block.as_ref() {
//         //         Some(block_ref) => block_ref,
//         //         None => {
//         //             // println!("错误：block 是空指针");
//         //             return;
//         //         }
//         //     }
//         // };
//         // 💡💡💡
//         let block = &s.block;
//         // 在🐦中j为97735
//         // 在这里添加检查，确保 ptr.add(i) 不会越界
//         // // println!("尝试获取 j 值: {}", ptr[j]); // ❌


//         // 用一个临时变量来存储 ptr.add(i) 的结果
//         // println!("尝试解引用 ptr");
        
//         let mut j = ptr[i] as isize - 1;



//         // let j_val = unsafe { ptr.add(i).as_ref() };
//         // match j_val {
//         //     Some(val) => // println!("获取到的 j 值: {}", val),
//         //     None => // println!("错误：尝试访问 ptr 时越界或指向无效内存"),
//         // }
//         // // println!("⚡️⛰️⚡️⛰️");


//         // if j_val.is_none() {
//         //     // println!("错误：尝试访问 ptr 时越界或指向无效内存");
//         //     return;
//         // }
//         // let j_val = j_val.unwrap();
//         // // println!("获取到的 j 值: {}", j_val);

//         // println!("计算后的 j 值: {}", j);

//         if j < 0 {
//             // println!("j 小于 0，进行调整");
//             j += s.nblock as isize;
//             // println!("调整后的 j 值: {}", j);
//         }
//         // println!("尝试获取 ll_i 值");
//         let ll_i = if j < (block.len()) as isize {
//             s.unseqToSeq[block[j as usize] as usize]
//         } else {
//             // 处理索引超出 Vec 长度的情况
//             // println!("错误：索引超出 block 的范围。j = {}, block 长度 = {}", j, block.len());
//             // 返回一个默认值或错误处理
//             0 // 或者其他适当的值
//         };
        
//         // println!("ll_i 值: {}", ll_i);


//         if yy[0] == ll_i {
//             z_pend += 1;
//             // println!("增加 z_pend，当前 z_pend = {}", z_pend);
//         } else {
//             if z_pend > 0 {
//                 // println!("处理 z_pend 大于 0 的情况");
//                 z_pend -= 1;
//                 while z_pend >= 0 {
//                     let value = if z_pend & 1 != 0 { 1 } else { 0 };
//                     let mtfv = &s.mtfv;
//                     // 确保 wr 是一个有效的索引
//                     if wr < mtfv.len() {
//                         mtfv[wr] = value as u16;
//                     } else {
//                         // 索引超出 Vec 的范围
//                         // println!("错误：索引超出 mtfv 的范围。wr = {}, mtfv 长度 = {}", wr, mtfv.len());
//                         // 在这里添加额外的错误处理逻辑
//                     }
//                     wr += 1;
//                     s.mtfFreq[value as usize] += 1;

//                     if z_pend < 2 {
//                         break;
//                     }
//                     z_pend = (z_pend - 2) / 2;
//                 }
//                 z_pend = 0;
//             }

//             // println!("处理 yy 数组更新");
//             let mut ryy_j = 1;
//             let mut rtmp = yy[1];
//             yy[1] = yy[0];
//             while rtmp != ll_i {
//                 ryy_j += 1;
//                 let rtmp2 = rtmp;
//                 rtmp = yy[ryy_j];
//                 yy[ryy_j] = rtmp2;
//             }
//             yy[0] = rtmp;
//             let mut mtfv = &s.mtfv;
//             // 确保 wr 是一个有效的索引 💡💡💡
//             if wr < mtfv.len() {
//                 mtfv[wr] = (ryy_j + 1) as u16;
//             } else {
//                 // 如果 wr 超出 mtfv 的长度，则需要处理这种情况
//                 // 比如可以打印错误消息或进行其他错误处理
//                 // println!("26_错误：索引超出 mtfv 的范围。wr = {}, mtfv 长度 = {}", wr, mtfv.len());
//                 // 这里可以添加错误处理代码
//             }

//             wr += 1;
//             s.mtfFreq[ryy_j + 1] += 1;
//         }
//     }

//     if z_pend > 0 {
//         // println!("处理剩余的 z_pend");
//         z_pend -= 1;
//         while z_pend >= 0 {
//             let value = if z_pend & 1 != 0 { 1 } else { 0 };
//             let mut mtfv = &s.mtfv;
//             // 确保 wr 是有效的索引 💡💡💡
//             if wr < mtfv.len() {
//                 mtfv[wr] = value as u16;
//             } else {
//                 // 处理索引超出 Vec 长度的情况
//                 // println!("错误：索引超出 mtfv 的范围。wr = {}, mtfv 长度 = {}", wr, mtfv.len());
//                 // 可以添加更多的错误处理代码
//             }
//             wr += 1;
//             s.mtfFreq[value as usize] += 1;

//             if z_pend < 2 {
//                 break;
//             }
//             z_pend = (z_pend - 2) / 2;
//         }
//     }

//     // let mtfv = unsafe { s.mtfv.as_mut().unwrap() };
//     let mut mtfv = &s.mtfv;
//     // unsafe {
//     //     *mtfv.add(wr) = eob as u16;
//     // }
//     if let Some(index) = wr.checked_sub(1) {
//         if index < mtfv.len() {
//             mtfv[index] = eob as u16;
//         } else {
//             // println!("索引超出范围: wr = {}, index = {}, Vec 长度 = {}", wr, index, mtfv.len());
//             // 可以添加更多的错误处理代码
//         }
//     } else {
//         // println!("计算索引时发生下溢: wr = {}", wr);
//         // 可以添加更多的错误处理代码
//     }
    
//     wr += 1;
//     s.mtfFreq[eob as usize] += 1;

//     s.nMTF = wr as i32;

//     // println!("完成 generate_mtf_values 函数");
// }

