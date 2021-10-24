use std::io; // standard input output library to allow us to take in user input
use std::cmp::Ordering;
use rand::Rng; // get methods of random number generators from rand crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // creates an empty string type (::new is an associated function of String; these are applied to types)
    
        /*
            let allows us to create a variable. variables are typically immutable by default (more on mutability in chapter 3)   
            we can make a variable mutable by adding mut before the name of the variable
        */
    
        io::stdin() // without the use at top of file we would need std::io::stdin()
            .read_line(&mut guess) // read input from user; & indicates a reference (more on references in chapter 4)
            .expect("Failed to read line."); // handles enums that Rust has (more on enums in chapter 6); throws error if Err returned by read_line
    
        let guess: u32 = match guess.trim().parse() { // use of match instead of .expect() to handle the error instead of crashing on error
            Ok(num) => num,
            Err(_) => continue, // the '_' is a catch all value for errors
        };
        // we create a "shadow" of a variable we wish to change the type of (more on shadowing in chapter 3)
        
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less  => println!("Too small"),
            Ordering::Greater  => println!("Too Large"),
            Ordering::Equal  => {
                println!("You win!");
                break;
            },
        }
    }
}
