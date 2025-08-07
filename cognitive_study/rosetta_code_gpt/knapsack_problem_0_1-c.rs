#[derive(Debug)]
struct Item<'a> {
    name: &'a str,
    weight: i32,
    value: i32,
}

fn knapsack(items: &[Item], w: i32) -> Vec<i32> {
    let n = items.len();
    let mut mm = vec![0; (n + 1) * (w as usize + 1)];
    let mut m: Vec<Vec<i32>> = Vec::new();
    m.push(mm[0..(w + 1) as usize].to_vec());

    for i in 1..=n {
        let mut row = vec![0; (w + 1) as usize];
        for j in 0..=w {
            if items[i - 1].weight > j {
                row[j as usize] = m[i - 1][j as usize];
            } else {
                let a = m[i - 1][j as usize];
                let b = m[i - 1][(j - items[i - 1].weight) as usize] + items[i - 1].value;
                row[j as usize] = std::cmp::max(a, b);
            }
        }
        m.push(row);
    }

    let mut s = vec![0; n];
    let mut j = w;
    for i in (1..=n).rev() {
        if m[i][j as usize] > m[i - 1][j as usize] {
            s[i - 1] = 1;
            j -= items[i - 1].weight;
        }
    }

    s
}

fn main() {
    let items = [
        Item { name: "map", weight: 9, value: 150 },
    ];

    let s = knapsack(&items, 400);
    let mut total_weight = 0;
    let mut total_value = 0;

    for (i, &picked) in s.iter().enumerate() {
        if picked == 1 {
            println!("{: <22} {: >5} {: >5}", items[i].name, items[i].weight, items[i].value);
            total_weight += items[i].weight;
            total_value += items[i].value;
        }
    }

    println!("{: <22} {: >5} {: >5}", "totals:", total_weight, total_value);
}