type SetT = u32; // probably 32 bits; change according to need

fn show_set(x: SetT, name: &str) {
    print!("{} is:", name);
    for i in 0..32 {
        if x & (1 << i) != 0 {
            print!(" {}", i);
        }
    }
    println!();
}

fn main() {
    let mut a: SetT = 0; // empty set
    for i in (0..10).step_by(3) { // add 0 3 6 9 to set a
        a |= 1 << i;
    }
    show_set(a, "a");

    for i in 0..5 {
        println!("\t{} is{} in set a", i, if a & (1 << i) != 0 { "" } else { " not" });
    }

    let mut b = a;
    b |= 1 << 5; b |= 1 << 10; // b is a plus 5, 10
    b &= !(1 << 0); // sans 0
    show_set(b, "b");

    let c: SetT;
    show_set(a | b, "union(a, b)");
    show_set(c = a & b, "c = common(a, b)");
    show_set(a & !b, "a - b"); // diff, not arithmetic minus
    show_set(b & !a, "b - a");
    println!("b is{} a subset of a", if b & !a == 0 { "" } else { " not" });
    println!("c is{} a subset of a", if c & !a == 0 { "" } else { " not" });

    println!("union(a, b) - common(a, b) {} union(a - b, b - a)",
             if (a | b) & !(a & b) == (a & !b) | (b & !a) { "equals" } else { "does not equal" });
}