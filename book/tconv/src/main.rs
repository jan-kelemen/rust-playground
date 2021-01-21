use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Invalid number of arguments!");
    }

    let temp: i32 = args[1].parse().expect("Enter a number!");

    let converted = match args[2].to_lowercase().as_ref() {
        "c" => c2f(temp),
        "f" => f2c(temp),
        _ => panic!("Unknown unit!"),
    };

    println!("Converted temperature is: {}", converted);
}

fn f2c(f: i32) -> f64 {
    let f = f as f64;

    (f - 32.) / 1.8
}

fn c2f(c: i32) -> f64 {
    let c = c as f64;

    c * 1.8 + 32.
}
