pub fn insertion_sort(list: &mut Vec<i64>) {
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
