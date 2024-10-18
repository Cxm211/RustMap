#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn sleep(__seconds: std::os::raw::c_uint) -> std::os::raw::c_uint;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut u8) -> *mut u8,
        >,
        __arg: *mut u8,
    ) -> std::os::raw::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> std::os::raw::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> std::os::raw::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: * mut crate::__pthread_internal_list,
    pub __next: * mut crate::__pthread_internal_list,
}
impl std::default::Default for __pthread_internal_list {
    fn default() -> Self {
        __pthread_internal_list {
        __prev: core::ptr::null_mut(),
        __next: core::ptr::null_mut()
        }
    }
}

pub type __pthread_list_t = crate::__pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: std::os::raw::c_int,
    pub __count: std::os::raw::c_uint,
    pub __owner: std::os::raw::c_int,
    pub __nusers: std::os::raw::c_uint,
    pub __kind: std::os::raw::c_int,
    pub __spins: std::os::raw::c_short,
    pub __elision: std::os::raw::c_short,
    pub __list: crate::__pthread_internal_list,
}
impl std::default::Default for __pthread_mutex_s {
    fn default() -> Self {
        __pthread_mutex_s {
        __lock: std::os::raw::c_int::default(),
        __count: std::os::raw::c_uint::default(),
        __owner: std::os::raw::c_int::default(),
        __nusers: std::os::raw::c_uint::default(),
        __kind: std::os::raw::c_int::default(),
        __spins: std::os::raw::c_short::default(),
        __elision: std::os::raw::c_short::default(),
        __list: crate::__pthread_internal_list::default()
        }
    }
}

pub type pthread_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [std::os::raw::c_char; 4],
    pub __align: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [std::os::raw::c_char; 56],
    pub __align: std::os::raw::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [std::os::raw::c_char; 40],
    pub __align: std::os::raw::c_long,
}
#[no_mangle]
pub static mut bucket_mutex: [pthread_mutex_t; 15] = [pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
}; 15];
#[no_mangle]
pub static mut buckets: [std::os::raw::c_int; 15] = [0; 15];
#[no_mangle]
pub static mut equalizer: pthread_t = 0;
#[no_mangle]
pub static mut randomizer: pthread_t = 0;
#[no_mangle]
pub unsafe extern "C" fn transfer_value(
    mut from: std::os::raw::c_int,
    mut to: std::os::raw::c_int,
    mut howmuch: std::os::raw::c_int,
) {
    let mut swapped: bool = 0 as std::os::raw::c_int != 0;
    if from == to || howmuch < 0 as std::os::raw::c_int || from < 0 as std::os::raw::c_int
        || to < 0 as std::os::raw::c_int || from >= 15 as std::os::raw::c_int || to >= 15 as std::os::raw::c_int
    {
        return;
    }
    if from > to {
        let mut temp1: std::os::raw::c_int = from;
        from = to;
        to = temp1;
        swapped = 1 as std::os::raw::c_int != 0;
        howmuch = -howmuch;
    }
    pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
    pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
    if howmuch > buckets[from as usize] && !swapped {
        howmuch = buckets[from as usize];
    }
    if -howmuch > buckets[to as usize] && swapped as std::os::raw::c_int != 0 {
        howmuch = -buckets[to as usize];
    }
    buckets[from as usize] -= howmuch;
    buckets[to as usize] += howmuch;
    pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
    pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
}
#[no_mangle]
pub unsafe extern "C" fn print_buckets() {
    let mut i: std::os::raw::c_int = 0;
    let mut sum: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        printf(b"%3d \0" as *const u8 as *const std::os::raw::c_char, buckets[i as usize]);
        sum += buckets[i as usize];
        i += 1;
        i;
    }
    printf(b"= %d\n\0" as *const u8 as *const std::os::raw::c_char, sum);
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn equalizer_start(mut t: *mut u8) -> *mut u8 {
    loop {
        let mut b1: std::os::raw::c_int = rand() % 15 as std::os::raw::c_int;
        let mut b2: std::os::raw::c_int = rand() % 15 as std::os::raw::c_int;
        let mut diff: std::os::raw::c_int = buckets[b1 as usize] - buckets[b2 as usize];
        if diff < 0 as std::os::raw::c_int {
            transfer_value(b2, b1, -diff / 2 as std::os::raw::c_int);
        } else {
            transfer_value(b1, b2, diff / 2 as std::os::raw::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn randomizer_start(
    mut t: *mut u8,
) -> *mut u8 {
    loop {
        let mut b1: std::os::raw::c_int = rand() % 15 as std::os::raw::c_int;
        let mut b2: std::os::raw::c_int = rand() % 15 as std::os::raw::c_int;
        let mut diff: std::os::raw::c_int = rand() % (buckets[b1 as usize] + 1 as std::os::raw::c_int);
        transfer_value(b1, b2, diff);
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut total: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        pthread_mutex_init(
            &mut *bucket_mutex.as_mut_ptr().offset(i as isize),
            0 as *const pthread_mutexattr_t,
        );
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        buckets[i as usize] = rand() % 100 as std::os::raw::c_int;
        total += buckets[i as usize];
        printf(b"%3d \0" as *const u8 as *const std::os::raw::c_char, buckets[i as usize]);
        i += 1;
        i;
    }
    printf(b"= %d\n\0" as *const u8 as *const std::os::raw::c_char, total);
    pthread_create(
        &mut equalizer,
        0 as *const pthread_attr_t,
        Some(
            equalizer_start
                as unsafe extern "C" fn(*mut u8) -> *mut u8,
        ),
        0 as *mut u8,
    );
    pthread_create(
        &mut randomizer,
        0 as *const pthread_attr_t,
        Some(
            randomizer_start
                as unsafe extern "C" fn(*mut u8) -> *mut u8,
        ),
        0 as *mut u8,
    );
    let mut i_0: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while i_0 < 2 as std::os::raw::c_int {
        sleep(1 as std::os::raw::c_int as std::os::raw::c_uint);
        print_buckets();
        i_0 += 1;
        i_0;
    }
    i = 0 as std::os::raw::c_int;
    while i < 15 as std::os::raw::c_int {
        pthread_mutex_destroy(bucket_mutex.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
