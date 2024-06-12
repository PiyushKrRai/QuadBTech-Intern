use std::io::{self, Write};

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}
