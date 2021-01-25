use std::fs::File;
use std::io::{self, BufRead};
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

// Read all lines and parse them using P::from_str to a vector
fn lines_to_vec<P>(lines: io::Lines<io::BufReader<File>>) -> Vec<P>
where
    P: FromStr,
{
    let mut entries: Vec<P> = vec![];
    for line in lines {
        if let Ok(ip) = line {
            match P::from_str(&ip) {
                Ok(val) => entries.push(val),
                Err(_) => println!("Unable to parse {}", ip),
            }
        }
    }
    entries
}

fn main() {
    let entries = lines_to_vec::<i32>(read_lines("./task1.txt").unwrap());
    println!("Read {} entries", entries.len());

    for a in 0..entries.len() {
        for b in a..entries.len() {
            if entries[a] + entries[b] == 2020 {
                println!("Answer task 1 {}", entries[a] * entries[b]);
            }


            for c in b..entries.len() {
                if entries[a] + entries[b] + entries[c] == 2020 {
                    println!("Answer task 2 {}", entries[a] * entries[b] * entries[c]);
                }
            }
        }
    }
}
