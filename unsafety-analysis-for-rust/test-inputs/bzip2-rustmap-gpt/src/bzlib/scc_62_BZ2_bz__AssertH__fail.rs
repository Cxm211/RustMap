use crate::bzlib::scc_42_root_57_BZ2_bzlibVersion::*;

// BZ2_bz__AssertH__fail ( int errcode )

// BZ2_bz__AssertH__fail ( int errcode )
use std::fs::{File, remove_file};
use std::panic::{catch_unwind, AssertUnwindSafe};


use std::io::{self, Write, Read, Cursor, stderr};

use std::process::Command;
pub fn bz2_bz_assert_h_fail(errcode: i32) {
    writeln!(
        stderr(),
        "\n\nbzip2/libbzip2: internal error number {}.\
        \nThis is a bug in bzip2/libbzip2, {}.\
        \nPlease report it to: bzip2-devel@sourceware.org. ... \
        \nThanks.\n\n",
        errcode,
        bz2_bzlib_version()
    )
    .unwrap();

    if errcode == 1007 {
        writeln!(
            stderr(),
            "\n*** A special note about internal error number 1007 ***\
            \nExperience suggests that a common cause of i.e. 1007...\
            \n\n"
        )
        .unwrap();
    }

    panic!("Exit with status 3");
}

use std::fs;
use std::path::Path;

#[test]
fn test_bz2_bz_assert_h_fail() {
    // 1. 创建临时的二进制项目
    Command::new("cargo")
        .arg("new")
        .arg("temp_binary")
        .output()
        .expect("Failed to create temp_binary");

    // 将逻辑写入临时的main.rs文件中
    let temp_main_content = r#"
        fn main() {
            // println!("Hello, temp_binary!");
        }
    "#;

    fs::write("temp_binary/src/main.rs", temp_main_content).expect("Unable to write to temp_binary's main.rs");

    // 2. 构建这个二进制项目
    let build_output = Command::new("cargo")
        .current_dir("temp_binary")
        .arg("build")
        .output()
        .expect("Failed to build temp_binary");

    if !build_output.status.success() {
        panic!("Failed to build temp_binary: {}", String::from_utf8_lossy(&build_output.stderr));
    }

    // 获取当前的工作目录
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let binary_path = current_dir.join("temp_binary/target/debug/temp_binary");

    // 3. 使用Command调用这个二进制文件
    if Path::new(&binary_path).exists() {
        let output = Command::new(binary_path)
            .arg("1007") // 传递参数到你的函数
            .output()
            .expect("Failed to execute temp_binary");

        let captured_stdout = String::from_utf8(output.stdout).expect("Buffer contains invalid UTF-8");
        assert_eq!(captured_stdout, "Hello, temp_binary!\n");
    } else {
        panic!("Binary not found at expected path");
    }

    // 4. 删除这个临时项目
    fs::remove_dir_all("temp_binary").expect("Failed to remove temp_binary");
}
