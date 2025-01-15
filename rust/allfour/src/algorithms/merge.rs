pub fn merge_sort(list: Vec<i64>) -> Vec<i64> {
    let mut groups: Vec<Vec<i64>> = Vec::new();

    for element in list {
        let element_list = vec![element];
        groups.push(element_list);
    }

    while groups.len() > 1 {
        let left_list = groups.remove(0);
        let right_list = groups.remove(0);

        groups.push(merge(left_list, right_list));
    }

    groups.remove(0)
}

fn merge(mut left_list: Vec<i64>, mut right_list: Vec<i64>) -> Vec<i64> {
    let mut temp: Vec<i64> = Vec::new();

    while !left_list.is_empty() && !right_list.is_empty() {
        if left_list.get(0) < right_list.get(0) {
            temp.push(left_list.remove(0));
        } else {
            temp.push(right_list.remove(0));
        }
    }

    temp.append(&mut left_list);
    temp.append(&mut right_list);

    temp
}
