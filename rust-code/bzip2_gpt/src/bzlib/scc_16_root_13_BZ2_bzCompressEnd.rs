use crate::global_vars::bzip2_c1::{bz_stream, EState};
use std::ptr;
pub fn BZ2_bzCompressEnd(strm: &mut Option<bz_stream>) -> i32 {
    if strm.is_none() {
        return -2;
    }

    let s = unsafe {
        if let Some(ref mut stream) = strm {
            match stream.state.as_mut() {
                Some(state) => state,
                None => return -2,
            }
        } else {
            return -2;
        }
    };

    // 对比裸指针
    if s.strm.is_none() || s.strm.unwrap() != strm.as_mut().unwrap() as *mut bz_stream {
        return -2;
    }

    // arr1, arr2, 和 ftab 作为 Vec<u32> 类型，将在离开作用域时自动释放内存

    // 清理 strm.state
    unsafe {
        strm.as_mut().unwrap().state = std::ptr::null_mut();
    }


    0
}













#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_BZ2_bzCompressEnd_null_stream() {
        let mut strm: Option<bz_stream> = None;
        let result = BZ2_bzCompressEnd(&mut strm);
        assert_eq!(result, -2);
    }
}
