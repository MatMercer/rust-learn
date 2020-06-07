pub fn string_example() {
    let mut s = String::new();

    let data = "some string";

    let s = data.to_string(); // str implements the Display trait, so this works

    println!("{}", data);

    let hello = String::from("Olá, as strings suportam UTF-8. á é í ó ú ç");
    println!("{}", hello);

    let mut recursive = String::from("recursive(null)");
    for i in 0..50 {
        recursive.insert_str(0, "recursive(");
        recursive.push_str(")");
    }

    println!("{}", recursive);

    concatenation_borrow();
    string_bytes("abc ç");
}

fn concatenation_borrow() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is dead. looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t: the implementation is more efficient than copying.

    println!("{}", s3);

    let s1 = String::from("Hello, ");

    let s4 = format!("{}{}", s1, s2); // same thing as above but doens't take ownership
    println!("{}", s4);
}

fn string_bytes(s: &str) {
    let mut chars: Vec<char> = vec![];
    let mut bytes: Vec<u8> = vec![];

    for c in s.chars() { // ç takes 2 bytes
        chars.push(c);
    }

    for b in s.bytes() { // ç takes 2 bytes
        bytes.push(b);
    }

    for i in 0..bytes.len() {
        println!("{} {}", bytes[i], chars.get(i).unwrap_or(&' '));
    }

}

