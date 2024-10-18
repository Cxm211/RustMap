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
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.offset(str_size as isize) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

//use c2rust_out::*;
extern "C" {
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ref {
    pub index: u16,
    pub last_char: i8,
    pub first_char: i8,
}
#[no_mangle]
pub static mut longest_path_refs: *mut Ref = 0 as *const Ref as *mut Ref;
#[no_mangle]
pub static mut longest_path_refs_len: u64 = 0;
#[no_mangle]
pub static mut refs: *mut Ref = 0 as *const Ref as *mut Ref;
#[no_mangle]
pub static mut refs_len: u64 = 0;
#[no_mangle]
pub static mut n_solutions: u64 = 0;
#[no_mangle]
pub static mut longest_path: *mut *const i8 = 0 as *const *const i8 as *mut *const i8;
#[no_mangle]
pub static mut longest_path_len: u64 = 0;
#[no_mangle]
pub extern "C" fn search(mut curr_len: u64) {
    unsafe {
        if curr_len == longest_path_refs_len {
            n_solutions = n_solutions.wrapping_add(1);
            n_solutions;
        } else if curr_len > longest_path_refs_len {
            n_solutions = 1;
            longest_path_refs_len = curr_len;
            memcpy(
                longest_path_refs as *mut libc::c_void,
                refs as *const libc::c_void,
                curr_len.wrapping_mul(::core::mem::size_of::<Ref>() as u64),
            );
        }
        let mut last_char: i64 = (*refs.offset(curr_len.wrapping_sub(1) as isize)).last_char as i64;
        let mut i: u64 = curr_len;
        while i < refs_len {
            if (*refs.offset(i as isize)).first_char as i64 == last_char {
                let mut aux: Ref = *refs.offset(curr_len as isize);
                *refs.offset(curr_len as isize) = *refs.offset(i as isize);
                *refs.offset(i as isize) = aux;
                search(curr_len.wrapping_add(1));
                *refs.offset(i as isize) = *refs.offset(curr_len as isize);
                *refs.offset(curr_len as isize) = aux;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn find_longest_chain(mut items: *mut *const i8, mut items_len: u64) {
    unsafe {
        refs_len = items_len;
        refs = calloc(refs_len, ::core::mem::size_of::<Ref>() as u64) as *mut Ref;
        longest_path_refs_len = 0;
        longest_path_refs = calloc(refs_len, ::core::mem::size_of::<Ref>() as u64) as *mut Ref;
        let mut i: u64 = 0;
        while i < items_len {
            let mut itemsi_len: u64 = strlen(*items.offset(i as isize));
            if itemsi_len <= 1 {
                exit(1);
            };
            (*refs.offset(i as isize)).index = i as u16;
            (*refs.offset(i as isize)).last_char =
                *(*items.offset(i as isize)).offset(itemsi_len.wrapping_sub(1) as isize);
            (*refs.offset(i as isize)).first_char = *(*items.offset(i as isize)).offset(0 as isize);
            i = i.wrapping_add(1);
            i;
        }
        let mut i_0: u64 = 0;
        while i_0 < items_len {
            let mut aux: Ref = *refs.offset(0 as isize);
            *refs.offset(0 as isize) = *refs.offset(i_0 as isize);
            *refs.offset(i_0 as isize) = aux;
            search(1);
            *refs.offset(i_0 as isize) = *refs.offset(0 as isize);
            *refs.offset(0 as isize) = aux;
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        longest_path_len = longest_path_refs_len;
        longest_path =
            calloc(longest_path_len, ::core::mem::size_of::<*const i8>() as u64) as *mut *const i8;
        let mut i_1: u64 = 0;
        while i_1 < longest_path_len {
            let ref mut fresh0 = *longest_path.offset(i_1 as isize);
            *fresh0 = *items.offset((*longest_path_refs.offset(i_1 as isize)).index as isize);
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        free(longest_path_refs as *mut libc::c_void);
        free(refs as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    let mut pokemon: [*const i8; 70] = [
        b"audino\0" as *const u8 as *const i8,
        b"bagon\0" as *const u8 as *const i8,
        b"baltoy\0" as *const u8 as *const i8,
        b"banette\0" as *const u8 as *const i8,
        b"bidoof\0" as *const u8 as *const i8,
        b"braviary\0" as *const u8 as *const i8,
        b"bronzor\0" as *const u8 as *const i8,
        b"carracosta\0" as *const u8 as *const i8,
        b"charmeleon\0" as *const u8 as *const i8,
        b"cresselia\0" as *const u8 as *const i8,
        b"croagunk\0" as *const u8 as *const i8,
        b"darmanitan\0" as *const u8 as *const i8,
        b"deino\0" as *const u8 as *const i8,
        b"emboar\0" as *const u8 as *const i8,
        b"emolga\0" as *const u8 as *const i8,
        b"exeggcute\0" as *const u8 as *const i8,
        b"gabite\0" as *const u8 as *const i8,
        b"girafarig\0" as *const u8 as *const i8,
        b"gulpin\0" as *const u8 as *const i8,
        b"haxorus\0" as *const u8 as *const i8,
        b"heatmor\0" as *const u8 as *const i8,
        b"heatran\0" as *const u8 as *const i8,
        b"ivysaur\0" as *const u8 as *const i8,
        b"jellicent\0" as *const u8 as *const i8,
        b"jumpluff\0" as *const u8 as *const i8,
        b"kangaskhan\0" as *const u8 as *const i8,
        b"kricketune\0" as *const u8 as *const i8,
        b"landorus\0" as *const u8 as *const i8,
        b"ledyba\0" as *const u8 as *const i8,
        b"loudred\0" as *const u8 as *const i8,
        b"lumineon\0" as *const u8 as *const i8,
        b"lunatone\0" as *const u8 as *const i8,
        b"machamp\0" as *const u8 as *const i8,
        b"magnezone\0" as *const u8 as *const i8,
        b"mamoswine\0" as *const u8 as *const i8,
        b"nosepass\0" as *const u8 as *const i8,
        b"petilil\0" as *const u8 as *const i8,
        b"pidgeotto\0" as *const u8 as *const i8,
        b"pikachu\0" as *const u8 as *const i8,
        b"pinsir\0" as *const u8 as *const i8,
        b"poliwrath\0" as *const u8 as *const i8,
        b"poochyena\0" as *const u8 as *const i8,
        b"porygon2\0" as *const u8 as *const i8,
        b"porygonz\0" as *const u8 as *const i8,
        b"registeel\0" as *const u8 as *const i8,
        b"relicanth\0" as *const u8 as *const i8,
        b"remoraid\0" as *const u8 as *const i8,
        b"rufflet\0" as *const u8 as *const i8,
        b"sableye\0" as *const u8 as *const i8,
        b"scolipede\0" as *const u8 as *const i8,
        b"scrafty\0" as *const u8 as *const i8,
        b"seaking\0" as *const u8 as *const i8,
        b"sealeo\0" as *const u8 as *const i8,
        b"silcoon\0" as *const u8 as *const i8,
        b"simisear\0" as *const u8 as *const i8,
        b"snivy\0" as *const u8 as *const i8,
        b"snorlax\0" as *const u8 as *const i8,
        b"spoink\0" as *const u8 as *const i8,
        b"starly\0" as *const u8 as *const i8,
        b"tirtouga\0" as *const u8 as *const i8,
        b"trapinch\0" as *const u8 as *const i8,
        b"treecko\0" as *const u8 as *const i8,
        b"tyrogue\0" as *const u8 as *const i8,
        b"vigoroth\0" as *const u8 as *const i8,
        b"vulpix\0" as *const u8 as *const i8,
        b"wailord\0" as *const u8 as *const i8,
        b"wartortle\0" as *const u8 as *const i8,
        b"whismur\0" as *const u8 as *const i8,
        b"wingull\0" as *const u8 as *const i8,
        b"yamask\0" as *const u8 as *const i8,
    ];
    let mut pokemon_len: u64 = (::core::mem::size_of::<[*const i8; 70]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const i8>() as u64);
    find_longest_chain(pokemon.as_mut_ptr(), pokemon_len);
    unsafe {
        print!("Maximum path length: {}\n", longest_path_len);
        print!("Paths of that length: {}\n", n_solutions);
    }
    print!("Example path of that length:\n");
    let mut i: u64 = 0;
    unsafe {
        while i < longest_path_len {
            print!("  ");
            let mut j: u64 = i;
            while j < i.wrapping_add(7) && j < longest_path_len {
                print!(
                    "{} ",
                    build_str_from_raw_ptr(*longest_path.offset(j as isize) as *mut u8)
                );
                j = j.wrapping_add(1);
                j;
            }
            print!("\n");
            i = (i).wrapping_add(7) as u64;
        }
        free(longest_path as *mut libc::c_void);
    }
    return 0;
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
