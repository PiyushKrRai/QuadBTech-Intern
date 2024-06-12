
use std::io::{self, Write};
fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Ensures the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the input to remove any newline characters
    let trimmed_input = input.trim();

    if is_palindrome(trimmed_input) {
        println!("\"{}\" is a palindrome.", trimmed_input);
    } else {
        println!("\"{}\" is not a palindrome.", trimmed_input);
    }
}
