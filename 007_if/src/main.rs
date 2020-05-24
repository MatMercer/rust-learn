fn main() {
    let number = 7;
    let forced_even = if 7 % 2 == 0 {
        number
    } else {
        number + 1
    };

    println!("{}", forced_even);
}
