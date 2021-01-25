use crate::{next_number, find_nth_number, find_nth_number_2};

#[test]
fn example_test() {
    assert_eq!(0, next_number(&vec![0, 3, 6]));
    assert_eq!(3, next_number(&vec![0, 3, 6, 0]));
    assert_eq!(3, next_number(&vec![0, 3, 6, 0, 3]));
    assert_eq!(1, next_number(&vec![0, 3, 6, 0, 3, 3]));
    assert_eq!(0, next_number(&vec![0, 3, 6, 0, 3, 3, 1]));
    assert_eq!(4, next_number(&vec![0, 3, 6, 0, 3, 3, 1, 0]));
    assert_eq!(0, next_number(&vec![0, 3, 6, 0, 3, 3, 1, 0, 4]));
}

#[test]
fn examples_test() {
    assert_eq!(0, find_nth_number(&mut vec![0, 3, 6], 4));
    assert_eq!(3, find_nth_number(&mut vec![0, 3, 6], 5));
    assert_eq!(3, find_nth_number(&mut vec![0, 3, 6], 6));

    assert_eq!(1, find_nth_number(&mut vec![1, 3, 2], 2020));
    assert_eq!(10, find_nth_number(&mut vec![2, 1, 3], 2020));
    assert_eq!(27, find_nth_number(&mut vec![1, 2, 3], 2020));
    assert_eq!(78, find_nth_number(&mut vec![2, 3, 1], 2020));
    assert_eq!(438, find_nth_number(&mut vec![3, 2, 1], 2020));
    assert_eq!(1836, find_nth_number(&mut vec![3, 1, 2], 2020));
}

#[test]
fn input_test() {
    let task1_result = find_nth_number(&mut vec![16, 11, 15, 0, 1, 7], 2020);
    println!("Result task 1 : {}", task1_result);

    assert_eq!(662, task1_result);
}

#[test]
fn input_test_2() {
//    assert_eq!(0, find_nth_number_2(vec![0, 3, 6], 4));
    //assert_eq!(3, find_nth_number_2(vec![0, 3, 6], 5));
     assert_eq!(3, find_nth_number_2(vec![0, 3, 6], 6));


}


#[test]
fn input_task2_test() {
    let task1_result = find_nth_number_2( vec![16, 11, 15, 0, 1, 7], 30000000);
    println!("Result task 2 : {}", task1_result);

    assert_eq!(37312, task1_result);
}
