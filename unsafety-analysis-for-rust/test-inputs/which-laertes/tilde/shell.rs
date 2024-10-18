#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn getuid() -> __uid_t;
    fn getenv(__name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
}
pub use crate::bash::__uid_t;
pub use crate::bash::__gid_t;
// #[derive(Copy, Clone)]

pub use crate::bash::passwd;
#[no_mangle]
pub unsafe extern "C" fn get_env_value(
    mut varname: *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    return getenv(varname);
}
#[no_mangle]
pub unsafe extern "C" fn get_home_dir() -> *mut std::os::raw::c_char {
    let mut home_dir: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut entry: *mut passwd = 0 as *mut passwd;
    home_dir = 0 as *mut std::os::raw::c_void as *mut std::os::raw::c_char;
    entry = getpwuid(getuid());
    if !entry.is_null() {
        home_dir = (*entry).pw_dir;
    }
    return home_dir;
}
