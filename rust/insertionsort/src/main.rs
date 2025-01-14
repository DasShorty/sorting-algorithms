fn main() {

    let mut data = generate_data();

    println!("Data before sorting: {:?}", data);

    let start_time = std::time::Instant::now();

    insertion_sort(&mut data);

    let end_time = start_time.elapsed();

    println!("Data after sorting: {:?}", data);
    println!("Time taken to sort: {:?}", end_time);

}

fn generate_data() -> Vec<i64> {
    let mut data = Vec::new();

    for _i in 0..1000 {
        data.push(rand::random());
    }

    data
}

fn insertion_sort(list: &mut Vec<i64>) {
    for element_position in 1..list.len() {
        let value = list[element_position];
        let mut current_element_position = element_position;

        while (current_element_position > 0) && (list[current_element_position - 1] > value) {
            let temp_value = list[current_element_position - 1];
            list[current_element_position] = temp_value;

            current_element_position -= 1;
        }

        list[current_element_position] = value;
    }
}
