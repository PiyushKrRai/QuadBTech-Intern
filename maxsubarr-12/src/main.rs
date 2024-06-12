fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;

    for &num in arr {
        max_ending_here = max_ending_here + num;
        if max_ending_here < num {
            max_ending_here = num;
        }
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
    }

    max_so_far
}

fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]; // Example array

    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
