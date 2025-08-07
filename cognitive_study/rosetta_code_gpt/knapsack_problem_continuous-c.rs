#[derive(Debug)]
struct Item<'a> {
    w: f64,
    v: f64,
    name: &'a str,
}

fn item_cmp(a: &Item, b: &Item) -> std::cmp::Ordering {
    let ua = a.v / a.w;
    let ub = b.v / b.w;
    ub.partial_cmp(&ua).unwrap_or(std::cmp::Ordering::Equal)
}

fn main() {
    let mut items = [
        Item { w: 3.8, v: 36.0, name: "beef" },
    ];

    items.sort_by(item_cmp);

    let mut space = 15.0;
    for it in items.iter().rev() {
        if space >= it.w {
            println!("take all {}", it.name);
            space -= it.w;
        } else if space > 0.0 {
            println!("take {}kg of {} kg of {}", space, it.w, it.name);
            break;
        }
    }
}