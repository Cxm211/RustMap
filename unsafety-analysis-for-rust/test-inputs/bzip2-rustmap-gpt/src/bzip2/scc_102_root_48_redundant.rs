use crate::global_vars::bzip2_c2::{PROG_NAME, Char};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MOCK_FPRINTF_CALLED: Mutex<bool> = Mutex::new(false);
    static ref CAPTURED_MSG: Mutex<String> = Mutex::new(String::new());
}

fn mock_fprintf(format: &str, args: std::fmt::Arguments) {
    *MOCK_FPRINTF_CALLED.lock().unwrap() = true;
    *CAPTURED_MSG.lock().unwrap() = format!("{}\n", format!("{}", args));  // 添加了换行符
}

pub fn redundant(flag: &[Char]) {
    let prog_name = unsafe {
        let pos = PROG_NAME.iter().position(|&x| x == 0).unwrap_or(PROG_NAME.len());
        std::str::from_utf8(&PROG_NAME[..pos]).unwrap()
    };
    
    mock_fprintf(
        "%s: %s is redundant in versions 0.9.5 and above\n",
        format_args!("{}: {} is redundant in versions 0.9.5 and above", prog_name, std::str::from_utf8(flag).unwrap())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redundant() {
        unsafe {
            PROG_NAME[..8].clone_from_slice(b"testProg");
        }
    
        redundant(b"testFlag");
    
        assert_eq!(*super::MOCK_FPRINTF_CALLED.lock().unwrap(), true);
        let expected_msg = "testProg: testFlag is redundant in versions 0.9.5 and above\n";
        assert_eq!(super::CAPTURED_MSG.lock().unwrap().as_str(), expected_msg);
    }
}
