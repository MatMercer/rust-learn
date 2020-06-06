fn main() {
    let some_number = Some(5);
    let some_str = Some("A simple String");
    let absent: Option<u32> = None;

    println!("Double = {}", some_number.map_or(999, |n| n * 2));
    println!("Absent = {}", some_str.map_or("Error", |s| s.to_string().reverse()));
    println!("Absent = {}", absent.unwrap_or(50));
}
