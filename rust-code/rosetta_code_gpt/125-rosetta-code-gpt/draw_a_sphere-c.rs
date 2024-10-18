use std::f64;
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
fn normalize(v: &mut [f64; 3]) {
    let len = f64::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    v[0] /= len;
    v[1] /= len;
    v[2] /= len;
}

fn dot(x: &[f64; 3], y: &[f64; 3]) -> f64 {
    let d = x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    if d < 0.0 {
        -d
    } else {
        0.0
    }
}

fn draw_sphere(r: f64, k: f64, ambient: f64) {
    let shades = ".:!*oe&#%@";
    let mut light = [30.0, 30.0, -50.0];
    normalize(&mut light);

    for i in (f64::floor(-r) as i32)..=(f64::ceil(r) as i32) {
        let x = i as f64 + 0.5;
        for j in (f64::floor(-2.0 * r) as i32)..=(f64::ceil(2.0 * r) as i32) {
            let y = j as f64 / 2.0 + 0.5;
            if x * x + y * y <= r * r {
                let mut vec = [x, y, f64::sqrt(r * r - x * x - y * y)];
                normalize(&mut vec);
                let mut b = f64::powf(dot(&light, &vec), k) + ambient;
                let mut intensity = ((1.0 - b) * (shades.len() as f64)) as usize;
                if intensity < 0 {
                    intensity = 0;
                }
                if intensity >= shades.len() {
                    intensity = shades.len() - 1;
                }
                print!("{}", shades.chars().nth(intensity).unwrap());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    draw_sphere(20.0, 4.0, 0.1);
    draw_sphere(10.0, 2.0, 0.4);
}