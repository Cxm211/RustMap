use libc::VLNEXT;
use std::env;
use crate::global_vars::bzlib_private::*;
use crate::global_vars::constants::*;

use crate::global_vars::bzip2_c1::*;
use crate::decompress::scc_5_root_4_makeMaps_d::*;
use crate::huffman::scc_4_root_3_BZ2_hbCreateDecodeTables::*;
use crate::bzlib::scc_62_BZ2_bz__AssertH__fail::*;
use crate::bzlib::scc_6_root_5_BZ2_indexIntoF::*;




// GET_1st_MTF_VAL_slice需要生命周期注释来确保其修改的Option<&mut [i32]>类型参数与DState实例的生命周期相匹配，防止悬垂引用。
// 确保你从s中借用的所有切片在函数返回之前都是有效的
// fn GET_1st_MTF_VAL_slice<'a>(
//     s: &'a mut DState, 
//     groupNo: i32, 
//     groupPos: &mut i32, 
//     gSel: &mut i32, 
//     gMinlen: &mut i32, 
//     gLimit_slice_ref: &mut Option<&'a mut [i32]>, 
//     gPerm_slice_ref: &mut Option<&'a mut [i32]>, 
//     gBase_slice_ref: &mut Option<&'a mut [i32]>, 
//     nSelectors: i32
// ) {
//     *groupPos = BZ_G_SIZE;
//     *gSel = s.selector[groupNo as usize] as i32;
//     *gMinlen = s.minLens[*gSel as usize];
//     if *gSel < (s.limit.len()) as i32 {
//         *gLimit_slice_ref = Some(&mut s.limit[*gSel as usize]);
//     }
//     if *gSel < (s.perm.len()) as i32 {
//         *gPerm_slice_ref = Some(&mut s.perm[*gSel as usize]);
//     }
//     if *gSel < (s.base.len()) as i32 {
//         *gBase_slice_ref = Some(&mut s.base[*gSel as usize]);
//     }
// }
use std::fs::File;
use std::io::prelude::*;

fn write_to_file_seqToUnseq(seqToUnseq: &[u8; 256]) {
    let mut file = match File::create("seqToUnseq.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("无法创建文件: {}", e);
            return;
        },
    };

    for &value in seqToUnseq {
        if let Err(e) = writeln!(file, "{}", value) {
            eprintln!("写入文件时出错: {}", e);
            return;
        }
    }

    println!("数据成功写入文件。");
}


fn BZ_INITIALISE_CRC(crc: &mut u32){
    *crc = 0xffffffff;
}

                 


fn GET_LL4(i_usize: usize, s: &DState) -> Option<u32> {
    // 首先检查ll4是否有值
    if let Some(ref ll4_vec) = s.ll4 {
        // 如果ll4是Some，我们可以安全地进行索引操作
        // 注意：这里假设索引操作不会越界，你可能需要添加额外的检查来确保索引安全
        let value = ll4_vec[i_usize >> 1] >> (((i_usize << 2) & 0x4) & 0xF);
        Some(value as u32)
    } else {
        // 如果ll4是None，这里可以根据你的需要处理
        // 例如，返回None表示没有值
        None
    }
}
fn GET_LL(i_usize: usize, s: &DState)  -> Option<u32> {
    if let Some(ref get_ll4) = GET_LL4(i_usize, s){
        if let Some(ref ll16_vec) = s.ll16 {
            Some((ll16_vec[i_usize] as u32) | get_ll4)
        } else{
            println!("GET_LL: ll16 is None");
            None
        }
    }
    else{
        println!("GET_LL: ll4 is None");
        None
    }
}     


fn SET_LL4(i_usize: usize, n: i32, s: &mut DState) {
    if let Some(ref mut ll4_vec) = s.ll4 {
        if i_usize & 0x1 == 0 {
             ll4_vec[i_usize >> 1] = (ll4_vec[(i_usize) >> 1] & 0xf0) | (n as u8) 
        } else {
            ll4_vec[i_usize >> 1] = (ll4_vec[i_usize >> 1] & 0x0f) | ( (n << 4)  as u8);
        }
    } else{
        println!("SET_LL4: ll4 is None");
    }

}

fn SET_LL_part(i_usize: usize, uc: u8, s: &mut DState) {
    if let Some(ref mut ll16_vec) = s.ll16 {
        ll16_vec[i_usize] = (s.cftabCopy[uc as usize] & 0x0000ffff) as u16;
    } else{
        println!("SET_LL: ll16 is None");
    }
}   
fn SET_LL(i_usize: usize, n: i32, s: &mut DState) {
    if let Some(ref mut ll16_vec) = s.ll16 {
        ll16_vec[i_usize] = (n & 0x0000ffff) as u16;
    } else{
        println!("SET_LL: ll16 is None");
    }
}   
fn BZ_GET_SMALL_1(s: &mut DState) -> Result<(), ()> {
    /* c_tPos is unsigned, hence test < 0 is pointless. */ 
    if s.tPos >= (100000 * s.blockSize100k as u32){  Ok(()) }
    else { Err(()) }
}    
fn BZ_GET_SMALL_2(s: &mut DState) {
    s.k0 = BZ2_indexIntoF(s.tPos as i32, & s.cftab) as i32;
    if let Some( get_ll_ref) =  GET_LL(s.tPos as usize, s){
        s.k0 = s.k0 + get_ll_ref as i32;
    } else{
        println!("BZ_GET_SMALL_2: ll is None");
    }
}                    

fn GET_1st_MTF_VALIDX(
    s: & mut DState, 
    groupNo: i32, 
    groupPos: &mut i32, 
    gSel: &mut i32, 
    gMinlen: &mut i32, 
    gLimit_IDX: &mut usize, 
    gPerm_IDX: &mut usize, 
    gBase_IDX: &mut usize, 
    nSelectors: i32
) {
    *groupPos = BZ_G_SIZE;
    *gSel = s.selector[groupNo as usize] as i32;
    *gMinlen = s.minLens[*gSel as usize];
    if *gSel < (s.limit.len()) as i32 {
        *gLimit_IDX = *gSel as usize;
    }
    if *gSel < (s.perm.len()) as i32 {
        *gPerm_IDX = *gSel as usize;
    }
    if *gSel < (s.base.len()) as i32 {
        *gBase_IDX = *gSel as usize;
    }
}
fn GET_2nd_MTF_VAL(groupPos: &mut i32, zn: &mut i32, gMinlen: i32){
    *groupPos -= 1;
    *zn = gMinlen;
}
// fn GET_3rd_MTF_VAL_slice_pureR(zvec: isize, gLimit_slice: Option<&mut [i32]>, zn_isize: isize) -> Result<(), ()> {
//   if (zvec <= gLimit[zn]) break;              
fn GET_3rd_MTF_VAL_pureRIDX(zvec: i32, gLimit_IDX: usize, zn_isize: isize, s: &DState)  -> Result<(), ()> {
    let gPerm_slice = &s.limit[gLimit_IDX][..];
    println!("MTF_3: zvec: {:?}; zn_isize: {:?}", zvec, zn_isize);
    if zvec <= gPerm_slice[zn_isize as usize] {
        Ok(())
    } else {
        Err(())
    }
} 
// zn++;    \
fn GET_4th_MTF_VAL(zn: &mut i32){
    *zn += 1;
}
// zvec = (zvec << 1) | zj;
fn GET_5th_MTF_VAL(zvec: &mut i32, zj: i32){
    println!("MTF-5: zvec: {:?}; zj: {:?}", zvec, zj);
    *zvec = (*zvec << 1) | zj
}
fn GET_6th_MTF_VAL_IFIDX(zvec: isize, gBase_IDX: usize, zn_isize: isize, s: &DState) -> bool {
    // 直接访问数组的行，然后使用[..]获取切片
    let gBase_slice = &s.base[gBase_IDX][..];
    (zvec - gBase_slice[zn_isize as usize] as isize) < 0 || 
    (zvec - gBase_slice[zn_isize as usize] as isize) >= BZ_MAX_ALPHA_SIZE as isize
}

 // lval = gPerm[zvec - gBase[zn]]
fn GET_7th_MTF_VAL_IDX(lval: &mut i32, gPerm_IDX: usize, zvec: isize, gBase_IDX: usize, zn_isize: isize, s: &DState) {
    let gBase_slice = &s.base[gBase_IDX][..];
    let gPerm_slice = &s.perm[gPerm_IDX][..];
    println!("MTF_7: zvec: {:?}; zn_isize: {:?}", zvec, zn_isize);
    *lval = gPerm_slice[(zvec - gBase_slice[zn_isize as usize] as isize) as usize];
}



// 不在宏内的任何循环中
// if (zvec - gBase[zn] < 0 || zvec - gBase[zn] >= BZ_MAX_ALPHA_SIZE) 
// fn GET_6th_MTF_VAL_IFIDX(zvec: isize, gBase_pointer: *mut i32, zn_isize: isize) -> bool{
//     zvec - unsafe { *gBase_pointer.offset(zn_isize) as isize } < 0 || zvec - unsafe { *gBase_pointer.offset(zn_isize) as isize } >= BZ_MAX_ALPHA_SIZE as isize
// }
// // lval = gPerm[zvec - gBase[zn]];
// fn GET_7th_MTF_VAL(lval: &mut i32, gPerm_pointer: *mut i32, zvec: isize, gBase_pointer: *mut i32, zn_isize: isize){
//     *lval = unsafe { *gPerm_pointer.offset(zvec - unsafe { *gBase_pointer.offset(zn_isize) as isize}) };
// }









fn GET_1st_single_BIT_half(s: &mut DState, uuu: &mut u32) -> Result<(), ()> {
    GET_BITS_first_half(s, uuu, 1)
}
fn GET_1st_UCHAR_half(s: &mut DState, uuu: &mut u32) -> Result<(), ()> {
    GET_BITS_first_half(s, uuu, 8)
}



fn GET_2nd_single_BIT_half(s: &mut DState) {
    GET_BITS_second_half(s);
}
fn GET_2nd_UCHAR_half(s: &mut DState){
    GET_BITS_second_half(s);
}



#[macro_export]
macro_rules! SVprintln {
    ($($arg:tt)*) => {{
        if std::env::var("RUST_APP_PRINT").is_ok() {
            println!($($arg)*);
        }
    }}
}
// 修改第一二项
fn GET_BITS_first_half(s: &mut DState, vvv: &mut u32, nnn: i32) -> Result<(), ()> {
    if s.bsLive >= nnn {
        SVprintln!("[if内]s->bsBuff: {:?}", s.bsBuff);  
        SVprintln!("[if内]s->bsLive: {:?}", s.bsLive); 
        SVprintln!("[if内]nnn: {:?}", nnn); 
        let v: u32 = (s.bsBuff >> (s.bsLive - nnn)) & ((1 << nnn) - 1);
        s.bsLive -= nnn;
        *vvv = v;
        if env::var("RUST_APP_PRINT").is_ok() {  println!("vvv: {:?}", vvv);   }
        Ok(()) // 成功时返回Ok
    } else {
        SVprintln!("s.bsLive: {:?}", s.bsLive);
        SVprintln!("nnn: {:?}", nnn);
        Err(()) // 条件不满足时返回Err
    }
}
// 无法直接替代GET_BITS的原因是因为goto save_state_and_return
fn GET_BITS_second_half(s: &mut DState) {
    unsafe {
        s.bsBuff = (s.bsBuff << 8) | *((*s.strmD).next_in_pointer) as u32;
        s.bsLive += 8;
        (*s.strmD).next_in_pointer = (*s.strmD).next_in_pointer.add(1);
        SVprintln!("[if外](*s.strmD).avail_in: {:?}", (*s.strmD).avail_in); 
        (*s.strmD).avail_in -= 1;
        SVprintln!("[if外](*s.strmD).avail_in: {:?}", (*s.strmD).avail_in); 
        (*s.strmD).total_in_lo32 += 1;
        if (*s.strmD).total_in_lo32 == 0 {
            (*s.strmD).total_in_hi32 += 1;
        }
    }
}





fn save_state_and_return (
    s: &mut DState,
    i: i32,
    j: i32,
    t: i32,
    alphaSize: i32,
    nGroups: i32,
    nSelectors: i32,
    EOB: i32,
    groupNo: i32,
    groupPos: i32,
    nextSym: i32,
    nblockMAX: i32,
    nblock: i32,
    es: i32,
    N: i32,
    curr: i32,
    zt: i32,
    zn: i32,
    zvec: i32,
    zj: i32,
    gSel: i32,
    gMinlen: i32,
    gLimit_idx: usize,
    gBase_idx: usize,
    gPerm_idx: usize,
) {
    s.save_i = i;
    s.save_j = j;
    s.save_t = t;
    s.save_alphaSize = alphaSize;
    s.save_nGroups = nGroups;
    s.save_nSelectors = nSelectors;
    s.save_EOB = EOB;
    s.save_groupNo = groupNo;
    s.save_groupPos = groupPos;
    s.save_nextSym = nextSym;
    s.save_nblockMAX = nblockMAX;
    s.save_nblock = nblock;
    s.save_es = es;
    s.save_N = N;
    s.save_curr = curr;
    s.save_zt = zt;
    s.save_zn = zn;
    s.save_zvec = zvec;
    s.save_zj = zj;
    s.save_gSel = gSel;
    s.save_gMinlen = gMinlen;
    s.save_gLimit_idx = gLimit_idx;
    s.save_gBase_idx = gBase_idx;
    s.save_gPerm_idx = gPerm_idx;
}



fn u8_to_bool(value: u8) -> bool {
    match value {
        0 => false,
        1 => true,
        _ => panic!("Invalid value: {}", value), // 或者可以返回 false 或 true，取决于你的需求
    }
}





pub fn maomi(s: &mut DState) -> i32{
    serializeDState(s, &format!("maomi.rs.txt"));       
    serializeDState(s, &format!("maomi1.rs.txt")); 


    println!("maomi");
    1
}





pub fn BZ2_decompress(s: &mut DState) -> i32 {
    println!("save_nextSym: {:?}", s.save_nextSym);
    println!("BZ2_decompress-1");
    unsafe { serialize_bz_streamD(&*s.strmD, &format!("59_enterSTRMD.rs.txt"));  }
    serializeDState(&s, &format!("59_enter.rs.txt"));       
    serializeDState(&s, &format!("59_enter1.rs.txt")); 
    println!("BZ2_decompress-2");

    let mut endhdr_2_state: bool = false;
    let mut uc: u8 = 0;
    let mut retVal: i32 = 0;
    let mut minLen: i32;
    let mut maxLen: i32;
    let strmD: &mut bz_stream_D = unsafe { &mut *s.strmD }; // 解引用为可变借用
    let (mut i, mut j, mut t, mut alphaSize, mut nGroups, mut nSelectors, mut EOB, mut groupNo, mut groupPos, mut nextSym, mut nblockMAX, mut nblock, mut es, mut N, mut curr, mut zt, mut zn, mut zvec, mut zj, mut gSel, mut gMinlen): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    let (mut gLimit_pointer, mut gBase_pointer, mut gPerm_pointer): (*mut i32, *mut i32, *mut i32) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
    let (mut gLimit_slice, mut gBase_slice, mut gPerm_slice): (Option<&mut [i32]>, Option<&mut [i32]>, Option<&mut [i32]>) = (None, None, None);
    let (mut gLimit_idx, mut gBase_idx, mut gPerm_idx): (usize, usize, usize) = (0, 0, 0);

// 霍夫曼解码过程中 长度都应nGroups * alphaSize
// gLimit: 最大长度       // gBase:  基值计算     // gPerm:  排列
    let (mut g_limit, mut g_base, mut g_perm): (Option<Vec<i32>>, Option<Vec<i32>>, Option<Vec<i32>>) = (None, None, None);
    if s.state == 10 as i32 {
        s.save_i = 0;         s.save_j = 0;         s.save_t = 0;        s.save_alphaSize = 0;        s.save_nGroups = 0;        s.save_nSelectors = 0;        s.save_EOB = 0;        s.save_groupNo = 0;        s.save_groupPos = 0;        s.save_nextSym = 0;        s.save_nblockMAX = 0;        s.save_nblock = 0;        s.save_es = 0;        s.save_N = 0;        s.save_curr = 0;        s.save_zt = 0;        s.save_zn = 0;        s.save_zvec = 0;        s.save_zj = 0;        s.save_gSel = 0;        s.save_gMinlen = 0;       
        s.save_gLimit = None;       s.save_gBase = None;        s.save_gPerm = None;
    }

    i = s.save_i;
    j = s.save_j;
    t = s.save_t;
    alphaSize = s.save_alphaSize;
    nGroups = s.save_nGroups;
    nSelectors = s.save_nSelectors;
    EOB = s.save_EOB;
    groupNo = s.save_groupNo;
    groupPos = s.save_groupPos;
    nextSym = s.save_nextSym;
    nblockMAX = s.save_nblockMAX;
    nblock = s.save_nblock;
    es = s.save_es;
    N = s.save_N;
    curr = s.save_curr;
    zt = s.save_zt;
    zn = s.save_zn;
    zvec = s.save_zvec;
    zj = s.save_zj;
    gSel = s.save_gSel;
    gMinlen = s.save_gMinlen;
    gLimit_idx = s.save_gLimit_idx;
    gBase_idx = s.save_gBase_idx;
    gPerm_idx = s.save_gPerm_idx;
    // gLimit_pointer = s.save_gLimit_pointer;
    // gBase_pointer = s.save_gBase_pointer;
    // gPerm_pointer = s.save_gPerm_pointer;

    retVal = CONSTS.BZ_OK;
    'state_loop: while s.state <= 50 {
        match s.state {
            10 => {
                #[cfg(debug_assertions)]
                {   println!("位点: {:?}", 10); }
                serializeDState(&s, &format!("10_before.rs.txt"));       
                serializeDState(&s, &format!("10_before1.rs.txt"));  
                
                s.state = 10; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);          }
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x42 {   retVal = CONSTS.BZ_DATA_ERROR_MAGIC;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); } 
                serializeDState(&s, &format!("10_after.rs.txt"));       
                serializeDState(&s, &format!("10_after1.rs.txt"));               
                s.state = 11;
            },  
            11 => {
                #[cfg(debug_assertions)]
                {   println!("位点: {:?}", 11); }
                s.state = 11; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){ 
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); }
                        GET_2nd_UCHAR_half(s);
                    }
                }
                if uc != 0x5a {   retVal = CONSTS.BZ_DATA_ERROR_MAGIC;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  } 
                unsafe  { serialize_combined(&s, &*s.strmD, "11"); }
                s.state = 12;
            },    
            12 => {
                #[cfg(debug_assertions)]
                {   println!("位点: {:?}", 12); }
                s.state = 12; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);}
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x68 {  retVal = CONSTS.BZ_DATA_ERROR_MAGIC;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); } 
                unsafe  { serialize_combined(&s, &*s.strmD, "12"); }
                s.state = 13;
            },    
            13 => {
                #[cfg(debug_assertions)]
                {   println!("位点: {:?}", 13); }
                s.state = 13; loop {
                    let mut tmp_blockSize100k =  s.blockSize100k as u32;
                    match GET_BITS_first_half(s, &mut tmp_blockSize100k, 8){     
                        Ok(_) => {  s.blockSize100k = tmp_blockSize100k as i32; break;  }
                        Err(_) => { s.blockSize100k = tmp_blockSize100k as i32; }, // 错误时继续循环
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);     }
                    GET_BITS_second_half(s);  
                }
                println!("s.blockSize100k: {:?}", s.blockSize100k);
                env::remove_var("RUST_APP_PRINT");
                if s.blockSize100k < (0x30 + 1) || s.blockSize100k > (0x30 + 9) {    retVal = CONSTS.BZ_DATA_ERROR_MAGIC;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); }
                // let BZ_HDR_0: = 0x30 as i32;
                s.blockSize100k -= 0x30; // BZ_HDR_0
            // 🌟🌟🌟🌟 不同于gBase, gPerm, gLimit 🌟🌟🌟🌟 ll16这些我们不用vec
        // sizeof(UInt16): 2;           sizeof(UChar): 1;           sizeof(Int32): 4
                if s.smallDecompress {
                    s.ll16 = Some(vec![0; (s.blockSize100k * 100000 * 2) as usize]);
                    s.ll4 = Some(vec![0; (1 + s.blockSize100k * 100_000 >> 1) as usize]);

                    // s.ll16 = Some(Vec::with_capacity((s.blockSize100k * 100000 * 2) as usize));
                    // s.ll4 = Some(Vec::with_capacity((((1 + s.blockSize100k * 100_000) >> 1) / 1) as usize));            
                    if s.ll16.is_none() || s.tt.is_none() {  retVal = CONSTS.BZ_MEM_ERROR;   save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  }
                } else {
                    // s.tt = Some(Vec::with_capacity((s.blockSize100k * 100000 * 4) as usize));
                    s.tt = Some(vec![0; (s.blockSize100k * 100000 * 4) as usize]);
                    if s.tt == None {   retVal = CONSTS.BZ_MEM_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym,  nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen,  gLimit_idx, gBase_idx, gPerm_idx);  }
                }
                unsafe  { serialize_combined(&s, &*s.strmD, "13"); }
                s.state = 14;
            },
            14 => {
                s.state = 14; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);}
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 14); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
                if uc == 0x17 {
                    s.state = 42;
                    endhdr_2_state = true;
                    continue;
                }
                // 0x31 在十进制中等于 3 * 16^1 + 1 * 16^0，这等于 48 + 1，合计是 49。
                if uc != 0x31 {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; } 
                unsafe  { serialize_combined(&s, &*s.strmD, "14"); }
                s.state = 15;
            },       
            15 => {
                s.state = 15; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 15); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
                if uc != 0x41 {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);   } 
                unsafe  { serialize_combined(&s, &*s.strmD, "15"); }
                s.state = 16;
            },  
            16 => {
                s.state = 16; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);return retVal;}
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 16); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }

                if uc != 0x59 { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                serializeDState(&s, "16_Dstate.rs.txt");
                unsafe  { serialize_bz_streamD(&*s.strmD, "16_strm.rs.txt"); }
                s.state = 17;
            }, 
            17 => {
                s.state = 17; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);return retVal; }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 17); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
    
                if uc != 0x26 {  retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;         }                
                serializeDState(&s, "17_Dstate.rs.txt");
                unsafe  { serialize_bz_streamD(&*s.strmD, "17_strm.rs.txt"); }
                s.state = 18;
            }, 
            18 => {
                s.state = 18; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 18); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }

                if uc != 0x53 { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; } 
                serializeDState(&s, "18_Dstate.rs.txt");
                unsafe  { serialize_bz_streamD(&*s.strmD, "18_strm.rs.txt"); }
                s.state = 19;
            },     
            19 => {
                s.state = 19; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 19); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
                if uc != 0x59 {  retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;    } 
                s.currBlockNo+=1;
                if s.verbosity >= 2 {
                    eprintln!("\n    [{}: huff+mtf ", s.currBlockNo);
                }
                s.storedBlockCRC = 0;
                serializeDState(&s, "19_Dstate.rs.txt");
                unsafe  { serialize_bz_streamD(&*s.strmD, "19_strm.rs.txt"); }
                s.state = 20;
            }, 
            20 => {
                s.state = 20; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;  }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 20); }
                        // case 20
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                serializeDState(&s, "20_Dstate.rs.txt");
                unsafe  { serialize_bz_streamD(&*s.strmD, "20_strm.rs.txt"); }
                s.state = 21;
            },
            21 => {
                s.state = 21; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 21); }
                        GET_2nd_UCHAR_half(s);
                    }
                }
                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                unsafe {    serialize_combined(&s, &*s.strmD, "21"); }
                s.state = 22;
            }, 
            22 => {
                s.state = 22; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;   }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 22); }
                        GET_2nd_UCHAR_half(s);
                    }
                }          
                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                unsafe {    serialize_combined(&s, &*s.strmD, "22"); }
                s.state = 23;
            }, 
            23 => {
                s.state = 23; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }

                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 23); }
                        
                        GET_2nd_UCHAR_half(s);
                    }
                }
                s.storedBlockCRC = (s.storedBlockCRC << 8) | uc as u32;
                unsafe {    serialize_combined(&s, &*s.strmD, "23"); }
                s.state = 24;
            }, 
            24 => {
                s.state = 24; loop {
                    let mut tmp_uc_32 = uc as u32; 
                    match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                    GET_2nd_single_BIT_half(s); 
                }
                unsafe {    serialize_combined(&s, &*s.strmD, "24"); }
                s.origPtr = 0;
                s.state = 25;
            }, 
            25 => {
                s.state = 25; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);        return retVal;    }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 25); }
                        GET_2nd_UCHAR_half(s);
                    }
                }
                unsafe {    serialize_combined(&s, &*s.strmD, "25"); }
                s.origPtr = (s.origPtr << 8) | uc as i32;
                s.state = 26;
            }, 
            26 => {
                s.state = 26; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    unsafe {
                        if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                        #[cfg(debug_assertions)]
                        {   println!("位点: {:?}", 26); }
                        GET_2nd_UCHAR_half(s);
                    }
                }
                s.origPtr = (s.origPtr << 8) | uc as i32;
                s.state = 27;
            }, 
            27 => {
                #[cfg(debug_assertions)] println!("位点: {:?}", 27); 
                s.state = 27; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8;  break; }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {  retVal = CONSTS.BZ_OK;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                    
                    GET_2nd_UCHAR_half(s);
                }
                s.origPtr = (s.origPtr << 8) | uc as i32;
                if s.origPtr < 0 {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                if s.origPtr > 10 + 100000 * s.blockSize100k { retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;  }

                /*--- Receive the mapping table ---*/
                for i in 0..16 { 
                    s.state = 28; loop {
                        let mut tmp_uc_32 = uc as u32; 
                        match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                            Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                            Err(_) => { uc = tmp_uc_32 as u8; }, 
                        } 
                        if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                         
                        GET_2nd_single_BIT_half(s);              
                    }
                    s.inUse16[i as usize] = uc == 1;   
                }
                for i in 0..256 {  s.inUse[i as usize] = false; }
                for i in 0..16 { 
                    for j in 0..16 {
                        if s.inUse16[i] {
                            s.state = 29; loop {
                                let mut tmp_uc_32 = uc as u32; 
                                match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                                    Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                                    Err(_) => { uc = tmp_uc_32 as u8; }, 
                                } 
                                if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                 
                                GET_2nd_single_BIT_half(s);
                            }
                        }
                        if uc == 1 {    s.inUse[i * 16 + j as usize] = true;    }
                    }
                    make_maps_d(s);   
                    if s.nInUse == 0 { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;     }
                    alphaSize = s.nInUse + 2;
                }
                s.state = 30;
            },
            30 => {
                #[cfg(debug_assertions)]
                {   println!("位点: {:?}", 30); }
                s.state = 30; loop {
                    let mut tmp_nGroups_32 = nGroups as u32; 
                    match GET_BITS_first_half(s, &mut tmp_nGroups_32, 3) {
                        Ok(_) => { nGroups = tmp_nGroups_32 as i32; break;}, // 成功时退出循环
                        Err(_) => { nGroups = tmp_nGroups_32 as i32; }, // 错误时继续循环
                    }
                    if unsafe { (*s.strmD).avail_in  == 0 } {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                    GET_BITS_second_half(s);
                }
                if nGroups < 2 || nGroups > 6 {  retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;                 }
                unsafe  { serialize_combined(&s, &*s.strmD, "30"); }
                s.state = 31;
            },
            31 => { 
                serializeDState(&s, &format!("31_DState_{}_before.rs.txt", t));
                unsafe { serialize_bz_streamD(&*s.strmD, &format!("31_bz_streamD_{}_before.rs.txt", t)); }
                if endhdr_2_state == false {
                    s.state = 31; loop {
                        let mut tmp_nSelectors = nSelectors as u32;
                        match GET_BITS_first_half(s, &mut tmp_nSelectors, 15){
                            Ok(_) => { nSelectors = tmp_nSelectors as i32; break;}, // 成功时退出循环
                            Err(_) => { nSelectors = tmp_nSelectors as i32;}, // 错误时继续循环
                        }
                        if unsafe { (*s.strmD).avail_in } == 0  { retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                        GET_BITS_second_half(s); 
                    }
                    if nGroups < 2 || nGroups > 6 {   retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;   }
                    for i in 0..nSelectors {
                        #[cfg(debug_assertions)] 
                        {
                            println!("位点🔥: {:?}", 32); println!("nSelectors: {:?}", nSelectors);
                            println!("nGroups: {:?}",nGroups);  
                        }
                        s.state = 32; loop {
                            let mut tmp_uc_32 = uc as u32; 
                            match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                                Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                                Err(_) => { uc = tmp_uc_32 as u8; }, 
                            } 
                            if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                            GET_2nd_single_BIT_half(s);
                        }
                        if uc == 0 {  break;  }
                        j += 1;
                        if j >= nGroups {
                            retVal = CONSTS.BZ_DATA_ERROR;
                            save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                            return retVal;
                        }
                        if i < (2 + (900000 / 50)){
                            println!("🐍i: {:?}", i);
                            println!("🐍j: {:?}", j);
                            s.selectorMtf[i as usize] = j as u8;
                        }
                    }   
                    if nSelectors > (2 + (900000 / 50)){    nSelectors = 2 + (900000 / 50);      }

                    //🔥重点观察 selectorMtf
                    serializeDState(&s, "beforeUndo.c.txt");

                    //  此处s.selectorMtf出现问题
                    /*--- Undo the MTF values for the selectors. ---*/
                    let mut pos = [0u8; 6]; // 使用固定大小的数组
                    let mut tmp: u8;
                    let mut v: u8;
                    for v in 0..nGroups as u8 {  pos[v as usize] = v;  }
                    for i in 0..nSelectors {
                        v = s.selectorMtf[i as usize] as u8;
                        tmp = pos[v as usize];
                        while v > 0 {
                            pos[v as usize] = pos[(v - 1) as usize];
                            v -= 1;
                        }
                        pos[0] = tmp;
                        s.selector[i as usize] = tmp;
                    }
                  
                    
                    println!("33nGroups前: {:?}", curr);
                        
                    /*--- Now the coding tables ---*/
                    let mut filename_33_34_35: String = "".to_string();
                    for t in 0..nGroups {
                        s.state = 33; loop {                
                            let mut tmp_curr_32 = curr as u32; 
                            match GET_BITS_first_half(s, &mut tmp_curr_32, 5){     
                                Ok(_) => {  curr = tmp_curr_32 as i32; break;  }
                                Err(_) => { curr = tmp_curr_32 as i32; }, 
                            } 
                            if unsafe { (*s.strmD).avail_in == 0 } {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                            GET_BITS_second_half(s);
                        }
                        if t == 1 ||  t == 0 { 
                            serializeDState(&s, &format!("33_DState_{}_after.rs.txt", t));  
                            unsafe { serialize_bz_streamD(&*s.strmD, & format!("33_bz_streamD_{}_after.rs.txt", t)); } 
                        } 
                        if  t == 1 ||  t == 0  {  env::remove_var("RUST_APP_PRINT"); }
                        println!("i33curr: {:?}", curr);
                        // 三重循环serialize🔥🔥🔥
                        for i in 0..alphaSize {                         
                            #[cfg(debug_assertions)] println!("位点🔥: {:?}", 34);
                            let mut while_counter_34_35 = 0;
                            loop {  
                                serializeDState(&s, &format!("34_DState_{}_{}_{}_before.rs.txt", t, i, while_counter_34_35));  
                                unsafe { serialize_bz_streamD(&*s.strmD, & format!("34_bz_streamD_{}_{}_{}_before.rs.txt", t, i, while_counter_34_35)); }                                 
                                s.state = 34;  loop {
                                    if curr < 1 || curr > 20 { 
                                        retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; 
                                    }    
                                    let mut tmp_uc_32 = uc as u32; 
                                    match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                                    } 
                                    if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                    GET_2nd_single_BIT_half(s);   
                                }
                                serializeDState(&s, &format!("34_DState_{}_{}_{}_after.rs.txt", t, i, while_counter_34_35));  
                                unsafe { serialize_bz_streamD(&*s.strmD, & format!("34_bz_streamD_{}_{}_{}_after.rs.txt", t, i, while_counter_34_35)); } 

                                if uc == 0 {  break;  }  
                                
                                #[cfg(debug_assertions)] println!("位点: {:?}", 35);              
                                s.state = 35;  loop {
                                    let mut tmp_uc_32 = uc as u32; 
                                    match GET_1st_single_BIT_half(s, &mut tmp_uc_32){     
                                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                                    } 
                                    if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                    GET_2nd_single_BIT_half(s);
                                }
                                serializeDState(&s, &format!("35_DState_{}_{}_{}_after.rs.txt", t, i, while_counter_34_35));  
                                unsafe { serialize_bz_streamD(&*s.strmD, & format!("35_bz_streamD_{}_{}_{}_after.rs.txt", t, i, while_counter_34_35)); } 

                                curr += if uc == 0 { 1 } else { -1 };
                                while_counter_34_35 += 1;
                            }
                            s.len[t as usize][i as usize] = curr as u8;
                        }
                        serializeDState(&s, &format!("after_alphaSize_forLoop_DState_{}_after.rs.txt", t));
                        unsafe { serialize_bz_streamD(&*s.strmD, &format!("after_alphaSize_forLoop_bz_streamD_{}_after.rs.txt", t)); }
                    }
                    

                    println!("Before nGroups uc: {:?}", uc);
                    /*--- Create the Huffman decoding tables ---*/
                    println!("DecodeTable前 group: {:?}", nGroups);
                    for t in 0..nGroups {
                        minLen = 32;
                        maxLen = 0;
                        for i in 0..alphaSize {
                            if s.len[t as usize][i as usize] > maxLen as u8 { maxLen = s.len[t as usize][i as usize]  as i32; }
                            if s.len[t as usize][i as usize] < minLen as u8 { minLen = s.len[t as usize][i as usize] as i32; }
                        }
                        println!("DecodeTable前 maxLen, minLen: {:?}, {:?}", maxLen, minLen);

                        // scc_59_BZ2_decompress.rs(2341, 41): expected an array with a fixed size of 23 elements, found one with 258 elements
                        // 为什么使用指针而不是单个值：
                        // 函数需要处理的不仅仅是一个单独的 unsigned char 值，
                        // 而是一个序列（例如，一行中的所有值），那么使用指针是有意义的。这样，函数就可以通过指针访问和操作整个序列。
                        // 使用指针还允许函数修改原始数据结构中的值。如果只传递一个值（例如 unsigned char len），
                        // 则函数只能接收并操作那个值的副本，而不是原始数据。
                        BZ2_hbCreateDecodeTables_slice(
                            &mut s.limit[t as usize],
                            &mut s.base[t as usize],
                            &mut s.perm[t as usize],
                            &mut s.len[t as usize],
                            minLen,
                            maxLen,
                            alphaSize as i32,
                        );               
                        s.minLens[t as usize] = minLen;
                        serializeDStateParitial(s, &format!("after_hbCreate_{}.rs.txt", t.to_string()));
                    } //✅
                    /*--- Now the MTF values ---*/
                    EOB = s.nInUse + 1;
                    nblockMAX = 100000 * s.blockSize100k;
                    groupNo = -1;
                    groupPos = 0;
                    for i in 0..=255 {  s.unzftab[i as usize] = 0;  }
                    /*-- MTF init --*/

                    let (mut ii, mut jj, mut kk): (i32, i32, i32);
                    kk = 4096 - 1; // MTFA_SIZE
                    for ii in (0..=256 / MTFL_SIZE - 1).rev() {
                        for jj in (0..=16 - 1).rev() {
                            s.mtfa[kk as usize] = (ii * 16 + jj) as u8;
                            kk -= 1;
                        }
                        s.mtfbase[ii as usize] = kk + 1;
                    }

                    serializeDState(s, "59_dstate.rs.txt");
                    unsafe { serialize_bz_streamD(&*s.strmD, "59_bzstream.rs.txt");}

                    /*-- end MTF init --*/
                    nblock = 0; // 进入第一个GET_MTF_VAL
                    if groupPos == 0 {
                        groupNo += 1;
                        if groupNo >= nSelectors {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                        GET_1st_MTF_VALIDX(s, groupNo, &mut groupPos, &mut gSel, &mut gMinlen, &mut gLimit_idx, &mut gPerm_idx, &mut gBase_idx,  nSelectors);
                    }
                    GET_2nd_MTF_VAL(&mut groupPos, &mut zn, gMinlen);    
                    // 2.14🔥🔥🔥
                    // #define GET_BITS(lll,vvv,nnn)
                    // MTF第一个 GET_BITS(label1, zvec, zn);
                    // 36,37 GET_MTF_VAL中的第1个GET_BITS
                    #[cfg(debug_assertions)] println!("位点: {:?}", 36);    
                    s.state = 36; loop {
                        let mut tmp_zvec_32 = zvec as u32; 
                        match GET_BITS_first_half(s, &mut tmp_zvec_32, zn){   Ok(_) => {  zvec = tmp_zvec_32 as i32; break;  }   Err(_) => { zvec = tmp_zvec_32 as i32; },   }                     
                        if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                        GET_BITS_second_half(s);
                    } 

                
                    while true {  
                        if zn > 20 { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                        match GET_3rd_MTF_VAL_pureRIDX(zvec, gLimit_idx, zn as isize, &s){  Ok(_) => {  break;  }  Err(_) => { }, } 
                        GET_4th_MTF_VAL(&mut zn);
                        #[cfg(debug_assertions)] println!("位点: {:?}", 37);    
                        // 🔥MTF第二个 GET_BIT(label2, zj);
                        s.state = 37;  loop {
                            let mut tmp_uc_32 = uc as u32; 
                            match GET_1st_single_BIT_half(s, &mut tmp_uc_32){   Ok(_) => {  uc = tmp_uc_32 as u8; break;  }Err(_) => { uc = tmp_uc_32 as u8; },   } 
                            if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                            GET_2nd_single_BIT_half(s);
                        }
                        GET_5th_MTF_VAL(&mut zvec, zn);
                    }
                    // 不在宏内的任何循环中
                    if GET_6th_MTF_VAL_IFIDX(zvec as isize, gBase_idx, zn as isize, &s) {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; }
                    GET_7th_MTF_VAL_IDX(&mut nextSym, gPerm_idx, zvec as isize, gBase_idx, zn as isize, &s);     
                   
                   
                    // 超大while (True) 对应两个continue 
                    let mut counter_38 = 0;
                    while true {  
                        if nextSym == EOB   { break; }
                        if nextSym == BZ_RUNA || nextSym == BZ_RUNB {
                            es = -1;
                            N = 1;
                            // do-while-loop-start{ }
                            loop { // * 检查 N 不会变得太大，以便 es 不会变为负数。 
                                // 可以进行 RUNA/RUNB 编码的最大值等于块大小（初始 RLE 之后），即 900k，
                                // 因此将 N 限制为 200 万应该防止溢出，而不拒绝任何合法输入。 */
                                if N >= 2 * 1024 * 1024 {   retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;    }
                                if nextSym == BZ_RUNA {   es += N;   } 
                                else if nextSym == BZ_RUNB {   es += 2 * N; }
                                N *= 2;
                                if groupPos == 0 { 
                                    groupNo += 1; 
                                    if groupNo >= nSelectors {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                                    GET_1st_MTF_VALIDX(s, groupNo, &mut groupPos, &mut gSel, &mut gMinlen, &mut gLimit_idx, &mut gPerm_idx, &mut gBase_idx,  nSelectors);
                                }  GET_2nd_MTF_VAL(&mut groupPos, &mut zn, gMinlen);  
                                // 38,39 MTF_VAL中的第1个GET_BITS
                                #[cfg(debug_assertions)] println!("位点: {:?}", 38); 
                                counter_38+=1;
                                s.state = 38; loop {
                                    if counter_38 == 49{      println!("counter_38: {:?}", counter_38);  }
                                    let mut tmp_zvec_32 = zvec as u32;
                                    match GET_BITS_first_half(s, &mut tmp_zvec_32, zn){   Ok(_) => {  zvec = tmp_zvec_32 as i32; break;  }   Err(_) => { zvec = tmp_zvec_32 as i32; },   }                     
                                    if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                    GET_2nd_single_BIT_half(s);
                                }
                                // 38,39 MTF_VAL中的第2个GET_BITS:39    while (1) { 
                                loop { 
                                    if zn > 20 {  retVal = 4; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                                    match GET_3rd_MTF_VAL_pureRIDX(zvec, gLimit_idx, zn as isize, &s){  Ok(_) => {  break;  }  Err(_) => { }, } 
                                    GET_4th_MTF_VAL(&mut zn);
                                    #[cfg(debug_assertions)] println!("位点: {:?}", 39);
                                    s.state = 39; loop {
                                        let mut tmp_uc_32 = uc as u32; 
                                        match GET_1st_single_BIT_half(s, &mut tmp_uc_32){   Ok(_) => {  uc = tmp_uc_32 as u8; break;  }  Err(_) => { uc = tmp_uc_32 as u8; }, } 
                                        if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                        GET_2nd_single_BIT_half(s);
                                    }
                                    GET_5th_MTF_VAL(&mut zvec, zn);
                                } 
                                // 不在宏内的任何循环中
                                if GET_6th_MTF_VAL_IFIDX(zvec as isize, gBase_idx, zn as isize, &s) {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; }
                                GET_7th_MTF_VAL_IDX(&mut nextSym, gPerm_idx, zvec as isize, gBase_idx, zn as isize, &s);     
                                if  nextSym == BZ_RUNA as i32 || nextSym == BZ_RUNB as i32 {   break;   }                                    //TODO: do-while-loop的while条件 while (nextSym == BZ_RUNA || nextSym == BZ_RUNB);     
                            } // do-while-loop-结束位点{ }

                            es += 1;
                            serializeDStateParitial(s, "uc-seqTo.rs.txt");
                            uc = s.seqToUnseq[s.mtfa[s.mtfbase[0] as usize] as usize];
                            s.unzftab[uc as usize] += es;

                            if s.smallDecompress {  
                                if let Some(ll16_ref) = &mut s.ll16{
                                    while es > 0 {
                                        if nblock >= nblockMAX { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;   }
                                        ll16_ref[nblock as usize] = uc as u16;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            } else {
                                if let Some(tt_ref) = &mut s.tt{
                                    while es > 0 {
                                        if nblock >= nblockMAX { retVal = CONSTS.BZ_DATA_ERROR; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;   }
                                        tt_ref[nblock as usize] = uc as u32;
                                        nblock += 1;
                                        es -= 1;
                                    }
                                }
                            } 
                            continue; // 对应的是37后的大loop while (True) {
                        } //上面忽略🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊🌊👆
                        else {  
                            if nblock >= nblockMAX {   retVal = CONSTS.BZ_DATA_ERROR; (s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;    }
                            /*-- uc = MTF ( nextSym-1 ) --*/
                            let mut nn: u32 = (nextSym - 1) as u32;
                            println!("最后else里面的nn: {:?}", nn);   
                            if nn < CONSTS.MTFL_SIZE as u32 {
                                let mut pp = s.mtfbase[0];
                                println!("pp: {:?}; nn:{:?}", pp, nn);
                                serializeDStateParitial(s, "s_mtfa_pp_nn.rs.txt");
                                uc = s.mtfa[(pp+ nn as i32) as usize];
                                while nn > 3 {
                                    let z = (pp + nn as i32) as usize;
                                    s.mtfa[z] = s.mtfa[z - 1]; // ❌❌
                                    s.mtfa[z - 1] = s.mtfa[z - 2];
                                    s.mtfa[z - 2] = s.mtfa[z - 3];
                                    s.mtfa[z - 3] = s.mtfa[z - 4];
                                    nn -= 4;
                                }
                                while nn > 0 {
                                    s.mtfa[(pp + nn as i32) as usize] = s.mtfa[( pp + nn as i32 - 1) as usize];
                                    nn -= 1;
                                }
                                s.mtfa[pp as usize] = uc;
                                serializeDStateParitial(s, "nn_MTFL_SIZE.rs.txt"); 
                            } else { // 忽略忽略忽略忽略忽略忽略忽略忽略忽略
                                /* general case */
                                let mut lno = nn / CONSTS.MTFL_SIZE as u32;
                                let off = nn % CONSTS.MTFL_SIZE as u32;
                                println!("lno: {:?} off={:?}", lno, off);
                                serializeDStateParitial(s, "lno_nn.rs.txt");
                                let mut pp: i32 = s.mtfbase[lno as usize] + off as i32;
                                serializeDStateParitial(s, "pp_else_mtfbase.rs.txt");
                                uc = s.mtfa[pp as usize];
                                while pp > s.mtfbase[lno as usize] {
                                    s.mtfa[pp as usize] = s.mtfa[(pp - 1) as usize]; pp -= 1;
                                }
                                s.mtfbase[lno as usize] += 1;
                                while lno > 0 {
                                    s.mtfbase[lno as usize] -= 1;
                                    s.mtfa[s.mtfbase[lno as usize] as usize] = s.mtfa[(s.mtfbase[lno as usize - 1] + CONSTS.MTFL_SIZE as i32 - 1) as usize];
                                    lno -= 1;
                                }
                                s.mtfbase[0 as usize] -= 1;
                                s.mtfa[s.mtfbase[0 as usize] as usize] = uc;
                                if s.mtfbase[0 as usize] == 0 {
                                    let mut kk = CONSTS.MTFA_SIZE - 1;
                                    for ii in (0..=CONSTS.MTFL_SIZE / 16 - 1).rev() {
                                        for jj in (0..=CONSTS.MTFL_SIZE - 1).rev() {
                                            s.mtfa[kk as usize] = s.mtfa[(s.mtfbase[ii as usize] + jj as i32) as usize];
                                            kk -= 1;
                                        }
                                        s.mtfbase[ii as usize] = kk as i32 + 1;
                                    }
                                }
                            }
                            serializeDStateParitial(s, "首elsett_nblock前.rs.txt"); 
                            /*-- end uc = MTF ( nextSym-1 ) --*/
                            s.unzftab[s.seqToUnseq[uc as usize] as usize] += 1;
                         //ll16 和 ll4 是 s.ll16 和 s.ll4 中的 Vec<i32> 的可变引用 // 此时uc =1
                
                            if s.smallDecompress {   
                                if let Some(ll16_ref) = &mut s.ll16{
                                    ll16_ref[nblock as usize] = s.seqToUnseq[uc as usize] as u16;   
                                } 
                            }else { 
                                if let Some(tt_ref) = &mut s.tt {
                                    println!("🔥🐱🔥🐱🔥tt不一致点s.seqToUnseq[{:?} as usize]: {:?}", uc, s.seqToUnseq[uc as usize]);
                                    tt_ref[nblock as usize] = s.seqToUnseq[uc as usize] as u32; 
                                }
                            }
                            serializeDStateParitial(s, "首else结束.rs.txt"); 
                            nblock += 1;
                            println!("BZ_X_MTF_5, BZ_X_MTF_6\n");
                            if groupPos == 0 {
                                groupNo += 1;
                                if groupNo >= nSelectors {   retVal = CONSTS.BZ_DATA_ERROR;   save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);    return retVal; }
                                GET_1st_MTF_VALIDX(s, groupNo, &mut groupPos, &mut gSel, &mut gMinlen, &mut gLimit_idx, &mut gPerm_idx, &mut gBase_idx,  nSelectors);
                            }
                            GET_2nd_MTF_VAL(&mut groupPos, &mut zn, gMinlen);
                        // 40,41 MTF_VAL中的第1个GET_BITS
                            s.state = 40; loop {
                                let mut tmp_zvec_32 = zvec as u32;
                                match GET_BITS_first_half(s, &mut tmp_zvec_32, zn){   Ok(_) => {  zvec = tmp_zvec_32 as i32; break;  }   Err(_) => { zvec = tmp_zvec_32 as i32; },   }                       
                                if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                GET_2nd_single_BIT_half(s);
                            }

                        // 40,41 MTF_VAL中的第2个GET_BITS    while (1) {         
                            loop {
                                if zn > 20 { retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;   }
                                match GET_3rd_MTF_VAL_pureRIDX(zvec, gLimit_idx, zn as isize, &s){  Ok(_) => {  break;  }  Err(_) => { }, } 
                                GET_4th_MTF_VAL(&mut zn);
                                s.state = 41; loop {
                                    let mut tmp_uc_32 = uc as u32; 
                                    match GET_1st_single_BIT_half(s, &mut tmp_uc_32){   Ok(_) => {  uc = tmp_uc_32 as u8; break;  }    Err(_) => { uc = tmp_uc_32 as u8; }, } 
                                    if unsafe { (*s.strmD).avail_in } == 0 {  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal;               }
                                    GET_2nd_single_BIT_half(s);
                                } 
                                GET_5th_MTF_VAL(&mut zvec, zn);
                            }
                            // 不在宏内的任何循环中
                            if GET_6th_MTF_VAL_IFIDX(zvec as isize, gBase_idx, zn as isize, &s) {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; }
                            GET_7th_MTF_VAL_IDX(&mut nextSym, gPerm_idx, zvec as isize, gBase_idx, zn as isize, &s);                           
                            
                               
                            continue;  // 对应的是37后的大loop while (True) {
                        }// continue-大loop结束位点
                    } //出while loop


                    println!("AFTER nextSym == BZ_RUNA || nextSym == BZ_RUNB  else");
                    if s.origPtr < 0 || s.origPtr >= nblock  {   retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  };
                    /*-- Set up cftab to facilitate generation of T^(-1) --*/
                    // 1st loop
                    for i in 0..=255   {                     /* Check: unzftab entries in range. */
                        if (s.unzftab[i] < 0 || s.unzftab[i] > nblock)   {  retVal = CONSTS.BZ_DATA_ERROR;   save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);  return retVal; }
                    }
                    s.cftab[0] = 0;       /* start Actually generate cftab. */
                    // 2nd loop
                    for i in 1..=256 {   s.cftab[i] = s.unzftab[i - 1];  }
                    // 3rd loop
                    for i in 1..=256 {     s.cftab[i] += s.cftab[i - 1];  }
                    // 4th loop         /* Check: cftab entries in range. */
                    for i in 0..=256 {
                        if s.cftab[i] < 0 || s.cftab[i] > nblock { retVal = CONSTS.BZ_DATA_ERROR;         save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;   }
                    }
                    // 5th loop           /* Check: cftab entries non-descending. */
                    for i in 1..=256 {
                        if s.cftab[i - 1] > s.cftab[i] { retVal = CONSTS.BZ_DATA_ERROR;         save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;   }
                    }

                    // Setting other struct fields
                    s.state_out_len = 0;
                    s.state_out_ch = 0;
                    BZ_INITIALISE_CRC ( &mut s.calculatedBlockCRC );
                    s.state = BZ_X_OUTPUT as i32; //2;
                    if s.verbosity >= 2 {   println!("    block size = {}", s.blockSize100k);    }
                    serializeDStateParitial(s, "before_GsmallDecompress.rs.txt");
                    if s.smallDecompress {
                        /*-- Make a copy of cftab, used in generation of T --*/
                        for i in 0..=256 {   s.cftabCopy[i] = s.cftab[i];      }

                        /*-- compute the T vector --*/
                        for i in 0..=nblock-1 { 
                            if let Some(ll16_ref) = (&mut s.ll16) {
                                uc = ll16_ref[i as usize] as u8;
                                SET_LL_part(i as usize, uc, s);                               
                            }
                            s.cftabCopy[uc as usize] += 1;
                        }

                        /*-- Compute T^(-1) by pointer reversal on T --*/
                        i = s.origPtr as i32;
                        if let Some(get_) = GET_LL(i as usize, &s){
                            j = get_ as i32;
                        } else {   println!("GET_LL(i as usize, &s) is None");              }
                        loop {
                            let mut tmp: u32 = 0;
                            SET_LL(j as usize, i, s);
                            i = j as i32;  // 添加 as usize 转换
                            j = tmp as i32;
                            if i == s.origPtr as i32 {  break; }
                        }

                        s.tPos = s.origPtr as u32;
                        s.nblock_used = 0;
                        if u8_to_bool(s.blockRandomised) {
                            BZ_RAND_INIT_MASK(s);
                            BZ_GET_SMALL_1(s);  BZ_GET_SMALL_2(s); s.nblock_used += 1;
                            BZ_RAND_UPD_MASK(s);
                            s.k0 ^= BZ_RAND_MASK(s);
                        }
                        else {
                            BZ_GET_SMALL_1(s);  BZ_GET_SMALL_2(s); s.nblock_used += 1;
                        }
                    } else {
                          /*-- compute the T^(-1) vector --*/
                          //🔥🔥🔥🔥 此时tt已经出现不一致 🔥🔥🔥🔥
                        println!("🌹最后一个nblock: {}\n", nblock);
                        for i in 0..nblock {
                            serializeDStateParitial(s, &format!("tt_{}.rs.txt", i));
                            if let Some(ref mut tt_ref) = s.tt {
                                uc = tt_ref[i as usize] as u8 & 0xff;
                                tt_ref[s.cftab[uc as usize] as usize] |= (i << 8) as u32;
                            }
                            s.cftab[uc as usize] += 1;
                        }
                    
                        //todo: raw ppinter for tt
                        // s.tPos = (s.tt[s.origPtr as usize] >> 8) as usize;
                        if let Some(ref tt) = s.tt {
                            s.tPos = (tt[s.origPtr as usize] >> 8) as u32;
                        }
                        s.nblock_used = 0;
            
                        if u8_to_bool(s.blockRandomised) {
                            BZ_RAND_INIT_MASK(s);
                            match BZ_GET_FAST_1(s) {     
                                Ok(_) => {  return 1;  }
                                Err(_) => {}, 
                            } 
                            BZ_GET_FAST_2(s); s.nblock_used += 1;
                            BZ_RAND_UPD_MASK(s);
                            s.k0 ^= BZ_RAND_MASK(s);
                        }
                        else {
                            match BZ_GET_FAST_1(s) {     
                                Ok(_) => {  return 1;  }
                                Err(_) => {}, 
                            } 
                            serializeDStateParitial(s, "BZ_GET_FAST_FINAL.rs.txt");
                            println!(">>最后一个BZ_GET_FAST: s->k0: {:?}\n", s.k0);
                            BZ_GET_FAST_2(s); s.nblock_used += 1;
                            serializeDStateParitial(s, "BZ_GET_FAST_FINAL-AFTER.rs.txt");
                            println!(">>FINAL最后一个BZ_GET_FAST: s->k0: {:?}\n", s.k0);
                        }
                    }     
                    {
                        retVal = CONSTS.BZ_OK; 
                        save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); 
                        return retVal;
                    }
                    s.state = 42;
                    println!("Here is endhdr_2:");
                    { retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                }
            }, // 31
            42 => {
                println!("42_wei_uc {:?}:", uc);
                s.state = 42; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x72 {
                    retVal = CONSTS.BZ_DATA_ERROR;
                    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                    return retVal;
                }
                s.state = 43;
            },
            43 => {
                s.state = 43; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x45 {  retVal = CONSTS.BZ_DATA_ERROR;  save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel,gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;              }
                s.state = 44;
            },
            44 => {
                s.state = 44; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {
                        retVal = CONSTS.BZ_OK;
                        save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                        return retVal;
                    }
                    
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x38 {
                    retVal = CONSTS.BZ_DATA_ERROR;
                    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel,gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                    return retVal;
                }
                s.state = 45;
            },
            45 => {
                s.state = 45; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {
                        retVal = CONSTS.BZ_OK;
                        save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                        return retVal;
                    }
                    
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x50 {
                    retVal = CONSTS.BZ_DATA_ERROR;
                    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel,gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                    return retVal;
                }
                s.state = 46;
            },
            46 => {  //add  s->storedCombinedCRC = 0;
                s.state = 46; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    
                    GET_2nd_UCHAR_half(s);
                }
                if uc != 0x90 {
                    retVal = CONSTS.BZ_DATA_ERROR;
                    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel,gMinlen, gLimit_idx, gBase_idx, gPerm_idx);
                    return retVal;
                }
                s.storedCombinedCRC = 0;
                s.state = 47;
            },
            // 47,48
            47 => {   
                s.state = 47; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    GET_2nd_UCHAR_half(s);
                }
                s.storedCombinedCRC = s.storedCombinedCRC << 8 | uc as u32;
                s.state = 48;
            },
            48 => {   
                // no more second save_state_and_return
                // s->storedCombinedCRC = (s->storedCombinedCRC << 8) | ((u32) uc);
                s.state = 48; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    GET_2nd_UCHAR_half(s);
                }
                s.storedCombinedCRC = s.storedCombinedCRC << 8 | uc as u32;
                s.state = 49;
            },
            49 => {   
                // no more second save_state_and_return
                // s->storedCombinedCRC = (s->storedCombinedCRC << 8) | ((u32) uc);
                s.state = 49; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 {   retVal = CONSTS.BZ_OK;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                    GET_2nd_UCHAR_half(s);
                }
                s.storedCombinedCRC = s.storedCombinedCRC << 8 | uc as u32;
                s.state = 50;
            },
            50 => {   
                s.state = 50; loop {
                    let mut tmp_uc_32 = uc as u32;
                    match GET_1st_UCHAR_half(s, &mut tmp_uc_32){     
                        Ok(_) => {  uc = tmp_uc_32 as u8; break;  }
                        Err(_) => { uc = tmp_uc_32 as u8; }, 
                    } 
                    if unsafe { (*s.strmD).avail_in } == 0 { retVal = CONSTS.BZ_OK; save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal; }
                    GET_2nd_UCHAR_half(s);
                }
                s.storedCombinedCRC = s.storedCombinedCRC << 8 | uc as u32;
                s.state = BZ_X_IDLE as i32;
                {   retVal = CONSTS.BZ_STREAM_END;    save_state_and_return(s, i as i32, j, t, alphaSize, nGroups, nSelectors, EOB, groupNo, groupPos, nextSym, nblockMAX, nblock, es, N, curr, zt, zn, zvec, zj, gSel, gMinlen, gLimit_idx, gBase_idx, gPerm_idx); return retVal;  }
                
            },
            _ => { 
                    BZ2_bz__AssertH__fail(4001);
            }
        } // end 
    }

    BZ2_bz__AssertH__fail(4002);
    return 4002;

    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn another_test() {

    }

}
    // loop {
    //     match s.state {
    //         State::State10 => {
        
    //             // ... State10 的逻辑
    //             if s.bsLive >= 8 {
    //                 let v = (s.bsBuff >> (s.bsLive - 8)) & ((1 << 8) - 1);
    //                 s.bsLive -= 8;
    //                 // ... 其他逻辑
    //             }
    //             if unsafe { (*s.strmD).avail_in } == 0 {
    //                 return Err(0);
    //             }
    //             // ... 其他逻辑
    //         }
    //         // ... 其他状态的处理
    //         State::End => break,
    //         _ => {}
    //     }
    // }


