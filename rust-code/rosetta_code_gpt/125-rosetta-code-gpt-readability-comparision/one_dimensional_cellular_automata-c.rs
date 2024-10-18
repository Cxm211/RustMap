fn main() {
    let animals = [
        "fly", 
        "spider", 
        "bird", 
        "cat", 
        "dog", 
        "goat", 
        "cow", 
        "horse"
    ];

    let verses = [
        "I don't know why she swallowed that fly.\nPerhaps she'll die\n",
        "That wiggled and jiggled and tickled inside her",
        "How absurd, to swallow a bird",
        "Imagine that. She swallowed a cat",
        "What a hog to swallow a dog",
        "She just opened her throat and swallowed that goat",
        "I don't know how she swallowed that cow",
        "She's dead of course"
    ];

    for i in 0..animals.len() {
        println!("There was an old lady who swallowed a {}\n{}\n", animals[i], verses[i]);
        for j in (1..=i).rev() {
            if i < animals.len() - 1 {
                println!("She swallowed the {} to catch the {}", animals[j], animals[j-1]);
                if j == 1 {
                    println!("{}\n", verses[0]);
                }
            }
        }
    }
}
