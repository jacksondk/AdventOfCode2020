use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;
use std::fmt;
use crate::Field::OccupiedChair;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Field {
    Floor,
    FreeChair,
    OccupiedChair,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Field::Floor => '.',
            Field::FreeChair => 'L',
            Field::OccupiedChair => '#',
        })
    }
}


struct Layout {
    fields: Vec<Vec<Field>>,
}

fn create_layout<P>(filename: P) -> Layout where P: AsRef<Path>, {
    let file_in = File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);
    let lines = file_reader.lines().map(|l| l.unwrap());

    let mut rows: Vec<Vec<Field>> = Vec::new();
    for line in lines {
        let mut row: Vec<Field> = Vec::new();
        row.push(Field::Floor);
        for char in line.chars() {
            let field: Field = match char {
                'L' => Field::FreeChair,
                '.' => Field::Floor,
                _ => panic!("Error reading floor plan")
            };
            row.push(field);
        }
        row.push(Field::Floor);
        rows.push(row);
    }
    let mut first_row: Vec<Field> = Vec::new();
    let mut last_row: Vec<Field> = Vec::new();
    for _ex in 0..rows[0].len() {
        first_row.push(Field::Floor);
        last_row.push(Field::Floor);
    }
    rows.insert(0, first_row);
    rows.push(last_row);

    Layout {
        fields: rows,
    }
}

const indices: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn count(pre: &Layout, row_index: usize, column_index: usize) -> usize {
    indices.iter().filter(|&idx| pre.fields[(((row_index as i32) + idx.0)) as usize][((column_index as i32) + idx.1) as usize] == OccupiedChair).count()
}

fn count_2(pre: &Layout, row_index: usize, column_index: usize) -> usize {
    let mut c: usize = 0;
    for direction in &indices {
        let mut row_idx: usize = row_index;
        let mut col_idx: usize = column_index;
        loop {
            row_idx = ((row_idx as i32) + direction.0) as usize;
            col_idx = ((col_idx as i32) + direction.1) as usize;

            if row_idx == 0 || col_idx == 0 || row_idx >= pre.fields.len() || col_idx >= pre.fields[row_idx].len() {
                break;
            }
            match pre.fields[row_idx][col_idx] {
                Field::OccupiedChair => {
                    c += 1;
                    break;
                }
                Field::FreeChair => break,
                _ => (),
            }
        }
    }
    c
}

fn count_occupied(l: &Layout) -> usize {
    l.fields.iter().fold(0, |acc, row| acc + row.iter().filter(|&f| *f == OccupiedChair).count())
}


fn layout_step(pre: &Layout, count: fn(&Layout, usize, usize) -> usize, limit: usize) -> (Layout, usize) {
    let mut post: Vec<Vec<Field>> = Vec::new();
    let mut changes: usize = 0;
    post.push(pre.fields[0].clone());
    for row_index in 1..(pre.fields.len() - 1) {
        let mut post_row: Vec<Field> = Vec::new();
        post_row.push(Field::Floor);
        for column_index in 1..(pre.fields[row_index].len() - 1) {
            let post_field = match pre.fields[row_index][column_index] {
                Field::FreeChair if count(&pre, row_index, column_index) == 0 => {
                    changes += 1;
                    Field::OccupiedChair
                }
                Field::OccupiedChair if count(&pre, row_index, column_index) >= limit => {
                    changes += 1;
                    Field::FreeChair
                }
                _ => pre.fields[row_index][column_index],
            };
            post_row.push(post_field);
        }
        post_row.push(Field::Floor);
        post.push(post_row);
    }
    post.push(pre.fields[0].clone());

    (Layout {
        fields: post,
    }, changes)
}

fn print_layout(l: &Layout) {
    for row_index in 1..(l.fields.len() - 1) {
        for column_index in 1..(l.fields[row_index].len() - 1) {
            print!("{}",
                   match l.fields[row_index][column_index] {
                       Field::Floor => '.',
                       Field::FreeChair => 'L',
                       Field::OccupiedChair => '#',
                   }
            );
        }
        println!();
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::{create_layout, layout_step, print_layout, count_occupied, count, count_2};

    #[test]
    fn load_test() {
        let layout = create_layout("example.txt");
        assert_eq!(layout.fields.len(), 12);
        assert!(layout.fields.iter().all(|row| row.len() == 12));
        print_layout(&layout);

        println!();
        let new_layout = layout_step(&layout, count, 4);
        print_layout(&new_layout.0);
        println!("There was {} changes", new_layout.1);

        println!();
        let new_layout = layout_step(&new_layout.0, count, 4);
        print_layout(&new_layout.0);
        println!("There was {} changes", new_layout.1);
    }

    #[test]
    fn stabilized_test() {
        let mut layout = create_layout("example.txt");

        loop {
            print_layout(&layout);
            let new_layout = layout_step(&layout, count, 4);

            layout = new_layout.0;

            println!("Changes {}", new_layout.1);
            if new_layout.1 == 0 {
                break;
            }
        }
        println!("Count occupied {}", count_occupied(&layout));
    }

    #[test]
    fn task1() {
        let mut layout = create_layout("input.txt");

        loop {
            let new_layout = layout_step(&layout, count, 4);

            layout = new_layout.0;

            if new_layout.1 == 0 {
                break;
            }
        }
        let result = count_occupied(&layout);
        println!("Count occupied {}", result);
        assert_eq!(2334, result)
    }

    #[test]
    fn task2_example() {
        let mut layout = create_layout("example.txt");

        loop {
            let new_layout = layout_step(&layout, count_2, 5);

            layout = new_layout.0;
            print_layout(&layout);
            if new_layout.1 == 0 {
                break;
            }
        }
        let result = count_occupied(&layout);
        println!("Count occupied {}", result);
        assert_eq!(26, result)
    }

    #[test]
    fn task2_input() {
        let mut layout = create_layout("input.txt");

        loop {
            let new_layout = layout_step(&layout, count_2, 5);

            layout = new_layout.0;
            if new_layout.1 == 0 {
                break;
            }
        }
        let result = count_occupied(&layout);
        println!("Count occupied {}", result);
        assert_eq!(2100, result)
    }
}