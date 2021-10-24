fn main() {
    // let x = if true { 5 } else { 6 }; // all branches in this manner should have the same return type
    // println!("The value of x is {}", x);

    // let mut count = 0;
    // 'counting: loop {
    //     println!("counting");
    //     count += 1;
    //     if count == 3 {
    //         break 'counting;
    //     }
    // }
    // println!("End count = {}", count);

    // return from a loop
    // let mut count = 0;
    // let res = loop {
    //     count += 1;
    //     if count == 10 {
    //         break count;
    //     }
    // };
    // println!("res: {}", res);

    // for loop (elements in an array)
    // let a = [1,2,3,4,5];
    // for ele in a {
    //     println!("ele: {}", ele);
    // }

    // for loop with a range
    for i in (1..4).rev() {
        println!("i: {}", i);
    }
}
