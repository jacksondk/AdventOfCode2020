use std::path::Path;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;

fn file_to_vec<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
{
    let file_in = File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}


fn main() {
    // compute_jolts("smallexample.txt");
    // compute_jolts("example.txt");
    let (a1, a2) = compute_jolts("input.txt");
    println!("Answer task 1 : {}", a1);
    println!("Answer task 2 : {}", a2);
}

fn compute_jolts<P>(filename: P) -> (i32, u64)
    where
        P: AsRef<Path>,
{
    let mut lines: Vec<u32> = file_to_vec(filename).unwrap().iter().map(|l| l.parse::<u32>().unwrap()).collect();
    lines.insert(0, 0);
    lines.sort();

    let mut jolt_jumps = vec![0; 4];
    jolt_jumps[3] = 1; // Our device is always 3 jolts above

    for index in 0..(lines.len() - 1) {
        let jolts_diff = (&lines[index + 1] - &lines[index]) as usize;
        if jolts_diff < 4 {
            jolt_jumps[jolts_diff] += 1
        } else {
            println!("Saw jump of {}", jolts_diff);
        }
    }

    let mut count: Vec<u64> = vec![0; lines.len()];
    count[0] = 1;
    for index in 1..lines.len() {
        let mut sum_jump = 0;
        for look_behind in 1..4 {
            if index >= look_behind && lines[index] - lines[index - look_behind] <= 3 {
                sum_jump += count[index - look_behind];
            }
        }
        count[index] = sum_jump;
    }
    let answer1 = jolt_jumps[1] * jolt_jumps[3];
    let answer2 = count.last().unwrap();

    (answer1, *answer2)
}

#[cfg(test)]
mod tests {
    use crate::compute_jolts;

    #[test]
    fn test_small_example() {
        let (a1, a2) = compute_jolts("smallexample.txt");

        assert_eq!(35, a1);
        assert_eq!(8, a2);
    }

    #[test]
    fn test_example() {
        let (a1, a2) = compute_jolts("example.txt");

        assert_eq!(220, a1);
        assert_eq!(19208, a2);
    }

    #[test]
    fn test_input() {
        let (a1, a2) = compute_jolts("input.txt");

        assert_eq!(2059, a1);
        assert_eq!(86812553324672, a2);
    }
}