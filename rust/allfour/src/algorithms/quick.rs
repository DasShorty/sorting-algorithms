pub fn quick_sort(arr: &mut Vec<i64>, left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, left, right);

        quick_sort(arr, left, partition_idx - 1);
        quick_sort(arr, partition_idx + 1, right);
    }
}

fn partition(arr: &mut Vec<i64>, left: isize, right: isize) -> isize {
    let pivot = right;
    let mut left_pos: isize = left - 1;

    for courser in left..=right - 1 {
        if arr[courser as usize] <= arr[pivot as usize] {
            left_pos += 1;
            arr.swap(left_pos as usize, courser as usize);
        }
    }

    arr.swap((left_pos + 1) as usize, pivot as usize);

    left_pos + 1
}