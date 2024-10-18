type Bool = bool;

fn sieve(c: &mut Vec<Bool>, limit: usize) {
    let mut p = 3;
    c[0] = true;
    c[1] = true;
    loop {
        let mut p2 = p * p;
        if p2 >= limit {
            break;
        }
        for i in (p2..limit).step_by(2 * p) {
            c[i] = true;
        }
        loop {
            p += 2;
            if !c[p] {
                break;
            }
        }
    }
}

fn print_helper(cat: &str, len: usize, lim: usize, n: usize) {
    let sp = if cat == "unsexy primes" { "" } else { "sexy prime " };
    let verb = if len == 1 { "is" } else { "are" };
    println!("Number of {}{} less than {:?} = {:?}", sp, cat, lim, len);
    println!("The last {} {}:", n, verb);
}

fn print_array(a: &[usize]) {
    print!("[");
    for &x in a {
        print!("{} ", x);
    }
    print!("\u{0008}]");
}

fn main() {
    let lim = 1000035;
    let mut sv: Vec<Bool> = vec![false; lim];
    sieve(&mut sv, lim);

}