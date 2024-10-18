#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn puts(__s: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut u8;
    fn free(_: *mut u8);
    fn abort() -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __time_t = std::os::raw::c_long;
pub type time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub payload: std::os::raw::c_int,
    pub height: std::os::raw::c_int,
    pub kid: [* mut crate::node; 2],
}
impl std::default::Default for node {
    fn default() -> Self {
        node {
        payload: std::os::raw::c_int::default(),
        height: std::os::raw::c_int::default(),
        kid: [core::ptr::null_mut(); 2]
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct trunk {
    pub prev: * mut crate::trunk,
    pub str_0: * mut std::os::raw::c_char,
}
impl std::default::Default for trunk {
    fn default() -> Self {
        trunk {
        prev: core::ptr::null_mut(),
        str_0: 0 as * mut std::os::raw::c_char
        }
    }
}

#[no_mangle]
pub static mut dummy: node = unsafe {
    {
        let mut init = node {
            payload: 0 as std::os::raw::c_int,
            height: 0 as std::os::raw::c_int,
            kid: [&dummy as *const node as *mut node, &dummy as *const node as *mut node],
        };
        init
    }
};
#[no_mangle]
pub static mut nnil: *mut node = unsafe { &dummy as *const node as *mut node };
#[no_mangle]
pub unsafe extern "C" fn new_node(mut value: std::os::raw::c_int) -> *mut node {
    let mut n: *mut node = calloc(
        1 as std::os::raw::c_int as std::os::raw::c_ulong,
        std::mem::size_of::<node>() as std::os::raw::c_ulong,
    ) as *mut node;
    *n = {
        let mut init = node {
            payload: value,
            height: 1 as std::os::raw::c_int,
            kid: [nnil, nnil],
        };
        init
    };
    return n;
}
#[no_mangle]
pub extern "C" fn max(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn set_height(mut n: *mut node) {
    (*n)
        .height = 1 as std::os::raw::c_int
        + max(
            (*(*n).kid[0 as std::os::raw::c_int as usize]).height,
            (*(*n).kid[1 as std::os::raw::c_int as usize]).height,
        );
}
#[no_mangle]
pub unsafe extern "C" fn ballance(mut n: *mut node) -> std::os::raw::c_int {
    return (*(*n).kid[0 as std::os::raw::c_int as usize]).height
        - (*(*n).kid[1 as std::os::raw::c_int as usize]).height;
}
#[no_mangle]
pub unsafe extern "C" fn rotate<'a1>(
    mut rootp: Option<&'a1 mut * mut crate::node>,
    mut dir: std::os::raw::c_int,
) -> * mut crate::node {
    let mut old_r: *mut node = *borrow_mut(&mut rootp).unwrap();
    let mut new_r: *mut node = (*old_r).kid[dir as usize];
    *borrow_mut(&mut rootp).unwrap() = new_r;
    if nnil == *borrow(& rootp).unwrap() {
        free(old_r as *mut u8);
    } else {
        (*old_r).kid[dir as usize] = (*new_r).kid[(dir == 0) as std::os::raw::c_int as usize];
        set_height(old_r);
        (*new_r).kid[(dir == 0) as std::os::raw::c_int as usize] = old_r;
    }
    return new_r;
}
#[no_mangle]
pub unsafe extern "C" fn adjust_balance<'a1>(mut rootp: Option<&'a1 mut * mut crate::node>) {
    let mut root: *mut node = *borrow_mut(&mut rootp).unwrap();
    let mut b: std::os::raw::c_int = ballance(root) / 2 as std::os::raw::c_int;
    if b != 0 {
        let mut dir: std::os::raw::c_int = (1 as std::os::raw::c_int - b) / 2 as std::os::raw::c_int;
        if ballance((*root).kid[dir as usize]) == -b {
            rotate(
                Some(&mut *((*root).kid).as_mut_ptr().offset(dir as isize)),
                (dir == 0) as std::os::raw::c_int,
            );
        }
        root = rotate(borrow_mut(&mut rootp), dir);
    }
    if root != nnil {
        set_height(root);
    }
}
#[no_mangle]
pub unsafe extern "C" fn query(
    mut root: *mut node,
    mut value: std::os::raw::c_int,
) -> *mut node {
    return if root == nnil {
        0 as *mut node
    } else if (*root).payload == value {
        root
    } else {
        query((*root).kid[(value > (*root).payload) as std::os::raw::c_int as usize], value)
    };
}
#[no_mangle]
pub unsafe extern "C" fn insert<'a1>(mut rootp: Option<&'a1 mut * mut crate::node>, mut value: std::os::raw::c_int) {
    let mut root: *mut node = *borrow_mut(&mut rootp).unwrap();
    if root == nnil {
        *borrow_mut(&mut rootp).unwrap() = new_node(value);
    } else if value != (*root).payload {
        insert(
            Some(&mut *((*root).kid)
                .as_mut_ptr()
                .offset((value > (*root).payload) as std::os::raw::c_int as isize)),
            value,
        );
        adjust_balance(borrow_mut(&mut rootp));
    }
}
#[no_mangle]
pub unsafe extern "C" fn delete<'a1>(mut rootp: Option<&'a1 mut * mut crate::node>, mut value: std::os::raw::c_int) {
    let mut root: *mut node = *borrow_mut(&mut rootp).unwrap();
    if root == nnil {
        return;
    }
    if (*root).payload == value {
        root = rotate(borrow_mut(&mut rootp), (ballance(root) < 0 as std::os::raw::c_int) as std::os::raw::c_int);
        if nnil == root {
            return;
        }
    }
    delete(
        Some(&mut *((*root).kid)
            .as_mut_ptr()
            .offset((value > (*root).payload) as std::os::raw::c_int as isize)),
        value,
    );
    adjust_balance(borrow_mut(&mut rootp));
}
#[no_mangle]
pub unsafe extern "C" fn show_trunks(mut p: *mut trunk) {
    if p.is_null() {
        return;
    }
    show_trunks((*p).prev);
    printf(b"%s\0" as *const u8 as *const std::os::raw::c_char, (*p).str_0);
}
#[no_mangle]
pub unsafe extern "C" fn show_tree(
    mut root: *mut node,
    mut prev: *mut trunk,
    mut is_left: std::os::raw::c_int,
) {
    if root == nnil {
        return;
    }
    let mut this_disp: trunk = {
        let mut init = trunk {
            prev: prev,
            str_0: b"    \0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
        };
        init
    };
    let mut prev_str: *mut std::os::raw::c_char = this_disp.str_0;
    show_tree((*root).kid[0 as std::os::raw::c_int as usize], &mut this_disp, 1 as std::os::raw::c_int);
    if prev.is_null() {
        this_disp
            .str_0 = b"---\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    } else if is_left != 0 {
        this_disp
            .str_0 = b".--\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
        prev_str = b"   |\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    } else {
        this_disp
            .str_0 = b"`--\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
        (*prev).str_0 = prev_str;
    }
    show_trunks(&mut this_disp);
    printf(b"%d\n\0" as *const u8 as *const std::os::raw::c_char, (*root).payload);
    if !prev.is_null() {
        (*prev).str_0 = prev_str;
    }
    this_disp.str_0 = b"   |\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
    show_tree((*root).kid[1 as std::os::raw::c_int as usize], &mut this_disp, 0 as std::os::raw::c_int);
    if prev.is_null() {
        puts(b"\0" as *const u8 as *const std::os::raw::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn verify(mut p: *mut node) -> std::os::raw::c_int {
    if p == nnil {
        return 1 as std::os::raw::c_int;
    }
    let mut h0: std::os::raw::c_int = (*(*p).kid[0 as std::os::raw::c_int as usize]).height;
    let mut h1: std::os::raw::c_int = (*(*p).kid[1 as std::os::raw::c_int as usize]).height;
    let mut b: std::os::raw::c_int = h0 - h1;
    if (*p).height != 1 as std::os::raw::c_int + max(h0, h1) || b < -(1 as std::os::raw::c_int)
        || b > 1 as std::os::raw::c_int
    {
        printf(
            b"node %d bad, balance %d\n\0" as *const u8 as *const std::os::raw::c_char,
            (*p).payload,
            b,
        );
        show_tree(p, core::ptr::null_mut(), 0 as std::os::raw::c_int);
        abort();
    }
    return (verify((*p).kid[0 as std::os::raw::c_int as usize]) != 0
        && verify((*p).kid[1 as std::os::raw::c_int as usize]) != 0) as std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut x: std::os::raw::c_int = 0;
    let mut root: *mut node = nnil;
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    x = 0 as std::os::raw::c_int;
    while x < 10 as std::os::raw::c_int * 32 as std::os::raw::c_int {
        if rand() & 1 as std::os::raw::c_int != 0 {
            insert(Some(&mut root), rand() % 32 as std::os::raw::c_int);
        } else {
            delete(Some(&mut root), rand() % 32 as std::os::raw::c_int);
        }
        verify(root);
        x += 1;
        x;
    }
    puts(b"Tree is:\0" as *const u8 as *const std::os::raw::c_char);
    show_tree(root, core::ptr::null_mut(), 0 as std::os::raw::c_int);
    puts(b"\nQuerying values:\0" as *const u8 as *const std::os::raw::c_char);
    x = 0 as std::os::raw::c_int;
    while x < 32 as std::os::raw::c_int {
        let mut p: *mut node = query(root, x);
        if !p.is_null() {
            printf(
                b"%2d found: %p %d\n\0" as *const u8 as *const std::os::raw::c_char,
                x,
                p,
                (*p).payload,
            );
        }
        x += 1;
        x;
    }
    x = 0 as std::os::raw::c_int;
    while x < 32 as std::os::raw::c_int {
        delete(Some(&mut root), x);
        verify(root);
        x += 1;
        x;
    }
    puts(b"\nAfter deleting all values, tree is:\0" as *const u8 as *const std::os::raw::c_char);
    show_tree(root, core::ptr::null_mut(), 0 as std::os::raw::c_int);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
