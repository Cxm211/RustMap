#![no_std]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {}
fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut degrees: [f64; 33] = [
            0.0f64, 16.87f64, 16.88f64, 33.75f64, 50.62f64, 50.63f64, 67.5f64, 84.37f64, 84.38f64,
            101.25f64, 118.12f64, 118.13f64, 135.0f64, 151.87f64, 151.88f64, 168.75f64, 185.62f64,
            185.63f64, 202.5f64, 219.37f64, 219.38f64, 236.25f64, 253.12f64, 253.13f64, 270.0f64,
            286.87f64, 286.88f64, 303.75f64, 320.62f64, 320.63f64, 337.5f64, 354.37f64, 354.38f64,
        ];
        let mut names : * const i8 =
b"North                 North by east         North-northeast       Northeast by north    Northeast             Northeast by east     East-northeast        East by north         East                  East by south         East-southeast        Southeast by east     Southeast             Southeast by south    South-southeast       South by east         South                 South by west         South-southwest       Southwest by south    Southwest             Southwest by west     West-southwest        West by south         West                  West by north         West-northwest        Northwest by west     Northwest             Northwest by north    North-northwest       North by west         North                 \0"
         as * const u8 as * const i8;
        i = 0;
        while i < 33 {
            j = (0.5f64 + degrees[i as usize] * 32 as f64 / 360 as f64) as i32;
            print!(
                "{:2}  {:.22}  {:6.2}\n",
                j % 32 + 1,
                build_str_from_raw_ptr(names.offset((j % 32 * 22i32) as isize) as *mut u8),
                degrees[i as usize]
            );
            i += 1;
            i;
        }
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
