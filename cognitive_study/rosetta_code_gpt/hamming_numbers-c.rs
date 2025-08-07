fn main() {
    let mut q: Vec<u64> = Vec::new();
    let mut alloc = 0;
    let mut n = 1;

    fn qpush(h: u64, q: &mut Vec<u64>, alloc: &mut usize, n: &mut usize) {
        let mut i;
        let mut j;

        if *alloc <= *n {
            *alloc = if *alloc > 0 { *alloc * 2 } else { 16 };
            q.resize(*alloc, 0);
        }

        i = *n;
        *n += 1;

        while {
            j = i / 2;
            j > 0 && q[j] > h
        } {
            q[i] = q[j];
            i = j;
        }
        q[i] = h;
    }

    fn qpop(q: &mut Vec<u64>, n: &mut usize) -> u64 {
        let mut i;
        let mut j;
        let mut r;
        let mut t;

        r = q[1];
        while *n > 1 && r == q[1] {
            i = 1;
            t = q[*n - 1];
            *n -= 1;

            while {
                j = i * 2;
                j < *n
            } {
                if j + 1 < *n && q[j] > q[j + 1] {
                    j += 1;
                }
                if t <= q[j] {
                    break;
                }
                q[i] = q[j];
                i = j;
            }
            q[i] = t;
        }
        r
    }

    qpush(1, &mut q, &mut alloc, &mut n);
    let mut h;
    for i in 1..=1691 {
        h = qpop(&mut q, &mut n);
        qpush(h * 2, &mut q, &mut alloc, &mut n);
        qpush(h * 3, &mut q, &mut alloc, &mut n);
        qpush(h * 5, &mut q, &mut alloc, &mut n);

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
    }
}
