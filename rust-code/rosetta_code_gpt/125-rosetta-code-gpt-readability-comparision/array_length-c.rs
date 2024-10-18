fn array_concat<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut c = Vec::with_capacity(a.len() + b.len());
    c.extend_from_slice(a);
    c.extend_from_slice(b);
    c
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![6, 7, 8, 9, 0];

    let c = array_concat(&a, &b);

    for i in c.iter() {
        println!("{}", i);
    }
}