use std::fmt::Write;

fn main() {
    let mut k = String::new();
    for i in 0..16 {
        for j in (32 + i)..128 {
            if j % 16 != i {
                continue;
            }

            k.clear();
            match j {
                32 => write!(k, "Spc").unwrap(),
                127 => write!(k, "Del").unwrap(),
                _ => write!(k, "{}", j as u8 as char).unwrap(),
            }

            print!("{:3} : {:<3}   ", j, k);
        }
        println!();
    }
}
