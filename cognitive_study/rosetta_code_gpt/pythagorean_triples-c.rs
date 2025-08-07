type Xint = u64;

const U: [[Xint; 9]; 3] = [
    [1, -2, 2,  2, -1, 2,  2, -2, 3],
    [1,  2, 2,  2,  1, 2,  2,  2, 3],
    [-1, 2, 2, -2,  1, 2, -2,  2, 3]
];

fn new_tri(in_arr: [Xint; 3], max_peri: Xint) -> (Xint, Xint) {
    let mut total = 0;
    let mut prim = 0;
    let p = in_arr[0] + in_arr[1] + in_arr[2];

    if p > max_peri {
        return (total, prim);
    }

    prim += 1;
    total += max_peri / p;

    for i in 0..3 {
        let t = [
            U[i][0] * in_arr[0] + U[i][1] * in_arr[1] + U[i][2] * in_arr[2],
            U[i][3] * in_arr[0] + U[i][4] * in_arr[1] + U[i][5] * in_arr[2],
            U[i][6] * in_arr[0] + U[i][7] * in_arr[1] + U[i][8] * in_arr[2]
        ];
        let (sub_total, sub_prim) = new_tri(t, max_peri);
        total += sub_total;
        prim += sub_prim;
    }
    (total, prim)
}

fn main() {
    let seed = [3, 4, 5];
    let mut max_peri = 10;

    while max_peri <= 100_000_000 {
        let (total, prim) = new_tri(seed, max_peri);
        println!("Up to {}: {} triples, {} primitives.", max_peri, total, prim);
        max_peri *= 10;
    }
}