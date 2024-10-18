fn main() {
    let mut n: u64 = 1;
    
    for _ in 0..30 {
        print!("{} ", n.count_ones());
        
        n *= 3;
    }
    println!();
    
    let mut od: Vec<i32> = Vec::new();
    
    let mut ne = 0;
    let mut no = 0;
    
    print!("evil  : ");
    
    let mut n = 0;
    while ne + no < 60 {
        if n.count_ones() % 2 == 0 {
            if ne < 30 {
                print!("{} ", n);
                ne += 1;
            }
        } else { // 否则，这个数量是奇数（"odious"）
            if no < 30 {
                od.push(n);
                no += 1;
            }
        }
        
        n += 1;
    }
    
    println!();
    print!("odious: ");
    
    for number in od.iter() {
        print!("{} ", number);
    }
    
    println!();
}