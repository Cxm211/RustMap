#[derive(Clone, Copy)]
struct Ref {
    index: usize,
    last_char: char,
    first_char: char,
}

fn search(curr_len: usize, longest_path_refs: &mut Vec<Ref>, refs: &mut [Ref], n_solutions: &mut usize) {
    if curr_len == longest_path_refs.len() {
        *n_solutions += 1;
    } else if curr_len > longest_path_refs.len() {
        *n_solutions = 1;
        longest_path_refs.clear();
        longest_path_refs.extend_from_slice(&refs[0..curr_len]);
    }

    let last_char = refs[curr_len - 1].last_char;
    for i in curr_len..refs.len() {
        if refs[i].first_char == last_char {
            refs.swap(curr_len, i);
            search(curr_len + 1, longest_path_refs, refs, n_solutions);
            refs.swap(i, curr_len);
        }
    }
}

fn find_longest_chain(items: &[&str]) -> Vec<&str> {
    let mut refs = vec![];
    for (i, &item) in items.iter().enumerate() {
        refs.push(Ref {
            index: i,
            last_char: item.chars().last().unwrap(),
            first_char: item.chars().next().unwrap(),
        });
    }

    let mut longest_path_refs = vec![];
    let mut n_solutions = 0;
    for i in 0..items.len() {
        refs.swap(0, i);
        search(1, &mut longest_path_refs, &mut refs, &mut n_solutions);
        refs.swap(i, 0);
    }

    longest_path_refs.iter().map(|r| items[r.index]).collect()
}

fn main() {
    let pokemon = ["audino", "bagon", "baltoy", "banette", "bidoof", "braviary", "bronzor", "carracosta",
                   /* ... */ ];
    let longest_path = find_longest_chain(&pokemon);
    println!("Maximum path length: {}", longest_path.len());
    println!("Example path of that length: {:?}", longest_path);
}