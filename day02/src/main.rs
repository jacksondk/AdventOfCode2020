use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

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
    let re = Regex::new(r"(\d*)-(\d*) (.): (.*)").unwrap();
    if let Ok(lines) = read_lines("input.txt") {
        // Mutable counters 
        let mut valid_rule_1 = 0;
        let mut valid_rule_2 = 0;
        for line in lines {
            if let Ok(pwd) = line {
                // We assume that the regular expression does not fail
                let cap = re.captures(&pwd).unwrap();

                // Extract the matches from the regular expression
                let min = cap[1].parse::<usize>().unwrap();
                let max = cap[2].parse::<usize>().unwrap();
                let letter = cap[3].chars().nth(0).unwrap();
                // Create a vector to get easy indexing
                let password = Vec::from_iter(cap[4].chars());

                // Need to clone the password as into_iter() "moves out" of vector
                let letter_count = password
                    .clone()
                    .into_iter()
                    .filter(|c| c == &letter)
                    .count();

                // Increase rule counter if password is valid according to rule 1
                if letter_count >= min && letter_count <= max {
                    valid_rule_1 = valid_rule_1 + 1;
                }

                // Increase rule counter if password is valid according to rule 2
                if (password[min - 1] == letter) != (password[max - 1] == letter) {
                    valid_rule_2 = valid_rule_2 + 1;
                }
            }
        }
        println!("Found {} valid lines", valid_rule_1);
        println!("Found {} valid lines", valid_rule_2);
    }
}
