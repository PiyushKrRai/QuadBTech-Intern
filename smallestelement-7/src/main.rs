fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn quickselect(arr: &mut [i32], k: usize) -> i32 {
    let mut low = 0;
    let mut high = arr.len() - 1;
    while low <= high {
        let pivot_index = partition(arr, low, high);
        if pivot_index == k {
            return arr[k];
        } else if pivot_index < k {
            low = pivot_index + 1;
        } else {
            high = pivot_index - 1;
        }
    }
    panic!("k is out of range");
}

fn main() {
    let mut arr = vec![3, 7, 2, 1, 5, 4, 6];
    let k = 2; // kth smallest element to find

    let kth_smallest = quickselect(&mut arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
