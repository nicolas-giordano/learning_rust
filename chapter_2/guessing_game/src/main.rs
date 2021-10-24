use std::io; // standard input output library to allow us to take in user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // creates an empty string type (::new is an associated function of String; these are applied to types)

    /*
        let allows us to create a variable. variables are typically immutable by default (more on mutability in chapter 3)   
        we can make a variable mutable by adding mut before the name of the variable
    */

    io::stdin() // without the use at top of file we would need std::io::stdin()
        .read_line(&mut guess) // read input from user; & indicates a reference (more on references in chapter 4)
        .expect("Failed to read line."); // handles enums that Rust has (more on enums in chapter 6); throws error if Err returned by read_line

    println!("You guessed {}", guess);
}
