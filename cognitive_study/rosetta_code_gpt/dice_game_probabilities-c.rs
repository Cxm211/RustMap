fn ipow(x: u32, y: u32) -> u64 {
    let mut result = 1u64;
    for _ in 1..=y {
        result *= x as u64;
    }
    result
}

fn min(x: u32, y: u32) -> u32 {
    if x < y { x } else { y }
}

fn throw_die(n_sides: u32, n_dice: u32, s: u32, counts: &mut Vec<u32>) {
    if n_dice == 0 {
        counts[s as usize] += 1;
        return;
    }

    for i in 1..=n_sides {
        throw_die(n_sides, n_dice - 1, s + i, counts);
    }
}

fn beating_probability(n_sides1: u32, n_dice1: u32, n_sides2: u32, n_dice2: u32) -> f64 {
    let len1 = (n_sides1 + 1) * n_dice1;
    let mut c1 = vec![0u32; len1 as usize];
    throw_die(n_sides1, n_dice1, 0, &mut c1);

    let len2 = (n_sides2 + 1) * n_dice2;
    let mut c2 = vec![0u32; len2 as usize];
    throw_die(n_sides2, n_dice2, 0, &mut c2);

    let p12 = ipow(n_sides1, n_dice1) * ipow(n_sides2, n_dice2) as f64;

    let mut tot = 0.0;
    for i in 0..len1 as usize {
        for j in 0..min(i, len2 as usize) {
            tot += c1[i] as f64 * c2[j] as f64 / p12;
        }
    }
    tot
}

fn main() {
    println!("{:1.16}", beating_probability(4, 9, 6, 6));
    println!("{:1.16}", beating_probability(10, 5, 7, 6));
}