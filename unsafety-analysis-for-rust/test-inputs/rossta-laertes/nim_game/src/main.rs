#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
// //use ::c2rust_out::*;
extern "C" {
    fn scanf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(b"Nim Game\n\n\0" as *const u8 as *const std::os::raw::c_char);
    let mut Tokens: std::os::raw::c_int = 12 as std::os::raw::c_int;
    while Tokens > 0 as std::os::raw::c_int {
        printf(
            b"How many tokens would you like to take?: \0" as *const u8
                as *const std::os::raw::c_char,
        );
        let mut uin: std::os::raw::c_int = 0;
        scanf(b"%i\0" as *const u8 as *const std::os::raw::c_char, &mut uin as *mut std::os::raw::c_int);
        let mut nextTokens: std::os::raw::c_int = playerTurn(Tokens, uin);
        if nextTokens == Tokens {
            continue;
        }
        Tokens = nextTokens;
        Tokens = computerTurn(Tokens);
    }
    printf(b"Computer wins.\0" as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn playerTurn(
    mut numTokens: std::os::raw::c_int,
    mut take: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if take < 1 as std::os::raw::c_int || take > 3 as std::os::raw::c_int {
        printf(
            b"\nTake must be between 1 and 3.\n\n\0" as *const u8 as *const std::os::raw::c_char,
        );
        return numTokens;
    }
    let mut remainingTokens: std::os::raw::c_int = numTokens - take;
    printf(b"\nPlayer takes %i tokens.\n\0" as *const u8 as *const std::os::raw::c_char, take);
    printf(
        b"%i tokens remaining.\n\n\0" as *const u8 as *const std::os::raw::c_char,
        remainingTokens,
    );
    return remainingTokens;
}
#[no_mangle]
pub unsafe extern "C" fn computerTurn(mut numTokens: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut take: std::os::raw::c_int = numTokens % 4 as std::os::raw::c_int;
    let mut remainingTokens: std::os::raw::c_int = numTokens - take;
    printf(b"Computer takes %u tokens.\n\0" as *const u8 as *const std::os::raw::c_char, take);
    printf(
        b"%i tokens remaining.\n\n\0" as *const u8 as *const std::os::raw::c_char,
        remainingTokens,
    );
    return remainingTokens;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
