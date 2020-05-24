fn main() {
    let squares = String::from("Squares have 4 sides");
    let square = String::from("Square");
    let bug = String::from("Können since utf-8 may break the last char logic"); // https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/21

    println!("{} is the first word of the phrase '{}'", first_word(&squares), squares);
    println!("{} is the first word of the phrase '{}'", first_word(&square), square);
    println!("{} is the first word of the phrase '{}'", first_word(&bug), bug); // It works, since first_word returns 7
    println!("{} is the first word of the phrase '{}'", first_word("Testing a word"), "Testing a word");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

