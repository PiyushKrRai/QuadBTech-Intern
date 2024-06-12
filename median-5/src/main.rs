use std::io::{self, Write};

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        panic!("Array cannot be empty");
    }
    if len % 2 == 0 {
        // If even, the median is the average of the two middle numbers
        (arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0
    } else {
        // If odd, the median is the middle number
        arr[len / 2] as f64
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter a list of sorted integers separated by spaces: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    if nums.is_empty() {
        println!("No numbers entered.");
    } else {
        let med = median(&nums);
        println!("The median is: {}", med);
    }
}
