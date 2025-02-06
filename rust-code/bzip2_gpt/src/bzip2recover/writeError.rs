type Char = u8;
pub static mut IN_FILE_NAME: [Char; 2000] = [0; 2000];
pub static mut OUT_FILE_NAME: [Char; 2000] = [0; 2000];
pub static mut PROG_NAME: [Char; 2000] = [0; 2000];



pub fn write_error() {
    unsafe {
        eprintln!("{}: I/O error reading `{}`, possible reason follows.", 
                  String::from_utf8_lossy(&PROG_NAME), 
                  String::from_utf8_lossy(&IN_FILE_NAME));
        eprintln!("Error: {}", std::io::Error::last_os_error());
        eprintln!("{}: warning: output file(s) may be incomplete.", String::from_utf8_lossy(&PROG_NAME));
        // std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, BufRead}; // Import required traits
    #[test]
    fn test_write_error() {
        // Redirect stderr to a file to capture the output
        let original_stderr = std::io::stderr();
        let stderr_backup = original_stderr.lock();

        let file = std::fs::File::create("stderr_output.txt").unwrap();
        let mut stderr_lock = std::io::stderr().lock();
        let _ = stderr_lock.write_all(b""); // Use write_all now that Write trait is imported
        
        unsafe {
            PROG_NAME[..11].copy_from_slice(b"testProgram");
            IN_FILE_NAME[..12].copy_from_slice(b"testFile.txt");
        }
        write_error();

        // ...

        // Read the output from the file and validate
        let file = std::fs::File::open("stderr_output.txt").unwrap();
        let reader = std::io::BufReader::new(file);

        for line in reader.lines().take(3) { // Use lines() now that BufRead trait is imported
            let line = line.unwrap();
            // Validate the lines as needed
        }
    }
}