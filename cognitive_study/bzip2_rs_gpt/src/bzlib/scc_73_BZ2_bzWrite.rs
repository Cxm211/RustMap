
use crate::global_vars::bzip2_c1::*;
use std::io::{Error, Write};
use crate::bzlib::scc_75_BZ2_bzCompress_76_BZ2_bzWriteClose64::*;

pub fn bz2_bz_write(bzerror: &mut Option<i32>, b: BZFILE, buf: &[u8], len:i32) {
    // println!("77_进入 bz2_bz_write 函数");
    let mut n: i32;
    let mut n2: i32;
    let mut ret: i32;
    if let Some(bzf) = unsafe { b.as_mut() } {
        // 对应C中的第一个if块
        *bzerror = Some(0);
        bzf.lastErr = 0;

        if len < 0 {
            *bzerror = Some(-2);
            bzf.lastErr = -2;
            return;
        }

        if !bzf.writing {
            *bzerror = Some(-1);
            bzf.lastErr = -1;
            return;
        }

        // 检查文件操作错误
        unsafe {
            if (*bzf.handle).sync_data().is_err() {
                *bzerror = Some(-6);
                bzf.lastErr = -6;
                return;
            }
        }

        if len == 0 {
            *bzerror = Some(0);
            bzf.lastErr = 0;
            return;
        }

        // 设置流参数
        bzf.strm.avail_in = len as u32;
        bzf.strm.next_in = buf.as_ptr() as *mut _;

        while true {
            bzf.strm.avail_in = len as u32;
            bzf.strm.next_out = bzf.buf.as_mut_ptr() as *mut i8;
            ret = bz2_compress(&mut bzf.strm, 0);
            if ret != 1 {
                *bzerror = Some(ret);
                bzf.lastErr = ret;
                return;
            }

            if bzf.strm.avail_out < 5000 {
                let n: usize = 5000 - bzf.strm.avail_out as usize;
                let n2 = match unsafe { (&mut *bzf.handle).write(&bzf.buf[0..n]) } {
                    Ok(written) => written,
                    Err(_) => {
                        *bzerror = Some(-6);
                        bzf.lastErr = -6;
                        return;
                    }
                };
                
    
                if n != n2 || unsafe { (&mut *bzf.handle).flush().is_err() } {
                    *bzerror = Some(-6);
                    bzf.lastErr = -6;
                    return;
                }
                
            }
            if bzf.strm.avail_in == 0 {
                *bzerror = Some(0);
                bzf.lastErr = 0;
                return;
            }    
        }
    }
}

    //     // println!("已获取 bzFile");
    //     match serialize_bzfile(bzf, "77_bzFile_BZ2_bzWrite_rust.json") {
    //         Ok(_) => // println!("Data successfully serialized to file."),
    //         Err(e) => eprintln!("Failed to serialize data: {}", e),
    //     }

    //     if let Some(error) = bzerror {
    //         *error = 0;
    //     }
    //     bzf.lastErr = 0;

    //     if buf.is_empty() || !bzf.writing {
    //         // println!("77_参数验证失败: buf为空或不处于写入状态");
    //         set_error(bzerror, &mut bzf.lastErr, if !bzf.writing { -1 } else { 0 });
    //         return;
    //     }

    // // println!("77_开始写入数据");
    //     unsafe {
    //         if bzf.handle.as_mut().unwrap().write(buf).is_err() {
    //             // println!("写入数据时发生错误");
    //             set_error(bzerror, &mut bzf.lastErr, -6);
    //             return;
    //         }
    //     }

    //     // println!("77_开始数据压缩处理");
    //     bzf.strm.avail_in = buf.len() as u32;
    //     bzf.strm.next_in = buf.as_ptr() as *mut _;
    //     //💡打印💡💡
    //     let result = serialize_bz_stream(&mut bzf.strm,"77_strm_bzWrite_bzf_rust.json"); 
    //     if let Err(e) = result {
    //         eprintln!("Error saving bz_stream: {}", e);
    //     }

    //     // println!("77_即将进入压缩循环");
    //     loop {
    //         // println!("77_bzWrite_压缩循环开始");
    //         bzf.strm.avail_out = 5000;
    //         bzf.strm.next_out = bzf.buf.as_mut_ptr() as *mut i8;
    //         // 出错点🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥
    //         // bzf.strm.state
    //         let ret = bz2_compress(&mut bzf.strm, 0);
    //         // println!("压缩返回值: {}", ret);
    //         if ret != 1 {
    //             // println!("压缩过程中发生错误");
    //             set_error(bzerror, &mut bzf.lastErr, ret);
    //             return;
    //         }
    
    //         if bzf.strm.avail_out < 5000 {
    //             let n = 5000 - bzf.strm.avail_out as usize;
    //             // println!("处理压缩后的数据, n = {}", n);
    //             unsafe {
    //                 if bzf.handle.as_mut().unwrap().write(&bzf.buf[..n]).is_err() {
    //                     // println!("处理压缩后的数据时发生错误");
    //                     set_error(bzerror, &mut bzf.lastErr, -6);
    //                     return;
    //                 }
    //             }
    //         }
    
    //         if bzf.strm.avail_in == 0 {
    //             // println!("压缩处理完成");
    //             set_error(bzerror, &mut bzf.lastErr, 0);
    //             return;
    //         }
    //     }
    // } else {
    //     // println!("bzFile 对象无效");
    //     if let Some(error) = bzerror {
    //         *error = -2;
    //     }
    // }

fn set_error(bzerror: &mut Option<i32>, lastErr: &mut i32, err: i32) {
    // println!("设置错误码: {}", err);
    if let Some(error) = bzerror {
        *error = err;
    }
    *lastErr = err;
}
