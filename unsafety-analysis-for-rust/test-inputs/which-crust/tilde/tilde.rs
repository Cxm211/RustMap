#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strncmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strncpy(
        _: *mut std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> *mut std::os::raw::c_char;
    fn free(_: *mut std::os::raw::c_void);
    fn getpwnam(_: *const std::os::raw::c_char) -> *mut passwd;
    fn endpwent();
    fn xmalloc(_: size_t) -> *mut std::os::raw::c_void;
    fn xrealloc(_: *mut std::os::raw::c_void, _: size_t) -> *mut std::os::raw::c_void;
    fn sh_get_home_dir() -> *mut std::os::raw::c_char;
    fn sh_get_env_value(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
}
pub type __uint32_t = std::os::raw::c_uint;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_time_t = std::os::raw::c_long;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_uid_t = __uint32_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
pub type gid_t = __darwin_gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut std::os::raw::c_char,
    pub pw_passwd: *mut std::os::raw::c_char,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_change: __darwin_time_t,
    pub pw_class: *mut std::os::raw::c_char,
    pub pw_gecos: *mut std::os::raw::c_char,
    pub pw_dir: *mut std::os::raw::c_char,
    pub pw_shell: *mut std::os::raw::c_char,
    pub pw_expire: __darwin_time_t,
}
pub type tilde_hook_func_t = unsafe extern "C" fn(
    *mut std::os::raw::c_char,
) -> *mut std::os::raw::c_char;
static mut default_prefixes: [*const std::os::raw::c_char; 3] = [
    b" ~\0" as *const u8 as *const std::os::raw::c_char,
    b"\t~\0" as *const u8 as *const std::os::raw::c_char,
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void as *const std::os::raw::c_char,
];
static mut default_suffixes: [*const std::os::raw::c_char; 3] = [
    b" \0" as *const u8 as *const std::os::raw::c_char,
    b"\n\0" as *const u8 as *const std::os::raw::c_char,
    0 as *const std::os::raw::c_void as *mut std::os::raw::c_void as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut tilde_expansion_preexpansion_hook: Option::<tilde_hook_func_t> = unsafe {
    ::std::mem::transmute::<
        *mut std::os::raw::c_void,
        Option::<tilde_hook_func_t>,
    >(0 as *const std::os::raw::c_void as *mut std::os::raw::c_void)
};
#[no_mangle]
pub static mut tilde_expansion_failure_hook: Option::<tilde_hook_func_t> = unsafe {
    ::std::mem::transmute::<
        *mut std::os::raw::c_void,
        Option::<tilde_hook_func_t>,
    >(0 as *const std::os::raw::c_void as *mut std::os::raw::c_void)
};
#[no_mangle]
pub static mut tilde_additional_prefixes: *mut *mut std::os::raw::c_char = unsafe {
    default_prefixes.as_ptr() as *mut *mut std::os::raw::c_char
};
#[no_mangle]
pub static mut tilde_additional_suffixes: *mut *mut std::os::raw::c_char = unsafe {
    default_suffixes.as_ptr() as *mut *mut std::os::raw::c_char
};

unsafe extern "C" fn tilde_find_prefix(
    mut string: *const std::os::raw::c_char,
    mut len: *mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut string_len: std::os::raw::c_int = 0;
    let mut prefixes: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    prefixes = tilde_additional_prefixes;
    string_len = strlen(string) as std::os::raw::c_int;
    *len = 0 as std::os::raw::c_int;
    if *string as std::os::raw::c_int == '\0' as i32 || *string as std::os::raw::c_int == '~' as i32 {
        return 0 as std::os::raw::c_int;
    }
    if !prefixes.is_null() {
        i = 0 as std::os::raw::c_int;
        while i < string_len {
            j = 0 as std::os::raw::c_int;
            while !(*prefixes.offset(j as isize)).is_null() {
                if strncmp(
                    string.offset(i as isize),
                    *prefixes.offset(j as isize),
                    strlen(*prefixes.offset(j as isize)),
                ) == 0 as std::os::raw::c_int
                {
                    *len = (strlen(*prefixes.offset(j as isize)))
                        .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
                    return i + *len;
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    return string_len;
}
unsafe extern "C" fn tilde_find_suffix(mut string: *const std::os::raw::c_char) -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut string_len: std::os::raw::c_int = 0;
    let mut suffixes: *mut *mut std::os::raw::c_char = 0 as *mut *mut std::os::raw::c_char;
    suffixes = tilde_additional_suffixes;
    string_len = strlen(string) as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < string_len {
        if *string.offset(i as isize) as std::os::raw::c_int == '/' as i32 {
            break;
        }
        j = 0 as std::os::raw::c_int;
        while !suffixes.is_null() && !(*suffixes.offset(j as isize)).is_null() {
            if strncmp(
                string.offset(i as isize),
                *suffixes.offset(j as isize),
                strlen(*suffixes.offset(j as isize)),
            ) == 0 as std::os::raw::c_int
            {
                return i;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn tilde_expand(
    mut string: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut result: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut result_size: std::os::raw::c_int = 0;
    let mut result_index: std::os::raw::c_int = 0;
    result_size = 0 as std::os::raw::c_int;
    result_index = result_size;
    result = strchr(string, '~' as i32);
    if !result.is_null() {
        result_size = (strlen(string)).wrapping_add(16 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        result = xmalloc(result_size as size_t) as *mut std::os::raw::c_char;
    } else {
        result_size = (strlen(string)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            as std::os::raw::c_int;
        result = xmalloc(result_size as size_t) as *mut std::os::raw::c_char;
    }
    loop {
        let mut start: std::os::raw::c_int = 0;
        let mut end: std::os::raw::c_int = 0;
        let mut tilde_word: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut expansion: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
        let mut len: std::os::raw::c_int = 0;
        start = tilde_find_prefix(string, &mut len);
        if result_index + start + 1 as std::os::raw::c_int > result_size {
            result_size += start + 20 as std::os::raw::c_int;
            result = xrealloc(
                result as *mut std::os::raw::c_void,
                (1 as std::os::raw::c_int + result_size) as size_t,
            ) as *mut std::os::raw::c_char;
        }
        strncpy(result.offset(result_index as isize), string, start as std::os::raw::c_ulong);
        result_index += start;
        string = string.offset(start as isize);
        end = tilde_find_suffix(string);
        if start == 0 && end == 0 {
            break;
        }
        tilde_word = xmalloc((1 as std::os::raw::c_int + end) as size_t) as *mut std::os::raw::c_char;
        strncpy(tilde_word, string, end as std::os::raw::c_ulong);
        *tilde_word.offset(end as isize) = '\0' as i32 as std::os::raw::c_char;
        string = string.offset(end as isize);
        expansion = tilde_expand_word(tilde_word);
        free(tilde_word as *mut std::os::raw::c_void);
        len = strlen(expansion) as std::os::raw::c_int;
        if result_index + len + 1 as std::os::raw::c_int > result_size {
            result_size += len + 20 as std::os::raw::c_int;
            result = xrealloc(
                result as *mut std::os::raw::c_void,
                (1 as std::os::raw::c_int + result_size) as size_t,
            ) as *mut std::os::raw::c_char;
        }
        strcpy(result.offset(result_index as isize), expansion);
        result_index += len;
        free(expansion as *mut std::os::raw::c_void);
    }
    *result.offset(result_index as isize) = '\0' as i32 as std::os::raw::c_char;
    return result;
}
unsafe extern "C" fn isolate_tilde_prefix(
    mut fname: *const std::os::raw::c_char,
    mut lenp: *mut std::os::raw::c_int,
) -> *mut std::os::raw::c_char {
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut i: std::os::raw::c_int = 0;
    ret = xmalloc(strlen(fname)) as *mut std::os::raw::c_char;
    i = 1 as std::os::raw::c_int;
    while *fname.offset(i as isize) as std::os::raw::c_int != 0
        && *fname.offset(i as isize) as std::os::raw::c_int != '/' as i32
    {
        *ret.offset((i - 1 as std::os::raw::c_int) as isize) = *fname.offset(i as isize);
        i += 1;
        i;
    }
    *ret.offset((i - 1 as std::os::raw::c_int) as isize) = '\0' as i32 as std::os::raw::c_char;
    if !lenp.is_null() {
        *lenp = i;
    }
    return ret;
}
unsafe extern "C" fn glue_prefix_and_suffix(
    mut prefix: *mut std::os::raw::c_char,
    mut suffix: *const std::os::raw::c_char,
    mut suffind: std::os::raw::c_int,
) -> *mut std::os::raw::c_char {
    let mut ret: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut plen: std::os::raw::c_int = 0;
    let mut slen: std::os::raw::c_int = 0;
    plen = (if !prefix.is_null() && *prefix as std::os::raw::c_int != 0 {
        strlen(prefix)
    } else {
        0 as std::os::raw::c_int as std::os::raw::c_ulong
    }) as std::os::raw::c_int;
    slen = strlen(suffix.offset(suffind as isize)) as std::os::raw::c_int;
    ret = xmalloc((plen + slen + 1 as std::os::raw::c_int) as size_t) as *mut std::os::raw::c_char;
    if plen != 0 {
        strcpy(ret, prefix);
    }
    strcpy(ret.offset(plen as isize), suffix.offset(suffind as isize));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tilde_expand_word(
    mut filename: *const std::os::raw::c_char,
) -> *mut std::os::raw::c_char {
    let mut dirname: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut expansion: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut username: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut user_len: std::os::raw::c_int = 0;
    let mut user_entry: *mut passwd = 0 as *mut passwd;
    if filename.is_null() {
        return 0 as *mut std::os::raw::c_void as *mut std::os::raw::c_char;
    }
    if *filename as std::os::raw::c_int != '~' as i32 {
        return strcpy(
            xmalloc((1 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(strlen(filename)))
                as *mut std::os::raw::c_char,
            filename,
        );
    }
    if *filename.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '\0' as i32
        || *filename.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int == '/' as i32
    {
        expansion = sh_get_env_value(b"HOME\0" as *const u8 as *const std::os::raw::c_char);
        if expansion.is_null() {
            expansion = sh_get_home_dir();
        }
        return glue_prefix_and_suffix(expansion, filename, 1 as std::os::raw::c_int);
    }
    username = isolate_tilde_prefix(filename, &mut user_len);
    if tilde_expansion_preexpansion_hook.is_some() {
        expansion = (Some(
            tilde_expansion_preexpansion_hook.expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(username);
        if !expansion.is_null() {
            dirname = glue_prefix_and_suffix(expansion, filename, user_len);
            free(username as *mut std::os::raw::c_void);
            free(expansion as *mut std::os::raw::c_void);
            return dirname;
        }
    }
    dirname = 0 as *mut std::os::raw::c_void as *mut std::os::raw::c_char;
    user_entry = getpwnam(username);
    if user_entry.is_null() {
        if tilde_expansion_failure_hook.is_some() {
            expansion = (Some(
                tilde_expansion_failure_hook.expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(username);
            if !expansion.is_null() {
                dirname = glue_prefix_and_suffix(expansion, filename, user_len);
                free(expansion as *mut std::os::raw::c_void);
            }
        }
        if dirname.is_null() {
            dirname = strcpy(
                xmalloc(
                    (1 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_add(strlen(filename)),
                ) as *mut std::os::raw::c_char,
                filename,
            );
        }
    } else {
        dirname = glue_prefix_and_suffix((*user_entry).pw_dir, filename, user_len);
    }
    free(username as *mut std::os::raw::c_void);
    endpwent();
    return dirname;
}
