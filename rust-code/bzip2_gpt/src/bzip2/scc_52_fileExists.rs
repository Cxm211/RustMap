use std::fs::File;

pub fn file_exists(name: &str) -> bool {
    match File::open(name) {
        Ok(_) => true,
        Err(_) => false,
    }
}
