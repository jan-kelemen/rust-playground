use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Invalid number of arguments!");
    }

    let n: i32 = args[1].parse().expect("Enter a number!");

    println!("{}-th Fibonnacci number is: {}", n, fib(n as i64));
}

fn fib(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
