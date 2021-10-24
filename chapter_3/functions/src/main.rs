fn main() {
    let num1 = 5; // this is a statement
    let num2 = 6;
    /*
        let x = {num1 + num2}; 
        this is an expression
    */
    let res = add(num1, num2);
    println!("The sum of {} + {} is {}", num1, num2, res); // expected "The sum of 5 + 6 is 11"
}

fn add(x: i32, y: i32) -> i32 { // this function should return an i32 type
    x + y
}
