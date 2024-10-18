#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
// //use ::c2rust_out::*;
extern "C" {
    fn malloc(_: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn exit(_: std::os::raw::c_int) -> !;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn memset(
        _: *mut u8,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> *mut u8;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
}
pub type size_t = std::os::raw::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn best_shuffle(
    mut txt: *const std::os::raw::c_char,
    mut result: *mut std::os::raw::c_char,
) {
    let len: size_t = strlen(txt);
    if len == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        return;
    }
    if len == strlen(result) {} else {
        __assert_fail(
            b"len == strlen(result)\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            16 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 40],
                &[std::os::raw::c_char; 40],
            >(b"void best_shuffle(const char *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_2313: {
        if len == strlen(result) {} else {
            __assert_fail(
                b"len == strlen(result)\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                16 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 40],
                    &[std::os::raw::c_char; 40],
                >(b"void best_shuffle(const char *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut counts: [size_t; 255] = [0; 255];
    memset(
        counts.as_mut_ptr() as *mut u8,
        '\0' as i32,
        ((127 as std::os::raw::c_int * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as std::os::raw::c_ulong)
            .wrapping_mul(std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong),
    );
    let mut fmax: size_t = 0 as std::os::raw::c_int as size_t;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < len {
        counts[*txt.offset(i as isize) as std::os::raw::c_uchar
            as usize] = (counts[*txt.offset(i as isize) as std::os::raw::c_uchar as usize])
            .wrapping_add(1);
        counts[*txt.offset(i as isize) as std::os::raw::c_uchar as usize];
        let fnew: size_t = counts[*txt.offset(i as isize) as std::os::raw::c_uchar as usize];
        if fmax < fnew {
            fmax = fnew;
        }
        i = i.wrapping_add(1);
        i;
    }
    if fmax > 0 as std::os::raw::c_int as std::os::raw::c_ulong && fmax <= len {} else {
        __assert_fail(
            b"fmax > 0 && fmax <= len\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            29 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 40],
                &[std::os::raw::c_char; 40],
            >(b"void best_shuffle(const char *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_2197: {
        if fmax > 0 as std::os::raw::c_int as std::os::raw::c_ulong && fmax <= len {} else {
            __assert_fail(
                b"fmax > 0 && fmax <= len\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                29 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 40],
                    &[std::os::raw::c_char; 40],
                >(b"void best_shuffle(const char *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ndx1: *mut size_t = malloc(
        len.wrapping_mul(std::mem::size_of::<size_t>() as std::os::raw::c_ulong),
    ) as *mut size_t;
    if ndx1.is_null() {
        exit(1 as std::os::raw::c_int);
    }
    let mut ch: size_t = 0 as std::os::raw::c_int as size_t;
    let mut i_0: size_t = 0 as std::os::raw::c_int as size_t;
    while ch
        < (127 as std::os::raw::c_int * 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) as std::os::raw::c_ulong
    {
        if counts[ch as usize] != 0 {
            let mut j: size_t = 0 as std::os::raw::c_int as size_t;
            while j < len {
                if ch == *txt.offset(j as isize) as std::os::raw::c_uchar as std::os::raw::c_ulong {
                    *ndx1.offset(i_0 as isize) = j;
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                }
                j = j.wrapping_add(1);
                j;
            }
        }
        ch = ch.wrapping_add(1);
        ch;
    }
    let mut ndx2: *mut size_t = malloc(
        len.wrapping_mul(std::mem::size_of::<size_t>() as std::os::raw::c_ulong),
    ) as *mut size_t;
    if ndx2.is_null() {
        exit(1 as std::os::raw::c_int);
    }
    let mut i_1: size_t = 0 as std::os::raw::c_int as size_t;
    let mut n: size_t = 0 as std::os::raw::c_int as size_t;
    let mut m: size_t = 0 as std::os::raw::c_int as size_t;
    while i_1 < len {
        *ndx2.offset(i_1 as isize) = *ndx1.offset(n as isize);
        n = (n as std::os::raw::c_ulong).wrapping_add(fmax) as size_t as size_t;
        if n >= len {
            m = m.wrapping_add(1);
            m;
            n = m;
        }
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    let grp: size_t = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_add(
            len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_div(fmax),
        );
    if grp > 0 as std::os::raw::c_int as std::os::raw::c_ulong && grp <= len {} else {
        __assert_fail(
            b"grp > 0 && grp <= len\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            58 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 40],
                &[std::os::raw::c_char; 40],
            >(b"void best_shuffle(const char *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_2002: {
        if grp > 0 as std::os::raw::c_int as std::os::raw::c_ulong && grp <= len {} else {
            __assert_fail(
                b"grp > 0 && grp <= len\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                58 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 40],
                    &[std::os::raw::c_char; 40],
                >(b"void best_shuffle(const char *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let lng: size_t = (1 as std::os::raw::c_int as std::os::raw::c_ulong)
        .wrapping_add(
            len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_rem(fmax),
        );
    if lng > 0 as std::os::raw::c_int as std::os::raw::c_ulong && lng <= len {} else {
        __assert_fail(
            b"lng > 0 && lng <= len\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            62 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 40],
                &[std::os::raw::c_char; 40],
            >(b"void best_shuffle(const char *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_1947: {
        if lng > 0 as std::os::raw::c_int as std::os::raw::c_ulong && lng <= len {} else {
            __assert_fail(
                b"lng > 0 && lng <= len\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                62 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 40],
                    &[std::os::raw::c_char; 40],
                >(b"void best_shuffle(const char *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i_2: size_t = 0 as std::os::raw::c_int as size_t;
    let mut j_0: size_t = 0 as std::os::raw::c_int as size_t;
    while i_2 < fmax {
        let first: size_t = *ndx2.offset(j_0 as isize);
        let glen: size_t = grp
            .wrapping_sub(
                (if i_2 < lng { 0 as std::os::raw::c_int } else { 1 as std::os::raw::c_int })
                    as std::os::raw::c_ulong,
            );
        let mut k: size_t = 1 as std::os::raw::c_int as size_t;
        while k < glen {
            *ndx1
                .offset(
                    j_0.wrapping_add(k).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as isize,
                ) = *ndx2.offset(j_0.wrapping_add(k) as isize);
            k = k.wrapping_add(1);
            k;
        }
        *ndx1
            .offset(
                j_0.wrapping_add(glen).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            ) = first;
        j_0 = (j_0 as std::os::raw::c_ulong).wrapping_add(glen) as size_t as size_t;
        i_2 = i_2.wrapping_add(1);
        i_2;
    }
    *result.offset(len as isize) = '\0' as i32 as std::os::raw::c_char;
    let mut i_3: size_t = 0 as std::os::raw::c_int as size_t;
    while i_3 < len {
        *result
            .offset(
                *ndx2.offset(i_3 as isize) as isize,
            ) = *txt.offset(*ndx1.offset(i_3 as isize) as isize);
        i_3 = i_3.wrapping_add(1);
        i_3;
    }
    free(ndx1 as *mut u8);
    free(ndx2 as *mut u8);
}
#[no_mangle]
pub unsafe extern "C" fn display(
    mut txt1: *const std::os::raw::c_char,
    mut txt2: *const std::os::raw::c_char,
) {
    let len: size_t = strlen(txt1);
    if len == strlen(txt2) {} else {
        __assert_fail(
            b"len == strlen(txt2)\0" as *const u8 as *const std::os::raw::c_char,
            b"main.c\0" as *const u8 as *const std::os::raw::c_char,
            85 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<
                &[u8; 41],
                &[std::os::raw::c_char; 41],
            >(b"void display(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_2427: {
        if len == strlen(txt2) {} else {
            __assert_fail(
                b"len == strlen(txt2)\0" as *const u8 as *const std::os::raw::c_char,
                b"main.c\0" as *const u8 as *const std::os::raw::c_char,
                85 as std::os::raw::c_int as std::os::raw::c_uint,
                (*std::mem::transmute::<
                    &[u8; 41],
                    &[std::os::raw::c_char; 41],
                >(b"void display(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut score: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < len {
        if *txt1.offset(i as isize) as std::os::raw::c_int
            == *txt2.offset(i as isize) as std::os::raw::c_int
        {
            score += 1;
            score;
        }
        i = i.wrapping_add(1);
        i;
    }
    printf(b"%s, %s, (%u)\n\0" as *const u8 as *const std::os::raw::c_char, txt1, txt2, score);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut data: [*const std::os::raw::c_char; 9] = [
        b"abracadabra\0" as *const u8 as *const std::os::raw::c_char,
        b"seesaw\0" as *const u8 as *const std::os::raw::c_char,
        b"elk\0" as *const u8 as *const std::os::raw::c_char,
        b"grrrrrr\0" as *const u8 as *const std::os::raw::c_char,
        b"up\0" as *const u8 as *const std::os::raw::c_char,
        b"a\0" as *const u8 as *const std::os::raw::c_char,
        b"aabbbbaa\0" as *const u8 as *const std::os::raw::c_char,
        b"\0" as *const u8 as *const std::os::raw::c_char,
        b"xxxxx\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let data_len: size_t = (std::mem::size_of::<[*const std::os::raw::c_char; 9]>()
        as std::os::raw::c_ulong)
        .wrapping_div(std::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong);
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < data_len {
        let shuf_len: size_t = (strlen(data[i as usize]))
            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        let vla = shuf_len as usize;
        let mut shuf: Vec::<std::os::raw::c_char> = ::std::vec::from_elem(0, vla);
        memset(
            shuf.as_mut_ptr() as *mut u8,
            0xff as std::os::raw::c_int,
            (vla * std::mem::size_of::<std::os::raw::c_char>()) as std::os::raw::c_ulong,
        );
        *shuf
            .as_mut_ptr()
            .offset(
                shuf_len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
            ) = '\0' as i32 as std::os::raw::c_char;
        best_shuffle(data[i as usize], shuf.as_mut_ptr());
        display(data[i as usize], shuf.as_mut_ptr());
        i = i.wrapping_add(1);
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
