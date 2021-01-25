use crate::{read_task, task1, task2, Bus, new_mod, chinese_remainder};

#[test]
fn load_test() {
    let task = read_task("example.txt");
    assert_eq!(939, task.minute);
    assert_eq!(8, task.busses.len());
}

#[test]
fn example_task_1() {
    let task = read_task("example.txt");

    let result = task1(&task);

    assert_eq!(295, result);
}

#[test]
fn example_task_2_inline() {
    let t1: Vec<Bus> = vec![Bus::Minute(17), Bus::None, Bus::Minute(13), Bus::Minute(19)];
    assert_eq!(3417, task2(&t1));

    let t2 = vec![Bus::Minute(67), Bus::Minute(7), Bus::Minute(59), Bus::Minute(61)];
    assert_eq!(754018, task2(&t2));

    let t2 = vec![Bus::Minute(67), Bus::None, Bus::Minute(7), Bus::Minute(59), Bus::Minute(61)];
    assert_eq!(779210, task2(&t2));

    let t2 = vec![Bus::Minute(67), Bus::Minute(7), Bus::None, Bus::Minute(59), Bus::Minute(61)];
    assert_eq!(1261476, task2(&t2));

    let t2 = vec![Bus::Minute(1789), Bus::Minute(37), Bus::Minute(47), Bus::Minute(1889)];
    assert_eq!(1202161486, task2(&t2));
}

#[test]
fn task22_test() {
    let n = new_mod(&vec![(0, 17), (2, 13), (3, 19)]);

    println!("Got {} {}", n.0, n.1);

    match chinese_remainder(&[0, 13 - 2, 19 - 3], &[17, 13, 19]) {
        Some(sol) => println!("Solution {}", sol),
        None => println!("No solution"),
    }

    match chinese_remainder(&[0, 37 - 1, 47 - 2, 1889 - 3], &[1789, 37, 47, 1889]) {
        Some(sol) => println!("Solution {}", sol),
        None => println!("No solution"),
    }
}

#[test]
fn example_task_2() {
    let task = read_task("example.txt");

    let result = task2(&task.busses);

    assert_eq!(1068781, result);
}


#[test]
fn example_input_1() {
    let task = read_task("input.txt");

    let result = task1(&task);

    assert_eq!(3882, result);
}

#[test]
fn example_input_2() {
    let task = read_task("input.txt");

    let result = task2(&task.busses);

    println!("Answer task 2 : {}", result);
    assert_eq!(3882, result);
}