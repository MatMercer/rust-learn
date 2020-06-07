use std::fs::File;
use objectstore::*;
mod objectstore;

fn main() {
    //explode();
    load_file("delete_me.txt");
    //objectstore::load_file("/etc/shadow"); // permission denied if not root
    let f = File::open("doesnt_exists.txt");
    //f.expect("Failed to open doesnt_exists.txt"); // panics with custom msg
    

    println!("{}", read_str_from_file("/etc/passwd").expect("failed to read file"));
    println!("{}", read_str_from_file("/etc/noexistent").expect("failed to read file"));

    //let f = File::open("doesnt_exists.txt")?; // cant be use here, main returns ()
}

fn explode() {
    panic!("BOOOM 💥");
}
