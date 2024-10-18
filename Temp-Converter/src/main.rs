use std::io;
fn main() {
    println!("1: C to F | 2: F to C");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Not a valid choice!");
    let trimmed = choice.trim();
    let input: Result<i32, _> = trimmed.trim().parse();
    match input {
        Ok(1) => c_to_f(),
        Ok(2) => f_to_c(),
        _ => println!("Not a valid choice!"),
    }
}
fn c_to_f() {
    println!("Please enter the temperature in Celsius:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature!");

    let temp: Result<f32, _> = temp.trim().parse();

    match temp {
        Ok(celsius) => {
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("Celsius: {} | Fahrenheit: {}", celsius, fahrenheit);
        }
        Err(_) => println!("Invalid temperature input!"),
    }
}
fn f_to_c() {
    println!("Please enter the temperature in Fahrenheit:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature!");

    let temp: Result<f32, _> = temp.trim().parse();

    match temp {
        Ok(fahrenheit) => {
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("Celsius: {} | Fahrenheit: {}", fahrenheit, celsius);
        }
        Err(_) => println!("Invalid temperature input!"),
    }
}
