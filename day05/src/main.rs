use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
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

fn main() {
    let lines = read_lines("input.txt").unwrap();

    let mut cards: Vec<(u32, u32, u32)> = Vec::new();
    for line_result in lines {
        if let Ok(line) = line_result {
            let mut row = 0;
            let mut column = 0;
            for code in line.chars() {
                match code {
                    'B' => row = row * 2 + 1,
                    'F' => row = row * 2,
                    'R' => column = column * 2 + 1,
                    'L' => column = column * 2,
                    _ => panic!("What is this"),
                }
            }
            cards.push((row, column, row * 8 + column))
        }
    }

    let max_card = cards.iter().max_by_key(|c| c.2);
    println!(
        "Max {} {} {}",
        max_card.unwrap().0,
        max_card.unwrap().1,
        max_card.unwrap().2
    );

    cards.sort_by_key(|c| c.2);
    for index in 0..(cards.len() - 1) {
        if (cards[index].2 + 2) == cards[index + 1].2 {
            println!("Your ID is {}", cards[index].2 + 1);
        }
    }
}
