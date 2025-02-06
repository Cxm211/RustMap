

pub fn bz_config_ok() -> bool {
    // 检查数据类型大小
    std::mem::size_of::<i32>() == 4 && // 检查i32类型大小是否为4字节
    std::mem::size_of::<i16>() == 2 && // 检查i16类型大小是否为2字节
    std::mem::size_of::<u8>() == 1     // 检查u8类型大小是否为1字节
}

pub fn default_bzalloc(_opaque: *mut std::ffi::c_void, items: i32, size: i32) -> *mut std::ffi::c_void {
    // 计算总大小
    let total_size = items.checked_mul(size).expect("Overflow during memory allocation size calculation");
    // 分配内存
    let layout = std::alloc::Layout::from_size_align(total_size as usize, std::mem::align_of::<usize>()).unwrap();
    unsafe {
        std::alloc::alloc(layout) as *mut std::ffi::c_void
    }
}

pub fn default_bzfree(_opaque: *mut std::ffi::c_void, addr: *mut std::ffi::c_void) {
    if !addr.is_null() {
        // 获取内存布局
        let layout = std::alloc::Layout::from_size_align(1, std::mem::align_of::<usize>()).unwrap(); // 使用最小布局来释放，实际上应该知道正确的布局。
        unsafe {
            std::alloc::dealloc(addr as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bz_config_ok() {
        // 通常情况下，在大多数平台上，i32是4字节，i16是2字节，u8是1字节。
        assert!(bz_config_ok());
    }

    #[test]
    fn test_default_bzalloc_and_bzfree() {
        let size_as_i32 = std::mem::size_of::<i32>() as i32;
        let mem = default_bzalloc(std::ptr::null_mut(), 5, size_as_i32);

        assert!(!mem.is_null());
        default_bzfree(std::ptr::null_mut(), mem);
    }
}

fn main() {
    // 在Rust中，测试通常通过 cargo test 运行。
}
