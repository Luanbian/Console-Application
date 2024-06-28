pub fn example() {
    let text = String::from("Rust é uma linguagem de programação moderna");

    let word = first_word_with('l', &text);
    println!("A palavra é '{}'", word);
}


fn first_word_with(character: char, slice: &str) -> &str {
    let bytes = slice.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == character as u8 {
            return slice[i..].split_whitespace().next().unwrap();
        }
    }
    slice
}