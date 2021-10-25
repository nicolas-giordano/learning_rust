fn main() {
    let mut s = String::from("Hello, World!");

    let len = calculate_len(&s); // this passes a reference to the String as an argument for the method
    println!("The length of {} is {}.", s, len); // this now works even though we used s

    
    change(&mut s); // With mutable references, you may only have one at a time to a specific location
    println!{"Here is the new string: {}", s};
}


fn calculate_len(s: &String) -> usize { // this function now takes a String reference rather than a String.
    s.len()

    // dereferencing is the opposite to this and can be achieved by using '*'. 
    // This will be covered in Chapter 8 and 15.
}

fn change(s: &mut String) {
    s.push_str(" This was added!");
}