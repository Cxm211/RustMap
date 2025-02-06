use crate::global_vars::bzip2_c1::{BZFILE, blockSize100k, verbosity, workFactor};
use crate::bzlib::scc_73_BZ2_bzWrite::*;
use crate::bzlib::scc_46_root_33_myfeof::*;
use crate::bzip2::scc_96_configError::*;
use crate::bzlib::scc_41_BZ2_bzCompressInit_42_BZ2_bzWriteOpen::bz2_bzwrite_open;
use crate::bzip2::scc_88_outOfMemory_scc_89_myMalloc::*;
use crate::bzlib::scc_75_BZ2_bzCompress_76_BZ2_bzWriteClose64::{bz2_bz_write_close64,};
use crate::bzlib::scc_73_BZ2_bzWrite::*;
use crate::bzlib::scc_41_BZ2_bzCompressInit_42_BZ2_bzWriteOpen::*;
use crate::bzip2::scc_82_crcError_83_ioError::*;
use crate::bzip2::scc_88_outOfMemory_scc_89_myMalloc::*;

use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use crate::bzlib::bz_constants::*;

pub fn compress_stream(stream: &mut File, z_stream: &mut File) -> Result<(), String> {
    let mut bzf: Option<BZFILE> = None;
    let mut ibuf = [0u8; 5000];
    let mut nbytes_in_lo32 = 0u32;
    let mut nbytes_in_hi32 = 0u32;
    let mut nbytes_out_lo32 = 0u32;
    let mut nbytes_out_hi32 = 0u32;
    let mut bzerr = 0i32;
    // println!("110_compressStream");
    if stream.metadata().is_err() || z_stream.metadata().is_err() {
        return Err("IO error".to_string());
    }

    let cloned_z_stream = z_stream.try_clone().map_err(|e| e.to_string())?;
    let bzf_boxed = unsafe {
        bz2_bzwrite_open(
            &mut bzerr, 
            cloned_z_stream, 
            blockSize100k, 
            verbosity, 
            workFactor
        )
    }.ok_or_else(|| "BZ2_bzWriteOpen failed".to_string())?;
    bzf = Some(Box::into_raw(bzf_boxed)); // 将 Box 转换为原始指针

    while !myfeof(stream) {
        match stream.read(&mut ibuf) {
            Ok(0) => break,
            Ok(n) => {
                let bzf_unwrapped = bzf.as_mut().ok_or("BZFile is null")?;
                bz2_bz_write(&mut Some(bzerr), *bzf_unwrapped, &ibuf[..n], ibuf[..n].len() as i32);//❌
                // 移除 map_err 调用，假设函数没有返回 Result
            }
            Err(e) => return Err(e.to_string()),
        }

        if bzerr != BZ_OK {
            return handle_err(bzerr);
        }
    }

    if let Some(bzfile) = bzf {
        bz2_bz_write_close64(&mut Some(bzerr), bzfile, 0, &mut Some(nbytes_in_lo32), &mut Some(nbytes_in_hi32),
                             &mut Some(nbytes_out_lo32), &mut Some(nbytes_out_hi32));
        // 移除 map_err 调用，假设函数没有返回 Result
    }
    Ok(())
}

fn handle_err(bzerr: i32) -> Result<(), String> {
    match bzerr {
        BZ_CONFIG_ERROR => {
            config_error();
            Err("Configuration error".to_string())
        },
        BZ_MEM_ERROR => {
            out_of_memory("input_name", "output_name", "program_name", None);
            Err("Out of memory".to_string())
        },
        BZ_IO_ERROR => {
            io_error();
            Err("IO error".to_string())
        },
        _ => {
            panic!("compress:unexpected error");
        }
    }
}

