use std::io;

fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn main() {
    let mut numbers = Vec::new();
    let mut input = String::new();

    loop {
        println!("Enter a number (stop = quit)");

        input.clear();

        io::stdin().read_line(&mut input).expect("User input failed");
        if input.trim() == "stop" {
            break;
        }

        let number: i32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Entrer a number !");
                continue;
            }
        };

        numbers.push(number);
    }

    let result = sum(&numbers);
    println!("The sum is {}", result);
}
