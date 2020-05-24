fn main() {
    let s = String::from("World");
    borrow(&s);
    steal(s);
    // borrow(&s); value moved, steal got the reference for him
    
    {
        let mut x = 4;
    }
    // x = 5; x died above
}

fn borrow(string: &String) {
    println!("Hello {}", string);
}

fn steal(string: String) {
    println!("Haha! I stealed your {} string", string); // s from line 5, died here, RIP s :(
}
