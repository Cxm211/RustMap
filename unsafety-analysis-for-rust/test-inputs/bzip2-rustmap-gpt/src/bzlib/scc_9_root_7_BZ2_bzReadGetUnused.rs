use crate::global_vars::bzip2_c1::{bzFile};

use std::fs::File;
use std::ptr;

pub fn bz2_bz_read_get_unused(bzerror: &mut Option<i32>, 
                              b: *mut bzFile, 
                              unused: &mut Option<*mut i8>, 
                              n_unused: &mut Option<i32>) {
    // 确认 b 是否为 null
    if b.is_null() {
        set_error(bzerror, ptr::null_mut(), -2);
        return;
    }

    let bzf = unsafe { &mut *b };  // 将指针 deref 为可变引用

    if bzf.lastErr != 4 {
        set_error(bzerror, b, -1);
        return;
    }

    if unused.is_none() || n_unused.is_none() {
        set_error(bzerror, b, -2);
        return;
    }

    if let Some(err) = bzerror {
        *err = 0;
    }
    bzf.lastErr = 0;

    *n_unused.as_mut().unwrap() = bzf.strm.avail_in as i32;
    *unused.as_mut().unwrap() = bzf.strm.next_in;
}

// 由于重复性的代码，提取出一个设置错误的函数
fn set_error(bzerror: &mut Option<i32>, b: *mut bzFile, err_code: i32) {
    if let Some(err) = bzerror {
        *err = err_code;
    }
    if !b.is_null() {
        unsafe { (*b).lastErr = err_code; }
    }
}