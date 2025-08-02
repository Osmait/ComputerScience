fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let target = 3;
    match binary_search(&arr, target) {
        Some(index) => println!("Found target at index: {}", index),
        None => println!("Target not found in the array."),
    }

    let target_not_found = 6;
    match binary_search(&arr, target_not_found) {
        Some(index) => println!("Found target at index: {}", index),
        None => println!("Target not found in the array."),
    }
}

