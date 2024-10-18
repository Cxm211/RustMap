#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub static mut tri: [std::os::raw::c_int; 171] = [
    55 as std::os::raw::c_int,
    94 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    95 as std::os::raw::c_int,
    30 as std::os::raw::c_int,
    96 as std::os::raw::c_int,
    77 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    26 as std::os::raw::c_int,
    67 as std::os::raw::c_int,
    97 as std::os::raw::c_int,
    13 as std::os::raw::c_int,
    76 as std::os::raw::c_int,
    38 as std::os::raw::c_int,
    45 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    36 as std::os::raw::c_int,
    79 as std::os::raw::c_int,
    16 as std::os::raw::c_int,
    37 as std::os::raw::c_int,
    68 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    9 as std::os::raw::c_int,
    18 as std::os::raw::c_int,
    70 as std::os::raw::c_int,
    26 as std::os::raw::c_int,
    6 as std::os::raw::c_int,
    18 as std::os::raw::c_int,
    72 as std::os::raw::c_int,
    79 as std::os::raw::c_int,
    46 as std::os::raw::c_int,
    59 as std::os::raw::c_int,
    79 as std::os::raw::c_int,
    29 as std::os::raw::c_int,
    90 as std::os::raw::c_int,
    20 as std::os::raw::c_int,
    76 as std::os::raw::c_int,
    87 as std::os::raw::c_int,
    11 as std::os::raw::c_int,
    32 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    7 as std::os::raw::c_int,
    49 as std::os::raw::c_int,
    18 as std::os::raw::c_int,
    27 as std::os::raw::c_int,
    83 as std::os::raw::c_int,
    58 as std::os::raw::c_int,
    35 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    11 as std::os::raw::c_int,
    25 as std::os::raw::c_int,
    57 as std::os::raw::c_int,
    29 as std::os::raw::c_int,
    85 as std::os::raw::c_int,
    14 as std::os::raw::c_int,
    64 as std::os::raw::c_int,
    36 as std::os::raw::c_int,
    96 as std::os::raw::c_int,
    27 as std::os::raw::c_int,
    11 as std::os::raw::c_int,
    58 as std::os::raw::c_int,
    56 as std::os::raw::c_int,
    92 as std::os::raw::c_int,
    18 as std::os::raw::c_int,
    55 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    90 as std::os::raw::c_int,
    3 as std::os::raw::c_int,
    60 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    49 as std::os::raw::c_int,
    41 as std::os::raw::c_int,
    46 as std::os::raw::c_int,
    33 as std::os::raw::c_int,
    36 as std::os::raw::c_int,
    47 as std::os::raw::c_int,
    23 as std::os::raw::c_int,
    92 as std::os::raw::c_int,
    50 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    36 as std::os::raw::c_int,
    59 as std::os::raw::c_int,
    42 as std::os::raw::c_int,
    79 as std::os::raw::c_int,
    72 as std::os::raw::c_int,
    20 as std::os::raw::c_int,
    82 as std::os::raw::c_int,
    77 as std::os::raw::c_int,
    42 as std::os::raw::c_int,
    56 as std::os::raw::c_int,
    78 as std::os::raw::c_int,
    38 as std::os::raw::c_int,
    80 as std::os::raw::c_int,
    39 as std::os::raw::c_int,
    75 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    66 as std::os::raw::c_int,
    66 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    3 as std::os::raw::c_int,
    55 as std::os::raw::c_int,
    72 as std::os::raw::c_int,
    44 as std::os::raw::c_int,
    25 as std::os::raw::c_int,
    67 as std::os::raw::c_int,
    84 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    67 as std::os::raw::c_int,
    11 as std::os::raw::c_int,
    61 as std::os::raw::c_int,
    40 as std::os::raw::c_int,
    57 as std::os::raw::c_int,
    58 as std::os::raw::c_int,
    89 as std::os::raw::c_int,
    40 as std::os::raw::c_int,
    56 as std::os::raw::c_int,
    36 as std::os::raw::c_int,
    85 as std::os::raw::c_int,
    32 as std::os::raw::c_int,
    25 as std::os::raw::c_int,
    85 as std::os::raw::c_int,
    57 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    84 as std::os::raw::c_int,
    35 as std::os::raw::c_int,
    47 as std::os::raw::c_int,
    62 as std::os::raw::c_int,
    17 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    1 as std::os::raw::c_int,
    99 as std::os::raw::c_int,
    89 as std::os::raw::c_int,
    52 as std::os::raw::c_int,
    6 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    28 as std::os::raw::c_int,
    75 as std::os::raw::c_int,
    94 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    37 as std::os::raw::c_int,
    10 as std::os::raw::c_int,
    23 as std::os::raw::c_int,
    51 as std::os::raw::c_int,
    6 as std::os::raw::c_int,
    48 as std::os::raw::c_int,
    53 as std::os::raw::c_int,
    18 as std::os::raw::c_int,
    74 as std::os::raw::c_int,
    98 as std::os::raw::c_int,
    15 as std::os::raw::c_int,
    27 as std::os::raw::c_int,
    2 as std::os::raw::c_int,
    92 as std::os::raw::c_int,
    23 as std::os::raw::c_int,
    8 as std::os::raw::c_int,
    71 as std::os::raw::c_int,
    76 as std::os::raw::c_int,
    84 as std::os::raw::c_int,
    15 as std::os::raw::c_int,
    52 as std::os::raw::c_int,
    92 as std::os::raw::c_int,
    63 as std::os::raw::c_int,
    81 as std::os::raw::c_int,
    10 as std::os::raw::c_int,
    44 as std::os::raw::c_int,
    10 as std::os::raw::c_int,
    69 as std::os::raw::c_int,
    93 as std::os::raw::c_int,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let len: std::os::raw::c_int = (std::mem::size_of::<[std::os::raw::c_int; 171]>()
        as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong)
        as std::os::raw::c_int;
    let base: std::os::raw::c_int = ((sqrt(
        (8 as std::os::raw::c_int * len + 1 as std::os::raw::c_int) as std::os::raw::c_double,
    ) - 1 as std::os::raw::c_int as std::os::raw::c_double) / 2 as std::os::raw::c_int as std::os::raw::c_double)
        as std::os::raw::c_int;
    let mut step: std::os::raw::c_int = base - 1 as std::os::raw::c_int;
    let mut stepc: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    i = len - base - 1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        tri[i as usize]
            += if tri[(i + step) as usize] > tri[(i + step + 1 as std::os::raw::c_int) as usize]
            {
                tri[(i + step) as usize]
            } else {
                tri[(i + step + 1 as std::os::raw::c_int) as usize]
            };
        stepc += 1;
        if stepc == step {
            step -= 1;
            step;
            stepc = 0 as std::os::raw::c_int;
        }
        i -= 1;
        i;
    }
    printf(
        b"%d\n\0" as *const u8 as *const std::os::raw::c_char,
        tri[0 as std::os::raw::c_int as usize],
    );
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
