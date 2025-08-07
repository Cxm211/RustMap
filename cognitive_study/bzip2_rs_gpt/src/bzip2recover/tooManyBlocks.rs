use crate::global_vars::bzip2_c2::{IN_FILE_NAME, PROG_NAME};

fn too_many_blocks(max_handled_blocks: i32) {
    eprintln!("{}: `{}' appears to contain more than {} blocks",
              unsafe { std::str::from_utf8(&PROG_NAME).unwrap_or_default() },
              unsafe { std::str::from_utf8(&IN_FILE_NAME).unwrap_or_default() },
              max_handled_blocks);

    eprintln!("{}: and cannot be handled.  To fix, increase", unsafe { std::str::from_utf8(&PROG_NAME).unwrap_or_default() });
    eprintln!("{}: BZ_MAX_HANDLED_BLOCKS in bzip2recover.c, and recompile.", unsafe { std::str::from_utf8(&PROG_NAME).unwrap_or_default() });

    // 如果不是在测试环境，退出程序
    #[cfg(not(test))]
    std::process::exit(1);
}


fn mock_exit(_status: i32) {
    // 在这里可以检查退出状态等...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_many_blocks() {
        let test_prog: &str = "testProg";
        let test_file: &str = "testFile.bz2";
    
        unsafe {
            PROG_NAME[..test_prog.len()].copy_from_slice(test_prog.as_bytes());
            IN_FILE_NAME[..test_file.len()].copy_from_slice(test_file.as_bytes());
        }

        too_many_blocks(1000);  // 调用函数
    }
}

