mod bzlib;

pub struct DState<'a> {
    pub strm: Option<&'a mut bz_stream>,

    pub state: i32,

    pub state_out_ch: u8,
    pub state_out_len: i32,
    pub block_randomised: bool,
    pub r_n_to_go: i32,
    pub r_t_pos: i32,

    pub bs_buff: u32,
    pub bs_live: i32,

    pub block_size_100k: i32,
    pub small_decompress: bool,
    pub curr_block_no: i32,
    pub verbosity: i32,

    pub orig_ptr: i32,
    pub t_pos: u32,
    pub k0: i32,
    pub unzftab: [i32; 256],
    pub nblock_used: i32,
    pub cftab: [i32; 257],
    pub cftab_copy: [i32; 257],

    pub tt: Option<&'a mut [u32]>,     // Dynamic array of `UInt32` equivalent to `u32` in Rust

    pub ll16: Option<&'a mut [u16]>,   // Dynamic array of `UInt16` equivalent to `u16` in Rust
    pub ll4: Option<&'a mut [u8]>,     // Dynamic array of `UChar` equivalent to `u8` in Rust

    pub stored_block_crc: u32,
    pub stored_combined_crc: u32,
    pub calculated_block_crc: u32,
    pub calculated_combined_crc: u32,

    pub n_in_use: i32,
    pub in_use: [bool; 256],
    pub in_use16: [bool; 16],
    pub seq_to_unseq: [u8; 256],

    pub mtfa: [u8; 4096],
    pub mtfbase: [i32; 256 / 16],
    pub selector: [u8; 2 + (900000 / 50)],
    pub selector_mtf: [u8; 2 + (900000 / 50)],
    pub len: [[u8; 258]; 6],

    pub limit: [[i32; 258]; 6],
    pub base: [[i32; 258]; 6],
    pub perm: [[i32; 258]; 6],
    pub min_lens: [i32; 6],

    pub save_i: i32,
    pub save_j: i32,
    pub save_t: i32,
    pub save_alpha_size: i32,
    pub save_n_groups: i32,
    pub save_n_selectors: i32,
    pub save_eob: i32,
    pub save_group_no: i32,
    pub save_group_pos: i32,
    pub save_next_sym: i32,
    pub save_nblock_max: i32,
    pub save_nblock: i32,
    pub save_es: i32,
    pub save_n: i32,
    pub save_curr: i32,
    pub save_zt: i32,
    pub save_zn: i32,
    pub save_zvec: i32,
    pub save_zj: i32,
    pub save_g_sel: i32,
    pub save_g_minlen: i32,
    pub save_g_limit: Option<&'a mut [i32]>,
    pub save_g_base: Option<&'a mut [i32]>,
    pub save_g_perm: Option<&'a mut [i32]>,
}


pub struct EState<'a> {
    pub strm: Option<&'a mut bz_stream>,

    pub mode: i32,
    pub state: i32,

    pub avail_in_expect: u32,

    pub arr1: Option<&'a mut [u32]>,   // Dynamic array of `u32`
    pub arr2: Option<&'a mut [u32]>,   // Dynamic array of `u32`
    pub ftab: Option<&'a mut [u32]>,   // Dynamic array of `u32`
    pub orig_ptr: i32,

    pub ptr: Option<&'a mut [u32]>,    // Dynamic array of `u32`
    pub block: Option<&'a mut [u8]>,   // Dynamic array of `u8` (UChar equivalent)
    pub mtfv: Option<&'a mut [u16]>,   // Dynamic array of `u16`
    pub zbits: Option<&'a mut [u8]>,   // Dynamic array of `u8` (UChar equivalent)

    pub work_factor: i32,

    pub state_in_ch: u32,
    pub state_in_len: i32,
    pub r_n_to_go: i32,
    pub r_t_pos: i32,

    pub nblock: i32,
    pub nblock_max: i32,
    pub num_z: i32,
    pub state_out_pos: i32,

    pub n_in_use: i32,
    pub in_use: [bool; 256],           // Bool array
    pub unseq_to_seq: [u8; 256],       // UChar array translated to `u8`

    pub bs_buff: u32,
    pub bs_live: i32,

    pub block_crc: u32,
    pub combined_crc: u32,

    pub verbosity: i32,
    pub block_no: i32,
    pub block_size_100k: i32,

    pub n_mtf: i32,
    pub mtf_freq: [i32; 258],          // Fixed-size array of `i32`
    pub selector: [u8; 2 + (900000 / 50)], // Fixed-size array of `u8`
    pub selector_mtf: [u8; 2 + (900000 / 50)], // Fixed-size array of `u8`

    pub len: [[u8; 258]; 6],           // Fixed-size 2D array
    pub code: [[i32; 258]; 6],         // Fixed-size 2D array of `i32`
    pub rfreq: [[i32; 258]; 6],        // Fixed-size 2D array of `i32`

    pub len_pack: [[u32; 4]; 258],     // Fixed-size 2D array of `u32`
}

#[repr(C)]
pub struct bzFile {
    pub handle: *mut libc::FILE, // Use a raw pointer to represent a C FILE*
    pub buf: [i8; 5000],         // Char is typically represented as i8 in Rust
    pub bufN: i32,               // Int32 translated to i32
    pub writing: bool,           // Bool translated to Rust's bool type
    pub strm: bz_stream,         // bz_stream remains unchanged
    pub lastErr: i32,            // Int32 translated to i32
    pub initialisedOk: bool,     // Bool translated to Rust's bool type
}
