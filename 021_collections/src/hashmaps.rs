use std::collections::HashMap;

pub fn hashmap_example() {
    let mut scores = HashMap::new();

    let owned_str = String::from("Blue");
    scores.insert(owned_str, 10); // str is moved here, since it doesn't have the copy trait
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let collected_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &collected_scores {
        println!("{}: {}", key, value);
    }

    println!("avg of 1, 4, 7, 9, 10 is {}", avg(&vec![1, 4, 7, 9, 10]));
}

fn avg(vec: &Vec<i32>) -> f32 {
    let sum = vec.iter().fold(0, |sum, x| sum +x);
    return sum as f32 / (vec.len() as f32);
}

