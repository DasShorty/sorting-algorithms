fn main() {
    println!("Hello, world!");
}

fn merge_sort(list: &mut Vec<i64>) {
    let mut groups: Vec<Vec<&mut i64>> = Vec::new();

    for element in list {
        let element_list = vec![element];
        groups.push(element_list);
    }

    let groups_length = groups.len();

    while groups_length > 1 {
        merge(&mut groups);
    }
}

fn merge(mut groups: &mut Vec<Vec<&mut i64>>) -> Vec<i64> {
    let mut left_list: Vec<&mut i64> = groups.remove(0);
    let mut right_list: Vec<&mut i64> = groups.remove(0);
    let mut temp: Vec<i64> = Vec::new();

    while !left_list.is_empty() && !right_list.is_empty() {
        if left_list.get(0) < right_list.get(0) {
            temp.push(*left_list.remove(0));
        } else {
            temp.push(*right_list.remove(0));
        }
    }

    temp
}
