fn qselect(v: &mut [i32], len: usize, k: usize) -> i32 {
    fn swap(v: &mut [i32], a: usize, b: usize) {
        let tmp = v[a];
        v[a] = v[b];
        v[b] = tmp;
    }

    let mut i;
    let mut st = 0;

    for i in 0..len - 1 {
        if v[i] > v[len - 1] {
            continue;
        }
        swap(v, i, st);
        st += 1;
    }

    swap(v, len - 1, st);

    if k == st {
        v[st]
    } else if st > k {
        qselect(&mut v[0..st], st, k)
    } else {
        qselect(&mut v[st..], len - st, k - st)
    }
}

fn main() {
    const N: usize = 10;
    let x = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let mut y = [0; N];

    for i in 0..N {
        y.copy_from_slice(&x); // qselect modifies array
        println!("{}: {}", i, qselect(&mut y, N, i));
    }
}