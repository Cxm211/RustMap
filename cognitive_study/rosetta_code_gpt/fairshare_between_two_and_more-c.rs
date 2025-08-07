fn turn(base: i32, mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        let rem = n % base;
        n = n / base;
        sum += rem;
    }
    sum % base
}

fn fairshare(base: i32, count: i32) {
    print!("Base {:2}:", base);
    for i in 0..count {
        let t = turn(base, i);
        print!(" {:2}", t);
    }
    println!();
}

fn turn_count(base: i32, count: i32) {
    let mut cnt = vec![0; base as usize];
    let (mut min_turn, mut max_turn, mut portion) = (i32::MAX, i32::MIN, 0);

    for i in 0..count {
        let t = turn(base, i);
        cnt[t as usize] += 1;
    }

    for &c in &cnt {
        if c > 0 {
            portion += 1;
        }
        min_turn = min_turn.min(c);
        max_turn = max_turn.max(c);
    }

    print!("  With {} people: ", base);
    if min_turn == 0 {
        println!("Only {} have a turn", portion);
    } else if min_turn == max_turn {
        println!("{}", min_turn);
    } else {
        println!("{} or {}", min_turn, max_turn);
    }
}

fn main() {
    fairshare(2, 25);
    fairshare(3, 25);
    fairshare(5, 25);
    fairshare(11, 25);

    println!("How many times does each get a turn in 50000 iterations?");
    turn_count(191, 50000);
    turn_count(1377, 50000);
    turn_count(49999, 50000);
    turn_count(50000, 50000);
    turn_count(50001, 50000);
}