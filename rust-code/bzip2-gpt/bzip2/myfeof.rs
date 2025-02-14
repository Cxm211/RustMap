use std::ptr;

pub fn myfeof(f: *mut libc::FILE) -> Bool {
    unsafe {
        let c = libc::fgetc(f);
        if c == -1 {
            return True;
        }
        libc::ungetc(c, f);
        False
    }
}
