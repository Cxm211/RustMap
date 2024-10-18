use std::f64;

type PFunc = fn(f64, f64, f64, &dyn Fn(f64) -> f64) -> f64;
type RFunc = fn(f64) -> f64;

fn int_leftrect(from: f64, to: f64, n: f64, func: &dyn Fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x);
        x += h;
    }
    h * sum
}

fn int_rightrect(from: f64, to: f64, n: f64, func: &dyn Fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h);
        x += h;
    }
    h * sum
}

fn int_midrect(from: f64, to: f64, n: f64, func: &dyn Fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = 0.0;
    let mut x = from;
    while x <= to - h {
        sum += func(x + h / 2.0);
        x += h;
    }
    h * sum
}

fn int_trapezium(from: f64, to: f64, n: f64, func: &dyn Fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum = func(from) + func(to);
    for i in 1..(n as i64) {
        sum += 2.0 * func(from + i as f64 * h);
    }
    h * sum / 2.0
}

fn int_simpson(from: f64, to: f64, n: f64, func: &dyn Fn(f64) -> f64) -> f64 {
    let h = (to - from) / n;
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..(n as i64) {
        sum1 += func(from + h * i as f64 + h / 2.0);
    }
    for i in 1..(n as i64) {
        sum2 += func(from + h * i as f64);
    }
    h / 6.0 * (func(from) + func(to) + 4.0 * sum1 + 2.0 * sum2)
}

fn main() {
    let methods: [PFunc; 5] = [
        int_leftrect,
        int_rightrect,
        int_midrect,
        int_trapezium,
        int_simpson,
    ];

    let method_names = ["leftrect", "rightrect", "midrect", "trapezium", "simpson"];

    let funcs: [RFunc; 3] = [
        |x| x,
        |x| 1.0 / x,
        |x| x * x * x,
    ];

    let intg = |f: RFunc, a: f64, b: f64| f(b) - f(a);

    for (j, &func) in funcs.iter().enumerate() {
        for (i, &method) in methods.iter().enumerate() {
            let ic = method(0.0, 1.0, 1000.0, &func);
            println!(
                "{} [0, 1] num: {}, an: {}",
                method_names[i],
                ic,
                intg(func, 0.0, 1.0)
            );
        }
        println!();
    }
}