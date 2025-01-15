use std::io::stdin;

pub fn read_console() -> String {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read line");

    input
}

pub enum SortingAlgorithm {
    BubbleSort,
    InsertionSort,
    MergeSort,
    QuickSort,
}

pub fn get_random_data() -> Vec<i64> {
    let mut data: Vec<i64> = Vec::new();

    for _i in 0..1000 {
        data.push(rand::random());
    }

    data
}

pub fn get_algorithm() -> SortingAlgorithm {
    let input = show_menu();
    match input.trim() {
        "1" => SortingAlgorithm::BubbleSort,
        "2" => SortingAlgorithm::InsertionSort,
        "3" => SortingAlgorithm::MergeSort,
        "4" => SortingAlgorithm::QuickSort,
        _ => get_algorithm(),
    }
}

fn show_menu() -> String {
    println!("Sorting algorithms implemented in Rust");

    println!("Enter the sorting algorithm you want to use: ");
    println!("1. Bubble Sort");
    println!("2. Insertion Sort");
    println!("3. Merge Sort");
    println!("4. Quick Sort");

    println!("Provide the number: ");

    read_console()
}
pub(crate) fn main() {
    println!("Loading lib.rs");
}
