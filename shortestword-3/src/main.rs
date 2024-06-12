use std::io::{self, Write};

fn find_shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let mut input = String::new();
    print!("Enter a string: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    match find_shortest_word(trimmed_input) {
        Some(word) => println!("The shortest word is: \"{}\"", word),
        None => println!("No words found in the input."),
    }
}
