pub fn slices() {
    let mut word = String::from("hello");

    add_hello(&mut word);

    println!("word is now {}", word);

    println!(" {}", first_word(&word));
}

fn first_word(word: &String) -> &str {
    let bytes = word.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[0..i];
        }
    }

    &word[..]
}

fn add_hello(word: &mut String) {
    word.push_str(" world");
}