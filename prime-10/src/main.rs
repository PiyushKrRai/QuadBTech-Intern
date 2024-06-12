use std::io::{self, Write};

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut divisor = 5;
    while divisor * divisor <= num {
        if num % divisor == 0 || num % (divisor + 2) == 0 {
            return false;
        }
        divisor += 6;
    }
    true
}

fn main() {
    let mut input = String::new();
    print!("Enter a number to check if it's prime: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    match input.parse::<u64>() {
        Ok(num) => {
            if is_prime(num) {
                println!("{} is a prime number.", num);
            } else {
                println!("{} is not a prime number.", num);
            }
        }
        Err(_) => println!("Please enter a valid positive integer."),
    }
}
