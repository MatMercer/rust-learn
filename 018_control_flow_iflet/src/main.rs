fn main() {
    let u8value = Some(7u8);

    match u8value {
        Some(7) => println!("matches"),
        _ => (),
    }

    if let Some(7) = u8value {
        println!("matches too");
    }
}
