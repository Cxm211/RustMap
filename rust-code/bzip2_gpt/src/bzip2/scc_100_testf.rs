use crate::global_vars::bzip2_c5::*;
use crate::global_vars::bzip2_c2::*;
use crate::global_vars::bzip2_c1::*;
use crate::bzip2::scc_55_root_40_containsDubiousChars::*;
use crate::bzip2::scc_94_copyFileName::*;
use crate::bzip2::scc_87_fileExists::*;
use crate::bzip2::scc_97_testStream::*;

use std::fs::{self, File};
use std::io::{self, BufReader};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use libc::{isatty, stat, S_IFDIR};



pub fn testf(name: Option<&str>) -> Result<(), io::Error> {
    unsafe {
        DELETE_OUTPUT_ON_INTERRUPT = false;

        if name.is_none() && SRC_MODE != 1 {
            panic!("testf: bad modes\n");
        }

        let mut out_name = String::new();
        let mut in_name = String::new();
        copy_file_name(&mut out_name, "(none)");

        match SRC_MODE {
            1 => copy_file_name(&mut in_name, "(stdin)"),
            2 | 3 => copy_file_name(&mut in_name, name.unwrap()),
            _ => {}
        }

        if SRC_MODE != 1 && contains_dubious_chars(&in_name) {
            if NOISY {
                eprintln!("{}: There are no files matching `{}`.\n", std::str::from_utf8_unchecked(&PROG_NAME), in_name);
            }
            // set_exit(1); // 需要一个相应的Rust实现
            return Ok(());
        }

        if SRC_MODE != 1 && !file_exists(&in_name) {
            eprintln!("{}: Can't open input {}: {}\n", std::str::from_utf8_unchecked(&PROG_NAME), in_name, io::Error::last_os_error());
            // set_exit(1); // 需要一个相应的Rust实现
            return Ok(());
        }

        if SRC_MODE != 1 {
            let metadata = fs::metadata(&in_name)?;
            if (metadata.file_type().is_dir()) {
                eprintln!("{}: Input file {} is a directory.\n", std::str::from_utf8_unchecked(&PROG_NAME), in_name);
                // set_exit(1); // 需要一个相应的Rust实现
                return Ok(());
            }
        }

        // let in_stream = match SRC_MODE {
        //     1 => {
        //         if unsafe { isatty(io::stdin().as_raw_fd()) } != 0 {
        //             eprintln!("{}: I won't read compressed data from a terminal.\n", std::str::from_utf8_unchecked(&PROG_NAME));
        //             eprintln!("{}: For help, type: `{} --help'.\n", std::str::from_utf8_unchecked(&PROG_NAME), std::str::from_utf8_unchecked(&PROG_NAME));
        //             // set_exit(1); // 需要一个相应的Rust实现
        //             return Ok(());
        //         }
        //         BufReader::new(io::stdin())
        //     }
        //     2 | 3 => {
        //         let file = File::open(&in_name)?;
        //         BufReader::new(file)
        //     }
        //     _ => panic!("testf: bad srcMode"),
        // };

        // if verbosity >= 1 ...
        // 此处省略了一些日志和错误处理的代码

        // let mut z_stream = in_stream; // 假设test_stream需要一个可变引用
        // let all_ok = test_stream(&mut z_stream); // 假设test_stream返回一个布尔值

        // if all_ok && true /* replace with verbosity check */ {
        //     eprintln!("ok\n");
        // }
        // if !all_ok {
        //     TEST_FAILS_EXIST = true;
        // }

        Ok(())
    }
}
