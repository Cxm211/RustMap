use crate::bzip2::scc_79_root_46_setExit::*;
use crate::global_vars::bzip2_c5::{exitValue};

pub fn copy_file_name(to: &mut String, from: &str) {
    const MAX_LEN: usize = 1034 - 10; // 明确注释数字类型

    if from.len() > MAX_LEN {
        eprintln!(
            "bzip2: file name\n`{}'\n is suspiciously (more than {} chars) long.\n Try using a reasonable file name instead.  Sorry! :-)",
            from, MAX_LEN
        );
        unsafe {
            set_exit(1);
            std::process::exit(exitValue);
        }
    }

    to.clear();
    to.push_str(&from[..from.len().min(MAX_LEN)]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_copy_file_name_valid() {
        let mut dest = String::new();
        let source = "test.txt";
        copy_file_name(&mut dest, source);
        assert_eq!(dest, source);
    }

    #[test]
    fn test_copy_file_name_too_long() {
        let result = panic::catch_unwind(|| {
            let mut dest = String::new();
            let source = "a".repeat(1024);
            copy_file_name(&mut dest, &source);
        });
        assert!(result.is_err());
    }
}
