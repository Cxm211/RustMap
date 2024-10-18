use std::fs;
use std::sync::Mutex;
use libc::{utime, utimbuf, fchmod, fchown};
use crate::bzip2::scc_82_crcError_83_ioError::*;
use lazy_static::lazy_static;
// 引入特定于Linux的MetadataExt trait
use std::os::linux::fs::MetadataExt;

lazy_static! {
    static ref FILE_META_INFO: Mutex<Option<fs::Metadata>> = Mutex::new(None);
}

pub fn save_input_file_meta_info(src_name: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(src_name)?;
    let mut file_meta_info = FILE_META_INFO.lock().unwrap();
    *file_meta_info = Some(metadata);
    Ok(())
}


pub fn apply_saved_time_info_to_output_file(dst_name: &str) -> std::io::Result<()> {
    let file_meta_info = FILE_META_INFO.lock().unwrap();
    if let Some(meta) = file_meta_info.as_ref() {
        let actime = meta.st_atime();
        let modtime = meta.st_mtime();
        let u_tim_buf = utimbuf {
            actime,
            modtime,
        };

        let ret_val = unsafe {
            utime(dst_name.as_ptr() as *const i8, &u_tim_buf)
        };
        if ret_val != 0 {
            io_error();
        }
    }
    Ok(())
}


pub fn apply_saved_file_attr_to_output_file(fd: i32) -> std::io::Result<()> {
    let file_meta_info = FILE_META_INFO.lock().unwrap();
    if let Some(meta) = file_meta_info.as_ref() {
        let ret_val = unsafe {
            fchmod(fd, meta.st_mode())
        };
        if ret_val != 0 {
            io_error();
        }
        
        unsafe {
            fchown(fd, meta.st_uid(), meta.st_gid());
        }
    }
    Ok(())
}
