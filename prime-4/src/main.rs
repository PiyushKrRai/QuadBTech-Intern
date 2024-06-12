use std::io::{self, Write};

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    match input.parse::<u32>() {
        Ok(n) => {
            if is_prime(n) {
                println!("{} is a prime number.", n);
            } else {
                println!("{} is not a prime number.", n);
            }
        },
        Err(_) => println!("Please enter a valid positive integer."),
    }
}
