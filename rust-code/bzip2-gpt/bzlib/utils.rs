use std::mem::size_of;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

pub fn bz_config_ok() -> bool {
    if size_of::<i32>() != 4 {
        return false;
    }
    if size_of::<i16>() != 2 {
        return false;
    }
    if size_of::<u8>() != 1 {
        return false;
    }
    true
}

pub fn default_bzalloc(_opaque: *mut std::ffi::c_void, items: i32, size: i32) -> *mut std::ffi::c_void {
    let total_size = items.checked_mul(size).expect("Overflow during allocation") as usize;
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>()).unwrap();

    unsafe {
        let ptr = alloc(layout) as *mut std::ffi::c_void;
        if ptr.is_null() {
            ptr::null_mut() // Return null if allocation fails
        } else {
            ptr
        }
    }
}

pub fn default_bzfree(_opaque: *mut std::ffi::c_void, addr: *mut std::ffi::c_void) {
    if !addr.is_null() {
        unsafe {
            let layout = Layout::for_value(&*(addr as *mut u8));
            dealloc(addr as *mut u8, layout);
        }
    }
}

pub fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.num_z = 0;
    s.state_out_pos = 0;
    s.block_crc = 0xffffffff; // Equivalent to 0xffffffffL in C

    // Set the `in_use` array to false (Bool is assumed to be bool in Rust)
    for i in 0..256 {
        s.in_use[i] = false;
    }

    // Increment the block number
    s.block_no += 1;
}

pub fn init_rl(s: &mut EState) {
    s.state_in_ch = 256;
    s.state_in_len = 0;
}

pub fn isempty_rl(s: &EState) -> bool {
    if s.state_in_ch < 256 && s.state_in_len > 0 {
        return false;
    } else {
        return true;
    }
}
