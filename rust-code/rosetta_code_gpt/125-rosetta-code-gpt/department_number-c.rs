fn main() {
    println!("Police\tSanitation\tFire");
    println!("----------------------------");

    for police in (2..=6).step_by(2) {
        for sanitation in 1..=7 {
            for fire in 1..=7 {
                if police != sanitation && sanitation != fire && fire != police && police + fire + sanitation == 12 {
                    println!("{}\t\t{}\t\t{}", police, sanitation, fire);
                }
            }
        }
    }
}