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
//use c2rust_out::*;
extern "C" {
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn meanAngle(mut angles: *mut f64, mut size: i32) -> f64 {
    unsafe {
        let mut y_part: f64 = 0 as f64;
        let mut x_part: f64 = 0 as f64;
        let mut i: i32 = 0;
        i = 0;
        while i < size {
            x_part += cos(*angles.offset(i as isize) * 3.14159265358979323846f64 / 180 as f64);
            y_part += sin(*angles.offset(i as isize) * 3.14159265358979323846f64 / 180 as f64);
            i += 1;
            i;
        }
        return atan2(y_part / size as f64, x_part / size as f64) * 180 as f64
            / 3.14159265358979323846f64;
    }
}

fn main_0() -> i32 {
    let mut angleSet1: [f64; 2] = [350 as f64, 10 as f64];
    let mut angleSet2: [f64; 4] = [90 as f64, 180 as f64, 270 as f64, 360 as f64];
    let mut angleSet3: [f64; 3] = [10 as f64, 20 as f64, 30 as f64];
    print!(
        "\nMean Angle for 1st set : {} degrees",
        meanAngle(angleSet1.as_mut_ptr(), 2)
    );
    print!(
        "\nMean Angle for 2nd set : {} degrees",
        meanAngle(angleSet2.as_mut_ptr(), 4)
    );
    print!(
        "\nMean Angle for 3rd set : {} degrees\n",
        meanAngle(angleSet3.as_mut_ptr(), 3)
    );
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
