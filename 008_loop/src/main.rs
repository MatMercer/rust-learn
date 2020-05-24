fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter > 10000 {
            break counter - 1;
        }
    };

    println!("Iterated {} times", result);
}
