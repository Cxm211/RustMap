use std::ptr;
use crate::bzlib::scc_36_root_27_default_bzfree_38_bz_config_ok_37_default_bzalloc::*;
use crate::global_vars::bzip2_c1::{EState, DState, bz_stream,  bzFile};
use crate::bzlib::scc_25_root_33_prepare_new_block_init_RL_isempty_RL_flush_RL_bzlib_static::*;
use std::fs::File;
use std::ptr::null_mut;

pub fn bz2_bzwrite_open(bzerror: &mut i32, f: File, block_size100k: i32, verbosity: i32, mut work_factor: i32) -> Option<Box<bzFile>> {
    // println!("开始执行 bz2_bzwrite_open");

    let mut bzf = Box::new(bzFile::default());

    *bzerror = 0;
    bzf.lastErr = 0;

    // println!("检查文件元数据");
    if f.metadata().is_err() {
        // println!("错误：无法获取文件元数据");
        *bzerror = -21;
        bzf.lastErr = -21;
        return None;
    }

    // println!("检查 block_size100k 参数");
    if block_size100k < 1 || block_size100k > 9 {
        // println!("错误：block_size100k 参数无效");
        *bzerror = -22;
        bzf.lastErr = -22;
        return None;
    }

    // println!("检查 work_factor 参数");
    if work_factor < 0 || work_factor > 250 {
        // println!("错误：work_factor 参数无效");
        *bzerror = -23;
        bzf.lastErr = -23;
        return None;
    }

    // println!("检查 verbosity 参数");
    if verbosity < 0 || verbosity > 4 {
        // println!("错误：verbosity 参数无效");
        *bzerror = -24;
        bzf.lastErr = -24;
        return None;
    }

    // println!("初始化 bzFile 结构");
    bzf.initialisedOk = false;
    bzf.bufN = 0;
    bzf.handle = Box::into_raw(Box::new(f));
    bzf.writing = true;
    bzf.strm.bzalloc = None;
    bzf.strm.bzfree = None;
    bzf.strm.opaque = null_mut();

    if work_factor == 0 {
        work_factor = 30;
    }

    // println!("调用 bz2_bzcompress_init");
    let ret = bz2_bzcompress_init(&mut bzf.strm, block_size100k, verbosity, work_factor);
    if ret != 0 {
        // println!("错误：bz2_bzcompress_init 失败，错误码: {}", ret);
        *bzerror = ret;
        bzf.lastErr = ret;
        unsafe {
            Box::from_raw(bzf.handle);
        }
        return None;
    }

    // println!("bz2_bzwrite_open 完成");
    bzf.strm.avail_in = 0;
    bzf.initialisedOk = true;
    Some(bzf)
}


use std::ffi::c_void;

pub fn bz2_bzcompress_init(
    strm: *mut bz_stream,
    block_size_100k: i32,
    verbosity: i32,
    mut work_factor: i32,
) -> i32 {
    if !bz_config_ok() {
        return -9;
    }

    if strm.is_null() || block_size_100k < 1 || block_size_100k > 9 || work_factor < 0 || work_factor > 250 {
        return -2;
    }

    if work_factor == 0 {
        work_factor = 30;
    }

    unsafe {
        let strm_ref = &mut *strm;

        if strm_ref.bzalloc.is_none() {
            strm_ref.bzalloc = Some(default_bzalloc);
        }
        if strm_ref.bzfree.is_none() {
            strm_ref.bzfree = Some(default_bzfree);
        }

        let s = (strm_ref.bzalloc.unwrap())(strm_ref.opaque, std::mem::size_of::<EState>() as i32, 1) as *mut EState;
        if s.is_null() {
            return -3;
        }

        let s_ref = &mut *s;
        s_ref.strm = Some(strm);

        let n = 100000 * block_size_100k as usize;
        Some((strm_ref.bzalloc.unwrap())(strm_ref.opaque, (n * std::mem::size_of::<u32>()) as i32, 1) as *mut u32);
        Some((strm_ref.bzalloc.unwrap())(strm_ref.opaque, ((n + (2 + 12 + 18 + 2)) * std::mem::size_of::<u32>()) as i32, 1) as *mut u32);
        Some((strm_ref.bzalloc.unwrap())(strm_ref.opaque, (65537 * std::mem::size_of::<u32>()) as i32, 1) as *mut u32);
        let n = 100000 * block_size_100k as usize;
        // 分配并初始化 arr1
        let arr1: Vec<u32>  = vec![0; n];  // 使用 0 初始化
        // 分配并初始化 arr2
        // 注意: 这里加上了 (2 + 12 + 18 + 2)
        let arr2: Vec<u32> = vec![0; n + (2 + 12 + 18 + 2)];
        
        // 分配并初始化 ftab
        s_ref.ftab = vec![0; 65537];
        
        // if s_ref.arr1.is_none() || s_ref.arr2.is_none() || s_ref.ftab.is_none() {
        //     if s_ref.arr1.is_some() {
        //         (strm_ref.bzfree.unwrap())(strm_ref.opaque, s_ref.arr1.unwrap() as *mut c_void);
        //     }
        //     if s_ref.arr2.is_some() {
        //         (strm_ref.bzfree.unwrap())(strm_ref.opaque, s_ref.arr2.unwrap() as *mut c_void);
        //     }
        //     if s_ref.ftab.is_some() {
        //         (strm_ref.bzfree.unwrap())(strm_ref.opaque, s_ref.ftab.unwrap() as *mut c_void);
        //     }
        //     if !s.is_null() {
        //         (strm_ref.bzfree.unwrap())(strm_ref.opaque, s as *mut c_void);
        //     }
        //     return -3;
        // }

        s_ref.blockNo = 0;
        s_ref.state = 2;
        s_ref.mode = 2;
        s_ref.combinedCRC = 0;
        // 是9，但貌似应该是1
        s_ref.blockSize100k = block_size_100k;
        s_ref.nblockMAX = 100000 * block_size_100k - 19;
        s_ref.verbosity = verbosity;
        s_ref.workFactor = work_factor;

        // 💡💡💡
        // s_ref.block = Some(s_ref.arr2.unwrap() as *mut u8);
        // s_ref.mtfv = Some(s_ref.arr1.unwrap() as *mut u16);
        s_ref.mtfv = arr1.iter().flat_map(|&x| {
            let high = (x >> 16) as u16;
            let low = x as u16;
            vec![high, low]
        }).collect();
        // 对于 s_ref.block    // 💡💡💡
        s_ref.block = arr2.iter().flat_map(|&x| {
            let bytes = x.to_ne_bytes();
            bytes.to_vec()
        }).collect();

        
        // s_ref.zbits = None;
        let zbits_size = s_ref.nblockMAX as usize;
        let zbits_vec = vec![0u8; zbits_size].into_boxed_slice();
        let zbits_size = s_ref.nblockMAX as usize;
        let zbits_vec = vec![0u8; zbits_size];
        s_ref.zbits = zbits_vec;

        // println!("49_初始化 arr1: {:?}", arr1);
        // s_ref.ptr = Some(s_ref.arr1.unwrap());     // 💡💡💡
        s_ref.ptr = arr1.clone();

        
        // 直接将s赋值给strm_ref.state
        strm_ref.state = s;

    
        strm_ref.total_in_lo32 = 0;
        strm_ref.total_in_hi32 = 0;
        strm_ref.total_out_lo32 = 0;
        strm_ref.total_out_hi32 = 0;

        // 我自己家的🏠🏠🏠
        // s_ref.numZ = 0;
        // s_ref.bsBuff = 0;
        // s_ref.bsLive = 0;

        init_rl(s_ref);
        prepare_new_block(s_ref);
    }

    0
}







#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Write, Read};
    use std::path::Path;
    use std::process::Command;
    use bzip2::write::BzEncoder;
    use bzip2::Compression;
    use std::fs::remove_file;
    
    #[test]
    fn test_bz2_bzwrite_open() {
            // 使用 touch 命令创建一个空文件
        let status = Command::new("touch")
            .arg("test_write_data.txt")
            .status()
            .expect("创建文件失败");
        assert!(status.success(), "touch 命令执行失败");
        let compressed_file_path = Path::new("test_write_data.txt.bz2");
    
        if compressed_file_path.exists() {
            remove_file(&compressed_file_path).expect("无法删除已存在的test_write_data.txt.bz2文件");
        }
    
        let test_file_path = Path::new("test_write_data.txt");
        let test_data = "Hello, world!";
        {
            let mut file = File::create(&test_file_path).expect("无法创建测试文件");
            file.write_all(test_data.as_bytes()).expect("写入初始数据到测试文件失败");
        }
    
        let output = Command::new("bzip2")
            .arg(test_file_path)
            .output()
            .expect("执行bzip2命令失败");
    
        if !output.status.success() {
            eprintln!("bzip2命令失败!");
            eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            assert!(false, "bzip2命令失败");
        }
    
        if !compressed_file_path.exists() {
            panic!("经bzip2压缩后的文件不存在，请检查bzip2是否正确运行");
        }
    
        let mut f = File::open(&compressed_file_path).expect("无法打开测试文件以进行读写");
        let mut bzerror: i32 = 0;
        let result = bz2_bzwrite_open(&mut bzerror, f, 1, 0, 30);
    
        if let None = result {
            eprintln!("bzerror的值是: {}", bzerror); // 输出bzerror的具体值，帮助诊断
        }
    
        assert!(result.is_some(), "使用bz2_bzwrite_open打开文件失败");
    
        let bzf = result.unwrap();
        assert_eq!(bzf.lastErr, 0, "期望lastErr为0，但得到了{}", bzf.lastErr);
        assert_eq!(bzf.writing, true, "期望writing为true");
    
        remove_file(&compressed_file_path).expect("删除测试文件失败");
    }
    
}
