fn main() {
    println!("word size = {} bits", 8 * std::mem::size_of::<usize>());
    
    let one: i32 = 1;
    let little_endian = unsafe { 
        *( &one as *const i32 as *const u8 ) == 1
    };
    
    if little_endian {
        println!("little endian");
    } else {
        println!("big endian");
    }
}