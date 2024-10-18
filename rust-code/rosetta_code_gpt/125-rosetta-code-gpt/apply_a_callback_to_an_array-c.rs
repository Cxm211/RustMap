fn coef(n: usize) -> Result<Vec<i64>, &'static str> {
    if n < 0 || n > 63 {
        return Err("Out of range");
    }

    let mut c = vec![0i64; n + 1];
    c[0] = 1;

    for i in 0..n {
        c[0] = -c[0];
        let mut j = i;
        c[j + 1] = 1;
        while j > 0 {
            c[j] = c[j - 1] - c[j];
            j -= 1;
        }
    }

    Ok(c)
}

fn is_prime(n: usize) -> Result<bool, &'static str> {
    let mut c = coef(n)?;

    c[0] += 1;
    c[n] -= 1;

    for &item in c.iter() {
        if item % n as i64 == 0 {
            return Ok(false);
        }
    }

    Ok(true)
}

fn show(c: &[i64]) {
    for (n, &coef) in c.iter().enumerate().rev() {
        print!("{:+}x^{}", coef, n);
    }
}

fn main() -> Result<(), &'static str> {
    for n in 0..10 {
        let c = coef(n)?;
        print!("(x-1)^{} = ", n);
        show(&c);
        println!();
    }

    print!("\nprimes (never mind the 1):");
    for n in 1..=63 {
        if is_prime(n)? {
            print!(" {}", n);
        }
    }

    println!();
    Ok(())
}