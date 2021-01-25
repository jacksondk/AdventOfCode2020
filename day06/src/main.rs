use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn file_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
fn main() {
    let lines = file_to_vec("input.txt").unwrap();

    let mut groups: Vec<([usize; 26], usize)> = Vec::new();
    let mut group_count: usize = 0;
    let mut group: [usize; 26] = [0; 26];
    let base: usize = 'a' as usize;
    for line in lines {
        match line.as_str() {
            "" => {
                groups.push((group, group_count));
                group = [0; 26];
                group_count = 0
            }
            _ => {
                for answer in line.chars() {
                    let unicode: usize = answer as usize;
                    let index = unicode - base;
                    group[index] += 1;
                }
                group_count += 1
            }
        }
    }
    groups.push((group, group_count));

    let mut answers = 0;
    let mut answers_2 = 0;
    for g in groups.iter() {
        let count = g.0.iter().filter(|&e| *e > 0).count();
        let count_2 = g.0.iter().filter(|&e| *e == g.1).count();
        answers += count;
        answers_2 += count_2;
    }
    println!("Part one answers {}", answers);
    println!("Part two answers {}", answers_2);
}
