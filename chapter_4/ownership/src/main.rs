fn main() {
    // more about method syntax in chapter 5
    let mut s = String::from("hello"); // method syntax for String type; creates a String from a string literal
    s.push_str(", world");
    println!("{}", s);

    let s2 = s.clone(); // this will make a deep copy of the String.println!
    println!("{} and {}",s, s2);

    goodByeString(s2);
    println!("{}", s2); // value has moved; value no longer exists in s2 and will give compile error.


}


fn goodByeString(s: String) {
    println!("The variable will no longer have any data after this function is called.");
}