fn main() {
    for n in (3..=99).rev() {
        println!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, {} bottles of beer on the wall.\n",
            n, n, n - 1
        );
    }

    println!(
        "2 bottles of beer on the wall, 2 bottles of beer.\n\
        Take one down and pass it around, 1 bottle of beer on the wall.\n\n\
        1 bottle of beer on the wall, 1 bottle of beer.\n\
        Take one down and pass it around, no more bottles of beer on the wall.\n\n\
        No more bottles of beer on the wall, no more bottles of beer.\n\
        Go to the store and buy some more, 99 bottles of beer on the wall."
    );
}