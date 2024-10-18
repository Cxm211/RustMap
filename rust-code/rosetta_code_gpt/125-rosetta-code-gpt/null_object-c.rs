fn main() {
    let object: Option<&char> = None;

    if object.is_none() {
        println!("object is null");
    }
}