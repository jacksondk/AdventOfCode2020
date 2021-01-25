use crate::{read_task, count_errors, Rules, Range, error_field_sum, error_all_rules, all_possible_valid_tickets};

#[test]
fn test_load() {
    let task = read_task("input.txt");

    assert_eq!(20, task.rules.len());
    assert_eq!(20, task.your_ticket.len());
    assert!(task.other_tickets.iter().all(|t| t.len() == 20));
}

#[test]
fn test_error_sum() {
    assert_eq!(10, error_field_sum(&vec![Rules { rule_set: vec![Range { min: 0, max: 5 }] }], &vec![10]));

    assert_eq!(25, error_field_sum(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![15, 10]));


    assert_eq!(15, error_field_sum(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![15, 120]));

}

#[test]
fn test_count_errors() {
    assert_eq!(0, count_errors(&vec![Rules { rule_set: vec![Range { min: 0, max: 5 }] }], &vec![2]));

    assert_eq!(1, count_errors(&vec![Rules { rule_set: vec![Range { min: 0, max: 5 }] }], &vec![10]));
    assert_eq!(1, count_errors(&vec![Rules { rule_set: vec![Range { min: 5, max: 10 }] }], &vec![1]));

    assert_eq!(0, count_errors(&vec![Rules {
        rule_set: vec![Range { min: 0, max: 5 },
                       Range { min: 8, max: 12 }]
    }], &vec![10]));

    assert_eq!(0, count_errors(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![10, 120]));

    assert_eq!(1, count_errors(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![15, 120]));

    assert_eq!(1, count_errors(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![10, 10]));

    assert_eq!(2, count_errors(&vec![
        Rules {
            rule_set: vec![Range { min: 0, max: 5 },
                           Range { min: 8, max: 12 }]
        },Rules {
            rule_set: vec![Range { min: 110, max: 115 },
                           Range { min: 118, max: 212 }]
        }
    ], &vec![15, 10]));
}

#[test]
fn run_example_task1() {
    let task = read_task("example.txt");

    let sum = task.other_tickets.iter().fold(0, |acc, ticket| {
        let errors = error_all_rules(&task.rules, ticket);
        acc + errors
    });
    println!("Sum of error fields example {}", sum);
}

#[test]
fn run_input_task1() {
    let task = read_task("input.txt");


    let sum = task.other_tickets.iter().fold(0, |acc, ticket| {
        acc + error_all_rules(&task.rules, ticket)
    });
    println!("Sum of error fields {}", sum);
}

#[test]
fn run_all_possible_tickets() {
    let task = read_task("example.txt");

    let valid = all_possible_valid_tickets(&task.rules, task.other_tickets);

    assert_eq!(1, valid.len());

    println!("{}", valid[0][0]);
}