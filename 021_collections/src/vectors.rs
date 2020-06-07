pub fn vector_example() {
    let mut v: Vec<i32> = Vec::new();
    let elVec = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    let forth: Option<&i32> = v.get(3);

    borrow_example();
    enum_collections();

    for i in &v {
        println!("{}", i);
    }

    let mut mut_v = vec![100, 32, 57];

    for i in &mut mut_v {
        *i += 50;
    }

    for i in &mut_v {
        println!("{}", i);
    }
}

fn borrow_example() {
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // error - immutable borrow occurs here
    
    v.push(6); // error - mutable borrow occurs here
} // error - immutable borrow ends here

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_collections() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(64.646464),
    ];
}

