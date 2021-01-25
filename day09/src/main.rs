use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Iterator;
use std::path::Path;

fn file_to_vec<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn find_fail(prog: &Vec<u64>, buffer_size: usize) -> Option<u64> {
    let mut buffer: Vec<u64> = vec![0; buffer_size];
    for idx in 0..buffer_size {
        buffer[idx] = prog[idx];
    }
    let mut buffer_idx: usize = 0;

    'prog: for prog_idx in buffer_size..prog.len() {
        let current = prog[prog_idx];        
        for first in 0..buffer_size - 1 {
            for second in first + 1..buffer_size {    
                if current == buffer[first] + buffer[second] {
                    buffer[buffer_idx] = current;
                    buffer_idx = (buffer_idx + 1) % buffer_size;
                    continue 'prog;
                }
            }
        }
        return Some(current);
    }
    None
}

fn task_example<P>(filename: P, buffer_size: usize)
where
    P: AsRef<Path>,
{
    let lines = file_to_vec(filename).unwrap();
    let parsed: Vec<u64> = lines
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
        if let Some(fail_line) = find_fail(&parsed, buffer_size) {
        println!("Failed on element {}", fail_line);

        'start: for first in 0..(parsed.len()-1) {
            let mut sum = 0;
            for idx in first..parsed.len() {
                sum += parsed[idx];
                if sum == fail_line {
                    let min = parsed[first..idx].iter().min().unwrap();
                    let max = parsed[first..idx].iter().max().unwrap();
                    println!("Min and max are {} and {} wit sum {}", min, max, min+max);
                    return;
                } else if sum > fail_line {
                    continue 'start;
                }
            }
        }
    }
}

fn main() {    
    task_example("example.txt", 5);
    task_example("input.txt", 25);    
}
