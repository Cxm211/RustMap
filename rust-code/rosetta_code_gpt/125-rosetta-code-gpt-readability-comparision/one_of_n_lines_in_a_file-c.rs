fn main() {
    const ANIMALS: [&str; 8] = [
        "fly",
        "spider",
        "bird",
        "cat",
        "dog",
        "goat",
        "cow",
        "horse",
    ];
    const VERSES: [&str; 8] = [
        "I don't know why she swallowed that fly.\nPerhaps she'll die",
        "That wiggled and jiggled and tickled inside her",
        "How absurd, to swallow a bird",
        "Imagine that. She swallowed a cat",
        "What a hog to swallow a dog",
        "She just opened her throat and swallowed that goat",
        "I don't know how she swallowed that cow",
        "She's dead of course",
    ];

    for i in 0..ANIMALS.len() {
        println!(
            "There was an old lady who swallowed a {}\n{}",
            ANIMALS[i], VERSES[i]
        );
        for j in (0..=i).rev() {
            if i < ANIMALS.len() - 1 {
                if j > 0 {
                    println!(
                        "She swallowed the {} to catch the {}",
                        ANIMALS[j], ANIMALS[j - 1]
                    );
                    if j == 1 {
                        println!("{}", VERSES[0]);
                    }
                }
            }
        }
    }
}