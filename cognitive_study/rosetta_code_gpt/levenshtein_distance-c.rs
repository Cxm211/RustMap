fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> usize {
    let mut a;
    let mut b;
    let mut c;

    if ls == 0 {
        return lt;
    }
    if lt == 0 {
        return ls;
    }

    if s.chars().nth(ls - 1) == t.chars().nth(lt - 1) {
        return levenshtein(&s[0..ls - 1], ls - 1, &t[0..lt - 1], lt - 1);
    }

    a = levenshtein(&s[0..ls - 1], ls - 1, &t[0..lt - 1], lt - 1);
    b = levenshtein(&s[0..ls], ls, &t[0..lt - 1], lt - 1);
    c = levenshtein(&s[0..ls - 1], ls - 1, &t[0..lt], lt);

    a = std::cmp::min(a, b);
    a = std::cmp::min(a, c);

    a + 1
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    println!(
        "distance between `{}` and `{}`: {}",
        s1,
        s2,
        levenshtein(s1, s1.len(), s2, s2.len())
    );
}