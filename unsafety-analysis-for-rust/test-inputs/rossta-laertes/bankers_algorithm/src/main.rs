#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut curr: [[std::os::raw::c_int; 5]; 5] = [[0; 5]; 5];
    let mut max_claim: [[std::os::raw::c_int; 5]; 5] = [[0; 5]; 5];
    let mut avl: [std::os::raw::c_int; 5] = [0; 5];
    let mut alloc: [std::os::raw::c_int; 5] = [
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
    ];
    let mut max_res: [std::os::raw::c_int; 5] = [0; 5];
    let mut running: [std::os::raw::c_int; 5] = [0; 5];
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut exec: std::os::raw::c_int = 0;
    let mut r: std::os::raw::c_int = 0;
    let mut p: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut safe: bool = 0 as std::os::raw::c_int != 0;
    printf(b"\nEnter the number of resources: \0" as *const u8 as *const std::os::raw::c_char);
    scanf(b"%d\0" as *const u8 as *const std::os::raw::c_char, &mut r as *mut std::os::raw::c_int);
    printf(b"\nEnter the number of processes: \0" as *const u8 as *const std::os::raw::c_char);
    scanf(b"%d\0" as *const u8 as *const std::os::raw::c_char, &mut p as *mut std::os::raw::c_int);
    i = 0 as std::os::raw::c_int;
    while i < p {
        running[i as usize] = 1 as std::os::raw::c_int;
        count += 1;
        count;
        i += 1;
        i;
    }
    printf(b"\nEnter Claim Vector: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < r {
        scanf(
            b"%d\0" as *const u8 as *const std::os::raw::c_char,
            &mut *max_res.as_mut_ptr().offset(i as isize) as *mut std::os::raw::c_int,
        );
        i += 1;
        i;
    }
    printf(b"\nEnter Allocated Resource Table: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < p {
        j = 0 as std::os::raw::c_int;
        while j < r {
            scanf(
                b"%d\0" as *const u8 as *const std::os::raw::c_char,
                &mut *(*curr.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(j as isize) as *mut std::os::raw::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printf(b"\nEnter Maximum Claim table: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < p {
        j = 0 as std::os::raw::c_int;
        while j < r {
            scanf(
                b"%d\0" as *const u8 as *const std::os::raw::c_char,
                &mut *(*max_claim.as_mut_ptr().offset(i as isize))
                    .as_mut_ptr()
                    .offset(j as isize) as *mut std::os::raw::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printf(b"\nThe Claim Vector is: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < r {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, max_res[i as usize]);
        i += 1;
        i;
    }
    printf(b"\nThe Allocated Resource Table:\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < p {
        j = 0 as std::os::raw::c_int;
        while j < r {
            printf(
                b"\t%d\0" as *const u8 as *const std::os::raw::c_char,
                curr[i as usize][j as usize],
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    printf(b"\nThe Maximum Claim Table:\n\0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < p {
        j = 0 as std::os::raw::c_int;
        while j < r {
            printf(
                b"\t%d\0" as *const u8 as *const std::os::raw::c_char,
                max_claim[i as usize][j as usize],
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < p {
        j = 0 as std::os::raw::c_int;
        while j < r {
            alloc[j as usize] += curr[i as usize][j as usize];
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printf(b"\nAllocated resources: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < r {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, alloc[i as usize]);
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < r {
        avl[i as usize] = max_res[i as usize] - alloc[i as usize];
        i += 1;
        i;
    }
    printf(b"\nAvailable resources: \0" as *const u8 as *const std::os::raw::c_char);
    i = 0 as std::os::raw::c_int;
    while i < r {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, avl[i as usize]);
        i += 1;
        i;
    }
    printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
    while count != 0 as std::os::raw::c_int {
        safe = 0 as std::os::raw::c_int != 0;
        i = 0 as std::os::raw::c_int;
        while i < p {
            if running[i as usize] != 0 {
                exec = 1 as std::os::raw::c_int;
                j = 0 as std::os::raw::c_int;
                while j < r {
                    if max_claim[i as usize][j as usize] - curr[i as usize][j as usize]
                        > avl[j as usize]
                    {
                        exec = 0 as std::os::raw::c_int;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                if exec != 0 {
                    printf(
                        b"\nProcess%d is executing.\n\0" as *const u8
                            as *const std::os::raw::c_char,
                        i + 1 as std::os::raw::c_int,
                    );
                    running[i as usize] = 0 as std::os::raw::c_int;
                    count -= 1;
                    count;
                    safe = 1 as std::os::raw::c_int != 0;
                    j = 0 as std::os::raw::c_int;
                    while j < r {
                        avl[j as usize] += curr[i as usize][j as usize];
                        j += 1;
                        j;
                    }
                    break;
                }
            }
            i += 1;
            i;
        }
        if !safe {
            printf(
                b"\nThe processes are in unsafe state.\0" as *const u8
                    as *const std::os::raw::c_char,
            );
            break;
        } else {
            if safe {
                printf(
                    b"\nThe process is in safe state.\0" as *const u8
                        as *const std::os::raw::c_char,
                );
            }
            printf(b"\nAvailable vector: \0" as *const u8 as *const std::os::raw::c_char);
            i = 0 as std::os::raw::c_int;
            while i < r {
                printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, avl[i as usize]);
                i += 1;
                i;
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
