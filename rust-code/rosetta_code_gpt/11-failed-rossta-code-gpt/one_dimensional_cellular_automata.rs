fn evolve(cell: &mut Vec<char>, backup: &mut Vec<char>) -> i32 {
    let trans = ['_', '_', '_', '#', '_', '#', '#', '_'];
    let mut diff = 0;

    for i in 0..cell.len() {
        let index = ((if cell.get(i.wrapping_sub(1)) == Some(&'#') { 4 } else { 0 }) +
                     (if cell.get(i) == Some(&'#') { 2 } else { 0 }) +
                     (if cell.get(i + 1) == Some(&'#') { 1 } else { 0 })) as usize;
        backup[i] = trans[index];
        if backup[i] != cell[i] {
            diff += 1;
        }
    }

    cell.copy_from_slice(backup);
    diff
}

fn main() {
    let mut c = vec!['_', '#', '#', '#', '_', '#', '#', '_', '#', '_', '#', '_', '#', '_', '_', '#', '#', '_', '_', '#'];
    let mut b = vec!['_'; 19];

    loop {
        println!("{}", c.iter().collect::<String>());
        if evolve(&mut c, &mut b) == 0 {
            break;
        }
    }
}