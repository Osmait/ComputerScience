use std::io;

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = hi;
    let mut i = lo;
    let mut j = hi - 1;

    loop {
        while arr[i] < arr[pivot] {
            i += 1;
        }
        while j > 0 && arr[j] > arr[pivot] {
            j -= 1;
        }
        if j == 0 || i >= j {
            break;
        } else if arr[i] == arr[j] {
            i += 1;
            j -= 1;
        } else {
            arr.swap(i, j);
        }
    }
    arr.swap(i, pivot);
    i
}
fn _quick_sort<T: Ord>(arr: &mut [T], mut lo: usize, mut hi: usize) {
    while lo < hi {
        let pivot = partition(arr, lo, hi);

        if pivot - lo < hi - pivot {
            if pivot > 0 {
                _quick_sort(arr, lo, pivot - 1)
            }
            lo = pivot + 1;
        } else {
            _quick_sort(arr, pivot + 1, hi);
            hi = pivot - 1;
        }
    }
}
fn quick_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, len - 1);
    }
}
fn main() {
    let mut input = String::new();
    println!("Enter number separated by a coma:");
    io::stdin()
        .read_line(&mut input)
        .expect("FIled to read line ");
    let mut list_of_number: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("please enter valid numbers"))
        .collect();
    quick_sort(&mut list_of_number);
    println!("{:?}", list_of_number)
}
