use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Find the good number between 1 and 100!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut gess = String::new();

        io::stdin()
            .read_line(&mut gess)
            .expect("Failed to read line");

        let gess: u32 = match gess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {gess}");

        match gess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}