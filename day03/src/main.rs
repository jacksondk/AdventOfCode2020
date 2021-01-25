use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Read all lines and parse them using P::from_str to a vector
fn lines_to_vec_of_vec_of_chars(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<char>> {
    let mut entries: Vec<Vec<char>> = vec![];
    for line in lines {
        if let Ok(ip) = line {
            let chars = Vec::from_iter(ip.chars());
            entries.push(chars);
        }
    }
    entries
}

fn test_route(right: usize, down: usize, map: &Vec<Vec<char>>) -> u32 {
    let row_length = map[0].len();
    let tree = '#';
    let mut tree_count = 0;
    // Start row 0 (aka 1)
    let mut step = 0;
    loop {
        if step * down >= map.len() {
            // We are out of the woods
            break;
        }

        let column = step * right;
        let column_index = column % row_length;
        if tree == map[step * down][column_index] {
            tree_count = tree_count + 1;
        }

        step += 1;
    }
    tree_count
}

fn main() {
    let map = lines_to_vec_of_vec_of_chars(read_lines("./input.txt").unwrap());

    let tree_count = test_route(3, 1, &map);

    println!("Number of trees for first answer {}", tree_count);

    let tests: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: u64 = 1;
    for test in tests.iter() {
        let (right, down) = test;
        let sub_tree_count = test_route(*right, *down, &map);
        println!("Test {} right {} down gave {}", right, down, sub_tree_count);
        product = product * (sub_tree_count as u64);
    }
    println!("Result for task 2 : {}", product);
}
