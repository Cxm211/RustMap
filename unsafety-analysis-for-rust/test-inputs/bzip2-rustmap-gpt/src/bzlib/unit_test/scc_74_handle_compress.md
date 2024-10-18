bz_stream结构体: bz_stream的定义是什么？特别是，它包含哪些字段？
pub struct bz_stream {
    pub   next_in: *mut i8, // char in C is i8 in Rust.
    pub   avail_in: u32,
    pub   total_in_lo32: u32,
     pub  total_in_hi32: u32,
   
    pub   next_out: *mut i8,
     pub  avail_out: u32,
     pub  total_out_lo32: u32,
     pub  total_out_hi32: u32,
   
    pub   state: *mut DState, // I'm assuming DState here based on your earlier code.
       
    pub   bzalloc: Option<fn(*mut std::ffi::c_void, Int32, Int32) -> *mut std::ffi::c_void>,
    pub   bzfree: Option<fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub   opaque: *mut std::ffi::c_void,
}



外部函数:
copy_output_until_stop: 这个函数的签名和功能是什么？
pub fn copy_output_until_stop(s: &mut EState) -> bool 
isempty_RL: 这个函数的签名和功能是什么？
fn is_empty_rl(s: &EState) -> bool {
    !(s.state_in_ch < 256 && s.state_in_len > 0)
}


prepare_new_block: 这个函数的签名和功能是什么？
fn prepare_new_block(s: &mut EState) {
    s.nblock = 0;
    s.num_z = 0;
    s.state_out_pos = 0;
    s.block_crc = 0xffffffff;
    for i in 0..256 {
        s.in_use[i] = false;
    }
    s.block_no += 1;
}


copy_input_until_stop: 这个函数的签名和功能是什么？
pub fn copy_input_until_stop(s: &mut EState) -> bool {}
flush_RL: 这个函数的签名和功能是什么？
fn flush_rl(s: &mut EState) {}

BZ2_compressBlock: 我们之前已经讨论过此函数的转换，我将在Rust实现中使用它。
枚举和常量:
BZ_S_OUTPUT, BZ_S_INPUT, BZ_M_FINISHING, BZ_M_FLUSHING, BZ_M_RUNNING: 这些常量或枚举值代表什么？
#define BZ_M_IDLE      1
#define BZ_M_RUNNING   2
#define BZ_M_FLUSHING  3
#define BZ_M_FINISHING 4

#define BZ_S_OUTPUT    1
#define BZ_S_INPUT     2


Bool类型: 在Rust中，布尔类型用bool表示。我将在转换时使用bool来替代C中的Bool类型。

