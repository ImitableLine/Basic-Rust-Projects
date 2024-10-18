use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = 7;
    println!("Guess the number!");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed lower!"),
            Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
            Ordering::Greater => println!("You guessed higher!"),
        }
    }
}
