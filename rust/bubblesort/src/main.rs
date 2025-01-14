fn main() {
    let mut data = generate_data();
    // println!("Before sorting: {:?}", data);

    let start_time = std::time::Instant::now();

    bubble_sort(&mut data);

    let end_time = start_time.elapsed();

    // println!("After sorting: {:?}", data);
    println!("Time taken: {:?}", end_time);
}

fn generate_data() -> Vec<i64> {
    let mut data = Vec::new();
    for _i in 0..1000 {
        data.push(rand::random());
    }
    data
}

fn bubble_sort(objects: &mut Vec<i64>) {
    let mut size = objects.len();

    let mut swapped = true;

    while swapped {
        swapped = false;
        for element in 1..size {
            if objects[element - 1] > objects[element] {
                objects.swap(element - 1, element);
                swapped = true;
            }
        }
        size -= 1;
    }
}
