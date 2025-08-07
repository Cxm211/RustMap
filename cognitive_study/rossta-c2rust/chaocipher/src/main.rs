#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
}
pub type size_t = std::os::raw::c_ulong;
pub type bool_0 = std::os::raw::c_int;
pub type cmode = std::os::raw::c_uint;
pub const DECRYPT: cmode = 1;
pub const ENCRYPT: cmode = 0;
#[no_mangle]
pub static mut l_alphabet: *const std::os::raw::c_char = b"HXUCZVAMDSLKPEFJRIGTWOBNYQ\0"
    as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut r_alphabet: *const std::os::raw::c_char = b"PTLNBQDEOYSFAVZKGJRIHWXUMC\0"
    as *const u8 as *const std::os::raw::c_char;
#[no_mangle]
pub unsafe extern "C" fn chao(
    mut in_0: *const std::os::raw::c_char,
    mut out: *mut std::os::raw::c_char,
    mut mode: cmode,
    mut show_steps: bool_0,
) {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut store: std::os::raw::c_char = 0;
    let mut len: size_t = strlen(in_0);
    let mut left: [std::os::raw::c_char; 27] = [0; 27];
    let mut right: [std::os::raw::c_char; 27] = [0; 27];
    let mut temp: [std::os::raw::c_char; 27] = [0; 27];
    strcpy(left.as_mut_ptr(), l_alphabet);
    strcpy(right.as_mut_ptr(), r_alphabet);
    temp[26 as std::os::raw::c_int as usize] = '\0' as i32 as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int;
    while (i as std::os::raw::c_ulong) < len {
        if show_steps != 0 {
            printf(
                b"%s  %s\n\0" as *const u8 as *const std::os::raw::c_char,
                left.as_mut_ptr(),
                right.as_mut_ptr(),
            );
        }
        if mode as std::os::raw::c_uint == ENCRYPT as std::os::raw::c_int as std::os::raw::c_uint {
            index = (strchr(right.as_mut_ptr(), *in_0.offset(i as isize) as std::os::raw::c_int))
                .offset_from(right.as_mut_ptr()) as std::os::raw::c_long as std::os::raw::c_int;
            *out.offset(i as isize) = left[index as usize];
        } else {
            index = (strchr(left.as_mut_ptr(), *in_0.offset(i as isize) as std::os::raw::c_int))
                .offset_from(left.as_mut_ptr()) as std::os::raw::c_long as std::os::raw::c_int;
            *out.offset(i as isize) = right[index as usize];
        }
        if i as std::os::raw::c_ulong == len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
            break;
        }
        j = index;
        while j < 26 as std::os::raw::c_int {
            temp[(j - index) as usize] = left[j as usize];
            j += 1;
            j;
        }
        j = 0 as std::os::raw::c_int;
        while j < index {
            temp[(26 as std::os::raw::c_int - index + j) as usize] = left[j as usize];
            j += 1;
            j;
        }
        store = temp[1 as std::os::raw::c_int as usize];
        j = 2 as std::os::raw::c_int;
        while j < 14 as std::os::raw::c_int {
            temp[(j - 1 as std::os::raw::c_int) as usize] = temp[j as usize];
            j += 1;
            j;
        }
        temp[13 as std::os::raw::c_int as usize] = store;
        strcpy(left.as_mut_ptr(), temp.as_mut_ptr());
        j = index;
        while j < 26 as std::os::raw::c_int {
            temp[(j - index) as usize] = right[j as usize];
            j += 1;
            j;
        }
        j = 0 as std::os::raw::c_int;
        while j < index {
            temp[(26 as std::os::raw::c_int - index + j) as usize] = right[j as usize];
            j += 1;
            j;
        }
        store = temp[0 as std::os::raw::c_int as usize];
        j = 1 as std::os::raw::c_int;
        while j < 26 as std::os::raw::c_int {
            temp[(j - 1 as std::os::raw::c_int) as usize] = temp[j as usize];
            j += 1;
            j;
        }
        temp[25 as std::os::raw::c_int as usize] = store;
        store = temp[2 as std::os::raw::c_int as usize];
        j = 3 as std::os::raw::c_int;
        while j < 14 as std::os::raw::c_int {
            temp[(j - 1 as std::os::raw::c_int) as usize] = temp[j as usize];
            j += 1;
            j;
        }
        temp[13 as std::os::raw::c_int as usize] = store;
        strcpy(right.as_mut_ptr(), temp.as_mut_ptr());
        i += 1;
        i;
    }
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut plain_text: *const std::os::raw::c_char = b"WELLDONEISBETTERTHANWELLSAID\0"
        as *const u8 as *const std::os::raw::c_char;
    let mut cipher_text: *mut std::os::raw::c_char = malloc(
        (strlen(plain_text)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    let mut plain_text2: *mut std::os::raw::c_char = malloc(
        (strlen(plain_text)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
    ) as *mut std::os::raw::c_char;
    printf(
        b"The original plaintext is : %s\n\0" as *const u8 as *const std::os::raw::c_char,
        plain_text,
    );
    printf(
        b"\nThe left and right alphabets after each permutation during encryption are :\n\n\0"
            as *const u8 as *const std::os::raw::c_char,
    );
    chao(plain_text, cipher_text, ENCRYPT, 1 as std::os::raw::c_int);
    printf(
        b"\nThe ciphertext is : %s\n\0" as *const u8 as *const std::os::raw::c_char,
        cipher_text,
    );
    chao(cipher_text, plain_text2, DECRYPT, 0 as std::os::raw::c_int);
    printf(
        b"\nThe recovered plaintext is : %s\n\0" as *const u8 as *const std::os::raw::c_char,
        plain_text2,
    );
    free(cipher_text as *mut std::os::raw::c_void);
    free(plain_text2 as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
