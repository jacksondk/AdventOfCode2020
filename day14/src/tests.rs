use crate::{read_input, mask_value, run_program, run_program_2};

#[test]
fn load_test() {
    let input = read_input("example.txt");

    assert_eq!(4, input.len())
}

#[test]
fn test_mask() {
    assert_eq!(73, mask_value(11, 2, 64));
    assert_eq!(101, mask_value(101, 2, 64));
    assert_eq!(64, mask_value(0, 2, 64));
}

#[test]
fn test_example_program() {
    let input = read_input("example.txt");
    let memory = run_program(&input);

    let sum = memory.iter().fold(0, |acc, v| acc+v.1);

    println!("Result task 1 on example {}", sum);
    assert_eq!(165, sum);
}

#[test]
fn test_example_input() {
    let input = read_input("input.txt");
    let memory = run_program(&input);

    let sum = memory.iter().fold(0, |acc, v| acc+v.1);

    println!("Result task 1 on example {}", sum);
    assert_eq!(13556564111697, sum);
}

#[test]
fn test_example_program_task2() {
    let input = read_input("example_2.txt");
    let memory = run_program_2(&input);

    let sum = memory.iter().fold(0, |acc, v| acc+v.1);

    println!("Result task 1 on example {}", sum);
    assert_eq!(208, sum);
}


#[test]
fn test_input_program_task2() {
    let input = read_input("input.txt");
    let memory = run_program_2(&input);

    let sum = memory.iter().fold(0, |acc, v| acc+v.1);

    println!("Result task 2 on input {}", sum);
    assert_eq!(4173715962894, sum);
}
