fn main() {
    let rv = another_function(5, 6);
    println!("Return type of function is not optional: {}", rv);
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x + y
}

