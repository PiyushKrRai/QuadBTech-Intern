use std::io::{self, Write};

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = &strs[0][..];

    for s in strs.iter().skip(1) {
        while !s.starts_with(prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix = &prefix[..prefix.len() - 1];
        }
    }

    prefix.to_string()
}

fn main() {
    let mut input = String::new();
    print!("Enter a list of strings separated by spaces: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let strs: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    if strs.is_empty() {
        println!("No strings entered.");
    } else {
        let lcp = longest_common_prefix(&strs);
        println!("The longest common prefix is: \"{}\"", lcp);
    }
}
