#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
// //use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn fgets(
        __s: *mut std::os::raw::c_char,
        __n: std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut std::os::raw::c_char;
    fn __ctype_b_loc() -> *mut *const std::os::raw::c_ushort;
    fn rand() -> std::os::raw::c_int;
    fn srand(__seed: std::os::raw::c_uint);
    fn exit(_: std::os::raw::c_int) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> std::os::raw::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: std::os::raw::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
pub type __time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut u8,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut u8,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
impl std::default::Default for _IO_FILE {
    fn default() -> Self {
        _IO_FILE {
        _flags: std::os::raw::c_int::default(),
        _IO_read_ptr: 0 as * mut std::os::raw::c_char,
        _IO_read_end: 0 as * mut std::os::raw::c_char,
        _IO_read_base: 0 as * mut std::os::raw::c_char,
        _IO_write_base: 0 as * mut std::os::raw::c_char,
        _IO_write_ptr: 0 as * mut std::os::raw::c_char,
        _IO_write_end: 0 as * mut std::os::raw::c_char,
        _IO_buf_base: 0 as * mut std::os::raw::c_char,
        _IO_buf_end: 0 as * mut std::os::raw::c_char,
        _IO_save_base: 0 as * mut std::os::raw::c_char,
        _IO_backup_base: 0 as * mut std::os::raw::c_char,
        _IO_save_end: 0 as * mut std::os::raw::c_char,
        _markers: 0 as * mut crate::_IO_marker,
        _chain: core::ptr::null_mut(),
        _fileno: std::os::raw::c_int::default(),
        _flags2: std::os::raw::c_int::default(),
        _old_offset: std::os::raw::c_long::default(),
        _cur_column: std::os::raw::c_ushort::default(),
        _vtable_offset: std::os::raw::c_schar::default(),
        _shortbuf: [std::os::raw::c_char::default(); 1],
        _lock: 0 as * mut u8,
        _offset: std::os::raw::c_long::default(),
        _codecvt: 0 as * mut crate::_IO_codecvt,
        _wide_data: 0 as * mut crate::_IO_wide_data,
        _freeres_list: core::ptr::null_mut(),
        _freeres_buf: 0 as * mut u8,
        __pad5: std::os::raw::c_ulong::default(),
        _mode: std::os::raw::c_int::default(),
        _unused2: [std::os::raw::c_char::default(); 20]
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = crate::_IO_FILE;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type time_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::os::raw::c_ulong; 16],
}
impl std::default::Default for __sigset_t {
    fn default() -> Self {
        __sigset_t {
        __val: [std::os::raw::c_ulong::default(); 16]
        }
    }
}

pub type __jmp_buf = [std::os::raw::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
impl std::default::Default for __jmp_buf_tag {
    fn default() -> Self {
        __jmp_buf_tag {
        __jmpbuf: [std::os::raw::c_long::default(); 8],
        __mask_was_saved: std::os::raw::c_int::default(),
        __saved_mask: crate::__sigset_t::default()
        }
    }
}

pub type jmp_buf = [crate::__jmp_buf_tag; 1];
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const OP_DIV: C2RustUnnamed_0 = 5;
pub const OP_MUL: C2RustUnnamed_0 = 4;
pub const OP_SUB: C2RustUnnamed_0 = 3;
pub const OP_ADD: C2RustUnnamed_0 = 2;
pub const OP_NUM: C2RustUnnamed_0 = 1;
pub const OP_NONE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expr_t {
    pub op: std::os::raw::c_int,
    pub val: std::os::raw::c_int,
    pub used: std::os::raw::c_int,
    pub left: * mut crate::expr_t,
    pub right: * mut crate::expr_t,
}
impl std::default::Default for expr_t {
    fn default() -> Self {
        expr_t {
        op: std::os::raw::c_int::default(),
        val: std::os::raw::c_int::default(),
        used: std::os::raw::c_int::default(),
        left: core::ptr::null_mut(),
        right: core::ptr::null_mut()
        }
    }
}

pub type expr = * mut crate::expr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac_t {
    pub denom: std::os::raw::c_int,
    pub num: std::os::raw::c_int,
}
impl std::default::Default for frac_t {
    fn default() -> Self {
        frac_t {
        denom: std::os::raw::c_int::default(),
        num: std::os::raw::c_int::default()
        }
    }
}

pub type frac = * mut crate::frac_t;
#[no_mangle]
pub static mut ctx: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut msg: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
#[no_mangle]
pub static mut digits: [expr_t; 4] = [expr_t {
    op: 0,
    val: 0,
    used: 0,
    left: 0 as *const expr_t as *mut expr_t,
    right: 0 as *const expr_t as *mut expr_t,
}; 4];
#[no_mangle]
pub unsafe extern "C" fn gen_digits() {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        digits[i as usize].val = 1 as std::os::raw::c_int + rand() % 9 as std::os::raw::c_int;
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut str: [std::os::raw::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut pos: std::os::raw::c_int = 0;
#[no_mangle]
pub static mut pool: [expr_t; 8] = [expr_t {
    op: 0,
    val: 0,
    used: 0,
    left: 0 as *const expr_t as *mut expr_t,
    right: 0 as *const expr_t as *mut expr_t,
}; 8];
#[no_mangle]
pub static mut pool_ptr: std::os::raw::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn reset() {
    let mut i: std::os::raw::c_int = 0;
    msg = 0 as *const std::os::raw::c_char;
    pos = 0 as std::os::raw::c_int;
    pool_ptr = pos;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        pool[i as usize].op = OP_NONE as std::os::raw::c_int;
        pool[i as usize].right = 0 as expr;
        pool[i as usize].left = pool[i as usize].right;
        i += 1;
        i;
    }
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        digits[i as usize].used = 0 as std::os::raw::c_int;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bail(mut s: *const std::os::raw::c_char) {
    msg = s;
    longjmp(ctx.as_mut_ptr(), 1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn new_expr() -> expr {
    if pool_ptr < 8 as std::os::raw::c_int {
        let fresh0 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        return pool.as_mut_ptr().offset(fresh0 as isize);
    }
    return 0 as expr;
}
#[no_mangle]
pub unsafe extern "C" fn next_tok() -> std::os::raw::c_int {
    while *(*__ctype_b_loc()).offset(str[pos as usize] as std::os::raw::c_int as isize)
        as std::os::raw::c_int & _ISspace as std::os::raw::c_int as std::os::raw::c_ushort as std::os::raw::c_int != 0
    {
        pos += 1;
        pos;
    }
    return str[pos as usize] as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn take() -> std::os::raw::c_int {
    if str[pos as usize] as std::os::raw::c_int != '\0' as i32 {
        pos += 1;
        return pos;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_expr() -> expr {
    let mut c: std::os::raw::c_int = 0;
    let mut l: expr = 0 as *mut expr_t;
    let mut r: expr = 0 as *mut expr_t;
    let mut ret: expr = 0 as *mut expr_t;
    ret = get_term();
    if ret.is_null() {
        bail(b"Expected term\0" as *const u8 as *const std::os::raw::c_char);
    }
    loop {
        c = next_tok();
        if !(c == '+' as i32 || c == '-' as i32) {
            break;
        }
        if take() == 0 {
            bail(b"Unexpected end of input\0" as *const u8 as *const std::os::raw::c_char);
        }
        r = get_term();
        if r.is_null() {
            bail(b"Expected term\0" as *const u8 as *const std::os::raw::c_char);
        }
        l = ret;
        ret = new_expr();
        (*ret)
            .op = if c == '+' as i32 {
            OP_ADD as std::os::raw::c_int
        } else {
            OP_SUB as std::os::raw::c_int
        };
        (*ret).left = l;
        (*ret).right = r;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_term() -> expr {
    let mut c: std::os::raw::c_int = 0;
    let mut l: expr = 0 as *mut expr_t;
    let mut r: expr = 0 as *mut expr_t;
    let mut ret: expr = 0 as *mut expr_t;
    ret = get_fact();
    loop {
        c = next_tok();
        if !(c == '*' as i32 || c == '/' as i32) {
            break;
        }
        if take() == 0 {
            bail(b"Unexpected end of input\0" as *const u8 as *const std::os::raw::c_char);
        }
        r = get_fact();
        l = ret;
        ret = new_expr();
        (*ret)
            .op = if c == '*' as i32 {
            OP_MUL as std::os::raw::c_int
        } else {
            OP_DIV as std::os::raw::c_int
        };
        (*ret).left = l;
        (*ret).right = r;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn get_digit() -> expr {
    let mut i: std::os::raw::c_int = 0;
    let mut c: std::os::raw::c_int = next_tok();
    let mut ret: expr = 0 as *mut expr_t;
    if c >= '0' as i32 && c <= '9' as i32 {
        take();
        ret = new_expr();
        (*ret).op = OP_NUM as std::os::raw::c_int;
        (*ret).val = c - '0' as i32;
        i = 0 as std::os::raw::c_int;
        while i < 4 as std::os::raw::c_int {
            if digits[i as usize].val == (*ret).val && digits[i as usize].used == 0 {
                digits[i as usize].used = 1 as std::os::raw::c_int;
                return ret;
            }
            i += 1;
            i;
        }
        bail(b"Invalid digit\0" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as expr;
}
#[no_mangle]
pub unsafe extern "C" fn get_fact() -> expr {
    let mut c: std::os::raw::c_int = 0;
    let mut l: expr = get_digit();
    if !l.is_null() {
        return l;
    }
    c = next_tok();
    if c == '(' as i32 {
        take();
        l = get_expr();
        if next_tok() != ')' as i32 {
            bail(b"Unbalanced parens\0" as *const u8 as *const std::os::raw::c_char);
        }
        take();
        return l;
    }
    return 0 as expr;
}
#[no_mangle]
pub unsafe extern "C" fn parse() -> expr {
    let mut i: std::os::raw::c_int = 0;
    let mut ret: expr = get_expr();
    if next_tok() != '\0' as i32 {
        bail(b"Trailing garbage\0" as *const u8 as *const std::os::raw::c_char);
    }
    i = 0 as std::os::raw::c_int;
    while i < 4 as std::os::raw::c_int {
        if digits[i as usize].used == 0 {
            bail(b"Not all digits are used\0" as *const u8 as *const std::os::raw::c_char);
        }
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn gcd(mut m: std::os::raw::c_int, mut n: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut t: std::os::raw::c_int = 0;
    while m != 0 {
        t = m;
        m = n % m;
        n = t;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn eval_tree<'a1>(mut e: * mut crate::expr_t, mut res: Option<&'a1 mut crate::frac_t>) {
    let mut l: frac_t = frac_t { denom: 0, num: 0 };
    let mut r: frac_t = frac_t { denom: 0, num: 0 };
    let mut t: std::os::raw::c_int = 0;
    if (*e).op == OP_NUM as std::os::raw::c_int {
        (*borrow_mut(&mut res).unwrap()).num = (*e).val;
        (*borrow_mut(&mut res).unwrap()).denom = 1 as std::os::raw::c_int;
        return;
    }
    eval_tree((*e).left, Some(&mut l));
    eval_tree((*e).right, Some(&mut r));
    match (*e).op {
        2 => {
            (*borrow_mut(&mut res).unwrap()).num = l.num * r.denom + l.denom * r.num;
            (*borrow_mut(&mut res).unwrap()).denom = l.denom * r.denom;
        }
        3 => {
            (*borrow_mut(&mut res).unwrap()).num = l.num * r.denom - l.denom * r.num;
            (*borrow_mut(&mut res).unwrap()).denom = l.denom * r.denom;
        }
        4 => {
            (*borrow_mut(&mut res).unwrap()).num = l.num * r.num;
            (*borrow_mut(&mut res).unwrap()).denom = l.denom * r.denom;
        }
        5 => {
            (*borrow_mut(&mut res).unwrap()).num = l.num * r.denom;
            (*borrow_mut(&mut res).unwrap()).denom = l.denom * r.num;
        }
        _ => {}
    }
    t = gcd((*borrow_mut(&mut res).unwrap()).denom, (*borrow_mut(&mut res).unwrap()).num);
    if t != 0 {
        (*borrow_mut(&mut res).unwrap()).denom /= t;
        (*borrow_mut(&mut res).unwrap()).num /= t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_input() {
    let mut i: std::os::raw::c_int = 0;
    loop {
        reset();
        printf(b"\nAvailable digits are:\0" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < 4 as std::os::raw::c_int {
            printf(b" %d\0" as *const u8 as *const std::os::raw::c_char, digits[i as usize].val);
            i += 1;
            i;
        }
        printf(
            b". Type an expression and I'll check it for you, or make new numbers.\nYour choice? [Expr/n/q] \0"
                as *const u8 as *const std::os::raw::c_char,
        );
        i = 0 as std::os::raw::c_int;
        while i < 64 as std::os::raw::c_int {
            str[i as usize] = '\n' as i32 as std::os::raw::c_char;
            i += 1;
            i;
        }
        fgets(str.as_mut_ptr(), 64 as std::os::raw::c_int, stdin);
        if *str.as_mut_ptr() as std::os::raw::c_int == '\0' as i32 {
            continue;
        }
        if str[(64 as std::os::raw::c_int - 1 as std::os::raw::c_int) as usize] as std::os::raw::c_int
            != '\n' as i32
        {
            bail(b"string too long\0" as *const u8 as *const std::os::raw::c_char);
        }
        i = 0 as std::os::raw::c_int;
        while i < 64 as std::os::raw::c_int {
            if str[i as usize] as std::os::raw::c_int == '\n' as i32 {
                str[i as usize] = '\0' as i32 as std::os::raw::c_char;
            }
            i += 1;
            i;
        }
        if str[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 'q' as i32 {
            printf(b"Bye\n\0" as *const u8 as *const std::os::raw::c_char);
            exit(0 as std::os::raw::c_int);
        }
        if !(str[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 'n' as i32) {
            break;
        }
        gen_digits();
    };
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut f: frac_t = frac_t { denom: 0, num: 0 };
    srand(time(0 as *mut time_t) as std::os::raw::c_uint);
    gen_digits();
    loop {
        get_input();
        _setjmp(ctx.as_mut_ptr());
        if !msg.is_null() {
            printf(
                b"%s at '%.*s'\n\0" as *const u8 as *const std::os::raw::c_char,
                msg,
                pos,
                str.as_mut_ptr(),
            );
        } else {
            eval_tree(parse(), Some(&mut f));
            if f.denom == 0 as std::os::raw::c_int {
                bail(b"Divide by zero\0" as *const u8 as *const std::os::raw::c_char);
            }
            if f.denom == 1 as std::os::raw::c_int && f.num == 24 as std::os::raw::c_int {
                printf(
                    b"You got 24.  Very good.\n\0" as *const u8 as *const std::os::raw::c_char,
                );
            } else {
                if f.denom == 1 as std::os::raw::c_int {
                    printf(
                        b"Eval to: %d, \0" as *const u8 as *const std::os::raw::c_char,
                        f.num,
                    );
                } else {
                    printf(
                        b"Eval to: %d/%d, \0" as *const u8 as *const std::os::raw::c_char,
                        f.num,
                        f.denom,
                    );
                }
                printf(b"no good.  Try again.\n\0" as *const u8 as *const std::os::raw::c_char);
            }
        }
    };
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
