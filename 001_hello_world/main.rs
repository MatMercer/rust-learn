fn main() {
    println!("Hello, {}!", "Rust"); // println! calls a macro
    println!("{}", format!("Hello, {}!", "Macro")); // format! is a macro too and println seems to extend it
}
