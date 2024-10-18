#![no_std]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
//use ::c2rust_out::*;
extern "C" {
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn free(_: *mut std::os::raw::c_void);
    fn exit(_: std::os::raw::c_int) -> !;
    fn memcpy(
        _: *mut std::os::raw::c_void,
        _: *const std::os::raw::c_void,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_void;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub type size_t = std::os::raw::c_ulong;
pub type __uint16_t = std::os::raw::c_ushort;
pub type uint16_t = __uint16_t;
pub type intptr_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub index: uint16_t,
    pub last_char: std::os::raw::c_char,
    pub first_char: std::os::raw::c_char,
}
#[no_mangle]
pub static mut longest_path_refs: *mut Ref = 0 as *const Ref as *mut Ref;
#[no_mangle]
pub static mut longest_path_refs_len: size_t = 0;
#[no_mangle]
pub static mut refs: *mut Ref = 0 as *const Ref as *mut Ref;
#[no_mangle]
pub static mut refs_len: size_t = 0;
#[no_mangle]
pub static mut n_solutions: size_t = 0;
#[no_mangle]
pub static mut longest_path: *mut *const std::os::raw::c_char = 0 as *const *const std::os::raw::c_char
    as *mut *const std::os::raw::c_char;
#[no_mangle]
pub static mut longest_path_len: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn search(mut curr_len: size_t) {
    if curr_len == longest_path_refs_len {
        n_solutions = n_solutions.wrapping_add(1);
        n_solutions;
    } else if curr_len > longest_path_refs_len {
        n_solutions = 1 as std::os::raw::c_int as size_t;
        longest_path_refs_len = curr_len;
        memcpy(
            longest_path_refs as *mut std::os::raw::c_void,
            refs as *const std::os::raw::c_void,
            curr_len.wrapping_mul(::core::mem::size_of::<Ref>() as std::os::raw::c_ulong),
        );
    }
    let mut last_char: intptr_t = (*refs
        .offset(curr_len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize))
        .last_char as intptr_t;
    let mut i: size_t = curr_len;
    while i < refs_len {
        if (*refs.offset(i as isize)).first_char as std::os::raw::c_long == last_char {
            let mut aux: Ref = *refs.offset(curr_len as isize);
            *refs.offset(curr_len as isize) = *refs.offset(i as isize);
            *refs.offset(i as isize) = aux;
            search(curr_len.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
            *refs.offset(i as isize) = *refs.offset(curr_len as isize);
            *refs.offset(curr_len as isize) = aux;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_longest_chain(
    mut items: *mut *const std::os::raw::c_char,
    mut items_len: size_t,
) {
    refs_len = items_len;
    refs = calloc(refs_len, ::core::mem::size_of::<Ref>() as std::os::raw::c_ulong) as *mut Ref;
    longest_path_refs_len = 0 as std::os::raw::c_int as size_t;
    longest_path_refs = calloc(refs_len, ::core::mem::size_of::<Ref>() as std::os::raw::c_ulong)
        as *mut Ref;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < items_len {
        let mut itemsi_len: size_t = strlen(*items.offset(i as isize));
        if itemsi_len <= 1 as std::os::raw::c_int as std::os::raw::c_ulong {
            exit(1 as std::os::raw::c_int);
        }
        (*refs.offset(i as isize)).index = i as uint16_t;
        (*refs.offset(i as isize))
            .last_char = *(*items.offset(i as isize))
            .offset(itemsi_len.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize);
        (*refs.offset(i as isize))
            .first_char = *(*items.offset(i as isize)).offset(0 as std::os::raw::c_int as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut i_0: size_t = 0 as std::os::raw::c_int as size_t;
    while i_0 < items_len {
        let mut aux: Ref = *refs.offset(0 as std::os::raw::c_int as isize);
        *refs.offset(0 as std::os::raw::c_int as isize) = *refs.offset(i_0 as isize);
        *refs.offset(i_0 as isize) = aux;
        search(1 as std::os::raw::c_int as size_t);
        *refs.offset(i_0 as isize) = *refs.offset(0 as std::os::raw::c_int as isize);
        *refs.offset(0 as std::os::raw::c_int as isize) = aux;
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    longest_path_len = longest_path_refs_len;
    longest_path = calloc(
        longest_path_len,
        ::core::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong,
    ) as *mut *const std::os::raw::c_char;
    let mut i_1: size_t = 0 as std::os::raw::c_int as size_t;
    while i_1 < longest_path_len {
        let ref mut fresh0 = *longest_path.offset(i_1 as isize);
        *fresh0 = *items
            .offset((*longest_path_refs.offset(i_1 as isize)).index as isize);
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    free(longest_path_refs as *mut std::os::raw::c_void);
    free(refs as *mut std::os::raw::c_void);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let mut pokemon: [*const std::os::raw::c_char; 70] = [
        b"audino\0" as *const u8 as *const std::os::raw::c_char,
        b"bagon\0" as *const u8 as *const std::os::raw::c_char,
        b"baltoy\0" as *const u8 as *const std::os::raw::c_char,
        b"banette\0" as *const u8 as *const std::os::raw::c_char,
        b"bidoof\0" as *const u8 as *const std::os::raw::c_char,
        b"braviary\0" as *const u8 as *const std::os::raw::c_char,
        b"bronzor\0" as *const u8 as *const std::os::raw::c_char,
        b"carracosta\0" as *const u8 as *const std::os::raw::c_char,
        b"charmeleon\0" as *const u8 as *const std::os::raw::c_char,
        b"cresselia\0" as *const u8 as *const std::os::raw::c_char,
        b"croagunk\0" as *const u8 as *const std::os::raw::c_char,
        b"darmanitan\0" as *const u8 as *const std::os::raw::c_char,
        b"deino\0" as *const u8 as *const std::os::raw::c_char,
        b"emboar\0" as *const u8 as *const std::os::raw::c_char,
        b"emolga\0" as *const u8 as *const std::os::raw::c_char,
        b"exeggcute\0" as *const u8 as *const std::os::raw::c_char,
        b"gabite\0" as *const u8 as *const std::os::raw::c_char,
        b"girafarig\0" as *const u8 as *const std::os::raw::c_char,
        b"gulpin\0" as *const u8 as *const std::os::raw::c_char,
        b"haxorus\0" as *const u8 as *const std::os::raw::c_char,
        b"heatmor\0" as *const u8 as *const std::os::raw::c_char,
        b"heatran\0" as *const u8 as *const std::os::raw::c_char,
        b"ivysaur\0" as *const u8 as *const std::os::raw::c_char,
        b"jellicent\0" as *const u8 as *const std::os::raw::c_char,
        b"jumpluff\0" as *const u8 as *const std::os::raw::c_char,
        b"kangaskhan\0" as *const u8 as *const std::os::raw::c_char,
        b"kricketune\0" as *const u8 as *const std::os::raw::c_char,
        b"landorus\0" as *const u8 as *const std::os::raw::c_char,
        b"ledyba\0" as *const u8 as *const std::os::raw::c_char,
        b"loudred\0" as *const u8 as *const std::os::raw::c_char,
        b"lumineon\0" as *const u8 as *const std::os::raw::c_char,
        b"lunatone\0" as *const u8 as *const std::os::raw::c_char,
        b"machamp\0" as *const u8 as *const std::os::raw::c_char,
        b"magnezone\0" as *const u8 as *const std::os::raw::c_char,
        b"mamoswine\0" as *const u8 as *const std::os::raw::c_char,
        b"nosepass\0" as *const u8 as *const std::os::raw::c_char,
        b"petilil\0" as *const u8 as *const std::os::raw::c_char,
        b"pidgeotto\0" as *const u8 as *const std::os::raw::c_char,
        b"pikachu\0" as *const u8 as *const std::os::raw::c_char,
        b"pinsir\0" as *const u8 as *const std::os::raw::c_char,
        b"poliwrath\0" as *const u8 as *const std::os::raw::c_char,
        b"poochyena\0" as *const u8 as *const std::os::raw::c_char,
        b"porygon2\0" as *const u8 as *const std::os::raw::c_char,
        b"porygonz\0" as *const u8 as *const std::os::raw::c_char,
        b"registeel\0" as *const u8 as *const std::os::raw::c_char,
        b"relicanth\0" as *const u8 as *const std::os::raw::c_char,
        b"remoraid\0" as *const u8 as *const std::os::raw::c_char,
        b"rufflet\0" as *const u8 as *const std::os::raw::c_char,
        b"sableye\0" as *const u8 as *const std::os::raw::c_char,
        b"scolipede\0" as *const u8 as *const std::os::raw::c_char,
        b"scrafty\0" as *const u8 as *const std::os::raw::c_char,
        b"seaking\0" as *const u8 as *const std::os::raw::c_char,
        b"sealeo\0" as *const u8 as *const std::os::raw::c_char,
        b"silcoon\0" as *const u8 as *const std::os::raw::c_char,
        b"simisear\0" as *const u8 as *const std::os::raw::c_char,
        b"snivy\0" as *const u8 as *const std::os::raw::c_char,
        b"snorlax\0" as *const u8 as *const std::os::raw::c_char,
        b"spoink\0" as *const u8 as *const std::os::raw::c_char,
        b"starly\0" as *const u8 as *const std::os::raw::c_char,
        b"tirtouga\0" as *const u8 as *const std::os::raw::c_char,
        b"trapinch\0" as *const u8 as *const std::os::raw::c_char,
        b"treecko\0" as *const u8 as *const std::os::raw::c_char,
        b"tyrogue\0" as *const u8 as *const std::os::raw::c_char,
        b"vigoroth\0" as *const u8 as *const std::os::raw::c_char,
        b"vulpix\0" as *const u8 as *const std::os::raw::c_char,
        b"wailord\0" as *const u8 as *const std::os::raw::c_char,
        b"wartortle\0" as *const u8 as *const std::os::raw::c_char,
        b"whismur\0" as *const u8 as *const std::os::raw::c_char,
        b"wingull\0" as *const u8 as *const std::os::raw::c_char,
        b"yamask\0" as *const u8 as *const std::os::raw::c_char,
    ];
    let mut pokemon_len: size_t = (::core::mem::size_of::<[*const std::os::raw::c_char; 70]>()
        as std::os::raw::c_ulong)
        .wrapping_div(::core::mem::size_of::<*const std::os::raw::c_char>() as std::os::raw::c_ulong);
    find_longest_chain(pokemon.as_mut_ptr(), pokemon_len);
    printf(
        b"Maximum path length: %u\n\0" as *const u8 as *const std::os::raw::c_char,
        longest_path_len,
    );
    printf(
        b"Paths of that length: %u\n\0" as *const u8 as *const std::os::raw::c_char,
        n_solutions,
    );
    printf(b"Example path of that length:\n\0" as *const u8 as *const std::os::raw::c_char);
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < longest_path_len {
        printf(b"  \0" as *const u8 as *const std::os::raw::c_char);
        let mut j: size_t = i;
        while j < i.wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong)
            && j < longest_path_len
        {
            printf(
                b"%s \0" as *const u8 as *const std::os::raw::c_char,
                *longest_path.offset(j as isize),
            );
            j = j.wrapping_add(1);
            j;
        }
        printf(b"\n\0" as *const u8 as *const std::os::raw::c_char);
        i = (i as std::os::raw::c_ulong).wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    free(longest_path as *mut std::os::raw::c_void);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
