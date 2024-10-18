fn main() {
    let mut is_open = [false; 100];

    for pass in 0..100 {
        for door in (pass..100).step_by(pass + 1) {
            is_open[door] = !is_open[door];
        }
    }

    for (door, &state) in is_open.iter().enumerate() {
        println!("door #{} is {}.", door + 1, if state { "open" } else { "closed" });
    }
}