struct Range {
    start: usize,
    end: usize,
    sum: i32,
}

fn max_subseq(sequence: &[i32]) -> Range {
    let mut max_sum = 0;
    let mut this_sum = 0;
    let mut start = 0;
    let mut end = 0;

    for (j, &item) in sequence.iter().enumerate() {
        this_sum += item;
        if this_sum < 0 {
            start = j + 1;
            this_sum = 0;
        } else if this_sum > max_sum {
            max_sum = this_sum;
            end = j;
        }
    }

    let r = if start <= end {
        Range {
            start,
            end: end + 1,
            sum: max_sum,
        }
    } else {
        Range { start: 0, end: 0, sum: 0 }
    };

    r
}

fn main() {
    let a = [-1, -2, 3, 5, 6, -2, -1, 4, -4, 2, -1];
    let r = max_subseq(&a);
    
    println!("Max sum = {}", r.sum);
    for i in r.start..r.end {
        print!("{} ", a[i]);
    }
    println!();
}