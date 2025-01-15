pub fn bubble_sort(objects: &mut Vec<i64>) {
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
