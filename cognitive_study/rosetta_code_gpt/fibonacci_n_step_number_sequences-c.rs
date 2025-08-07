fn anynacci(seed_array: Vec<i32>, how_many: usize) -> Vec<i32> {
    let mut result = vec![0; how_many];
    let initial_cardinality = seed_array.len();

    for i in 0..initial_cardinality {
        result[i] = seed_array[i];
    }

    for i in initial_cardinality..how_many {
        result[i] = 0;
        for j in i - initial_cardinality..i {
            result[i] += result[j];
        }
    }

    result
}

fn main() {
    let fibo = vec![1, 1];
    let tribo = vec![1, 1, 2];
    let tetra = vec![1, 1, 2, 4];
    let luca = vec![2, 1];

    let fibonacci = anynacci(fibo, 10);
    let tribonacci = anynacci(tribo, 10);
    let tetranacci = anynacci(tetra, 10);
    let lucas = anynacci(luca, 10);

    println!("\nFibonacci\tTribonacci\tTetranacci\tLucas");
    for i in 0..10 {
        println!("\n{}\t\t{}\t\t{}\t\t{}", fibonacci[i], tribonacci[i], tetranacci[i], lucas[i]);
    }
}