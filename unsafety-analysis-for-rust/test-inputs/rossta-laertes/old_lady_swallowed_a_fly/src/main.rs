#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
static mut animals: [*const std::os::raw::c_char; 8] = [
    b"fly\0" as *const u8 as *const std::os::raw::c_char,
    b"spider\0" as *const u8 as *const std::os::raw::c_char,
    b"bird\0" as *const u8 as *const std::os::raw::c_char,
    b"cat\0" as *const u8 as *const std::os::raw::c_char,
    b"dog\0" as *const u8 as *const std::os::raw::c_char,
    b"goat\0" as *const u8 as *const std::os::raw::c_char,
    b"cow\0" as *const u8 as *const std::os::raw::c_char,
    b"horse\0" as *const u8 as *const std::os::raw::c_char,
];
static mut verses: [*const std::os::raw::c_char; 8] = [
    b"I don't know why she swallowed that fly.\nPerhaps she'll die\n\0" as *const u8
        as *const std::os::raw::c_char,
    b"That wiggled and jiggled and tickled inside her\0" as *const u8
        as *const std::os::raw::c_char,
    b"How absurd, to swallow a bird\0" as *const u8 as *const std::os::raw::c_char,
    b"Imagine that. She swallowed a cat\0" as *const u8 as *const std::os::raw::c_char,
    b"What a hog to swallow a dog\0" as *const u8 as *const std::os::raw::c_char,
    b"She just opened her throat and swallowed that goat\0" as *const u8
        as *const std::os::raw::c_char,
    b"I don't know how she swallowed that cow\0" as *const u8 as *const std::os::raw::c_char,
    b"She's dead of course\0" as *const u8 as *const std::os::raw::c_char,
];
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i
        < (std::mem::size_of::<[*const std::os::raw::c_char; 8]>() as std::os::raw::c_ulong)
            .wrapping_div(std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong)
    {
        printf(
            b"There was an old lady who swallowed a %s\n%s\n\0" as *const u8
                as *const std::os::raw::c_char,
            animals[i as usize],
            verses[i as usize],
        );
        let mut j: size_t = i;
        while j > 0 as std::os::raw::c_int as std::os::raw::c_ulong
            && i
                < (std::mem::size_of::<[*const std::os::raw::c_char; 8]>() as std::os::raw::c_ulong)
                    .wrapping_div(
                        std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong,
                    )
                    .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        {
            printf(
                b"She swallowed the %s to catch the %s\n\0" as *const u8
                    as *const std::os::raw::c_char,
                animals[j as usize],
                animals[j.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as usize],
            );
            if j == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                printf(
                    b"%s\n\0" as *const u8 as *const std::os::raw::c_char,
                    verses[0 as std::os::raw::c_int as usize],
                );
            }
            j = j.wrapping_sub(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
