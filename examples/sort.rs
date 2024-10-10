fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut tmp = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut tmp);
    arr.copy_from_slice(&tmp);
}

fn merge(left: &[i32], right: &[i32], res: &mut [i32]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut res_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] < right[right_idx] {
            res[res_idx] = left[left_idx];
            left_idx += 1;
        } else {
            res[res_idx] = right[right_idx];
            right_idx += 1;
        }
        res_idx += 1;
    }

    while left_idx < left.len() {
        res[res_idx] = left[left_idx];
        left_idx += 1;
        res_idx += 1;
    }

    while right_idx < right.len() {
        res[res_idx] = right[right_idx];
        right_idx += 1;
        res_idx += 1;
    }
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = partition(arr);
    quick_sort(&mut arr[..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_idx = arr.len() - 1;
    let pivot = arr[pivot_idx];

    let mut i = 0;
    for j in 0..pivot_idx {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_idx);
    i
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut arr = [38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut arr = [38, 27, 43, 3, 9, 82, 10];
    println!("Original array: {:?}", arr);
    quick_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
