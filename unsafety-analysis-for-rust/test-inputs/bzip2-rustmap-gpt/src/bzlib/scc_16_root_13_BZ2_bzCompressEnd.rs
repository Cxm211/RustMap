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


// pub fn BZ2_bzCompressEnd(strm: &mut Option<bz_stream>) -> i32 {
//     let stream_ref = match strm.as_mut() {
//         Some(s) => s,
//         None => return -2,
//     };

//     let s: *mut EState = stream_ref.state as *mut _ as *mut EState;

//     if s.is_null() {
//         return -2;
//     }

//     if s.is_null() {
//         return -2;
//     }

//     unsafe {
//         if let Some(ptr) = (*s).arr1 {
//             if !ptr.is_null() {
//                 stream_ref.bzfree.unwrap()(stream_ref.opaque, ptr as *mut _);
//             }
//         }

//         if let Some(ptr) = (*s).arr2 {
//             if !ptr.is_null() {
//                 stream_ref.bzfree.unwrap()(stream_ref.opaque, ptr as *mut _);
//             }
//         }

//         if let Some(ptr) = (*s).ftab {
//             if !ptr.is_null() {
//                 stream_ref.bzfree.unwrap()(stream_ref.opaque, ptr as *mut _);
//             }
//         }

//         stream_ref.bzfree.unwrap()(stream_ref.opaque, s as *mut _);
//     }

//     stream_ref.state = ptr::null_mut();

//     0
// }

// ... 其他代码不变 ...

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
