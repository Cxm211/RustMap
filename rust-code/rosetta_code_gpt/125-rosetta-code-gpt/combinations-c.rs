type Marker = u64;
const ONE: Marker = 1;

fn comb(pool: usize, need: usize, chosen: Marker, at: usize) {
    if pool < need + at {
        return;
    }

    if need == 0 {
        for i in 0..pool {
            if (chosen & (ONE << i)) != 0 {
                print!("{} ", i);
            }
        }
        println!();
        return;
    }

    comb(pool, need - 1, chosen | (ONE << at), at + 1);
    comb(pool, need, chosen, at + 1);
}

fn main() {
    comb(5, 3, 0, 0);
}