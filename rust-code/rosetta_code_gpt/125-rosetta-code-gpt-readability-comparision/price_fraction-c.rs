fn main() {
    {
        let mut n: u64 = 1;
        for _ in 0..30 {
            println!("{}", n.count_ones());
            n *= 3;
        }
        println!();
    }
    
    let mut od = [0; 30];
    let mut ne = 0;
    let mut no = 0;
    println!("evil  : ");
    for n in 0.. {
        if n.count_ones() % 2 == 0 {
            if ne < 30 {
                print!("{} ", n);
                ne += 1;
            }
        } else {
            if no < 30 {
                od[no] = n;
                no += 1;
            }
        }
        if ne + no >= 60 {
            break;
        }
    }
    println!();
    println!("odious: ");
    for &num in od.iter() {
        print!("{} ", num);
    }
    println!();
}