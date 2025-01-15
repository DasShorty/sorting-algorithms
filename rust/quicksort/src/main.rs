use std::time::Instant;

fn main() {
    let mut data = generate_data();

    println!("Unsorted data: {:?}", data);

    let length = data.len();

    let start_time = Instant::now();

    quicksort(&mut data, 0, (length - 1) as isize);

    let elapsed_time = start_time.elapsed();

    println!("Sorted data: {:?}", data);
    println!("Time taken: {:?}", elapsed_time);
}

fn generate_data() -> Vec<i64> {
    let mut data: Vec<i64> = Vec::new();

    for _i in 0..1000 {
        data.push(rand::random());
    }

    data
}

fn quicksort(arr: &mut Vec<i64>, left: isize, right: isize) {
    if left <= right {
        let partition_idx = partition(arr, left, right);

        quicksort(arr, left, partition_idx - 1);
        quicksort(arr, partition_idx + 1, right);
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
