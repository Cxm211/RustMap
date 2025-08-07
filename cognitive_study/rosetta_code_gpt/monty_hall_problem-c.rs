const MAX_SIZE: usize = 100;

fn move_to_front(table: &mut Vec<char>, c: char) -> usize {
    let index = table.iter().position(|&x| x == c).unwrap();
    table.remove(index);
    table.insert(0, c);
    index
}

fn decode(pass: &[usize], size: usize, table: &mut Vec<char>, sym: &mut Vec<char>) {
    for &x in &pass[0..size] {
        let c = table[x];
        let index = move_to_front(table, c);
        assert_eq!(x, index, "there is an error");
        sym.push(c);
    }
}

fn encode(sym: &str, table: &mut Vec<char>, pass: &mut Vec<usize>) {
    for c in sym.chars() {
        let index = move_to_front(table, c);
        pass.push(index);
    }
}

fn check(sym: &str, pass: &[usize], table: &mut Vec<char>) -> bool {
    let size = sym.len();
    let mut pass2 = vec![0; size];
    let mut sym2 = String::new();
    encode(sym, table, &mut pass2);
    decode(&pass2, size, table, &mut sym2.chars().collect::<Vec<_>>());

    pass == pass2 && sym == sym2
}

fn main() {
    let mut sym = ["broood", "bananaaa", "hiphophiphop"];
    let mut pass: Vec<usize> = vec![0; MAX_SIZE];
    let mut table: Vec<char> = ('a'..='z').collect();

    for s in sym.iter_mut() {
        let len = s.len();
        encode(s, &mut table, &mut pass);
        println!("{}: {:?}", s, &pass[0..len]);

        if check(s, &pass[0..len], &mut table) {
            println!("Correct :)");
        } else {
            println!("Incorrect :(");
        }
    }
}