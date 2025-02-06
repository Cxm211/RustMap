use std::process::Command;
pub fn execute_command (command: &str) {
    // println!("Executing command: {}", command); // 打印出将要执行的命令

    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status();

    match status {
        Ok(status) if status.success() =>  println!("Command executed successfully"),
        _ => eprintln!("Command failed to execute"),
    }
}

pub fn string_to_u8_array_2000(s: String) -> [u8; 2000] {
    let mut array = [0; 2000];
    let bytes = s.into_bytes();
    if bytes.len() > 2000 {
        panic!("String 长度超过 2000 字节");
    }
    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte;
    }
    array
}

pub fn string_to_u8_array_1034(s: String) -> [u8; 1034] {
    let mut array = [0; 1034];
    let bytes = s.into_bytes();
    if bytes.len() > 1034 {
        panic!("String 长度超过 1034 字节");
    }
    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte;
    }
    array
}
pub fn u8_array_to_string(array: &[u8]) -> String {
    let end = array.iter().position(|&x| x == 0).unwrap_or(array.len());
    String::from_utf8(array[..end].to_vec())
        .expect("数组中的数据不是有效的 UTF-8 字符串")
}

