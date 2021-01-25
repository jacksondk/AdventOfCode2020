use std::collections::HashMap;

mod tests;

fn next_number(list: &Vec<u64>) -> u64 {
    let recent_number = *list.last().unwrap();
    let list_length = list.len();

    let mut index = list_length - 2;
    loop {
        let elem = list[index];
        if elem == recent_number {
            let new_element = (list_length - index - 1) as u64;
            return new_element;
        }
        if index == 0 {
            return 0;
        }
        index = index - 1;
    }
}

fn find_nth_number(list: &mut Vec<u64>, index: usize) -> u64 {
    loop {
        let next = next_number(list);
        list.push(next);
        if list.len() == index {
            return next;
        }
    }
}

fn next_number_2(last_in_list_map: &HashMap<u64, u64>, last_element: u64, index: usize) -> u64 {
    return if last_in_list_map.contains_key(&last_element) {
        // println!("Found {} at {} (at index {})", &last_element, last_in_list_map[&last_element], index);
        (index as u64) - last_in_list_map[&last_element] - 1
    } else {
        // println!("It is a first for {}", last_element);
        0
    };
}

fn find_nth_number_2(list: Vec<u64>, index: usize) -> u64 {
    let mut map: HashMap<u64, u64> = HashMap::new();

    for idx in 0..(list.len() - 1) {
        // println!(" Last {} at index {}", list[idx], idx);
        map.insert(list[idx], idx as u64);
    }
    let mut last_element = *list.last().unwrap();
    let mut current_index = list.len();

    println!("Starting loop");
    loop {
        // println!("Last seen is {}", last_element);
        let next_element = next_number_2(&map, last_element, current_index);
        // println!("Next element {}", next_element);
        current_index += 1;
        map.insert(last_element, (current_index - 2) as u64);
        // println!(" Last {} at index {}", last_element, current_index - 2);
        if current_index == index {
            return next_element;
        }
        last_element = next_element;
    }
}

fn main() {
    println!("Hello, world!");
}
