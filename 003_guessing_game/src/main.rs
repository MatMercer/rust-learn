use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let right_guess = rand::thread_rng() // get a random generator that is unique to the current thread
        .gen_range(1, 101); // It’s inclusive on the lower bound but exclusive on the upper bound

    loop {
        // Ask the user for a guess
        print!("Write the guess: ");
        io::stdout().flush();
        let mut guess = String::new(); // associated function new, some languages call this a static method. new is associated with the type String
        io::stdin() // can be writen as std::io:stdin()
            .read_line(&mut guess) // guess needs to be passed as mutable to have it's contents changed. without &, the guess would be copied instead of passed as reference. like variables, references are immutable by default. that's why it's a mut there.
            .expect("Failed to read the number"); // every function in rust returns a value, like in lisp. this returs io::Result, which has the method expect. Expect is a enum both of Err or Ok. if Err, program stops with message in expect. if Ok, program continues and expect returns the same Ok. in this case, it's the number of byte read. without expect, there will be compile warnings

        let guess: u32 = match guess // shadow the value of guess to a new one
            .trim()
            .parse() /* parses to a type, in this case, u32 */ {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number!");
                    continue;
                },
        };
        match guess.cmp(&right_guess) {                     // match expression
            Ordering::Less => println!("Too small!"),      // made of arms
            Ordering::Greater /* pattern of the arm */ => println!("Too big!"), /* code that will execute of the arm */
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
