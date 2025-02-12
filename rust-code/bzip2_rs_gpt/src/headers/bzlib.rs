

pub struct bz_stream {
    pub next_in: *mut u8,
    pub avail_in: u32,
    pub total_in_lo32: u32,
    pub total_in_hi32: u32,

    pub next_out: *mut u8,
    pub avail_out: u32,
    pub total_out_lo32: u32,
    pub total_out_hi32: u32,

    pub state: *mut std::ffi::c_void,

    pub bzalloc: Option<unsafe extern "C" fn(*mut std::ffi::c_void, i32, i32) -> *mut std::ffi::c_void>,
    pub bzfree: Option<unsafe extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub opaque: *mut std::ffi::c_void,
}