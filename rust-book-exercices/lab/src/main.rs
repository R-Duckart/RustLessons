fn main() {
    let mut number = 1;
    let mut previous_number = 1;
    let mut tmp_number = 0;
    while number < 100{
        println!("number is {}", number);

        tmp_number = number;
        number += previous_number;
        previous_number = tmp_number;
    }
}

