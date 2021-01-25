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

pub enum ProgramAnalysis {
    Loop(i32),
    OutOfBounds,
    Success(i32),
    InvalidOpCode,
}

fn run_program(program: &Vec<(String, i32)>, swap: Option<usize>) -> ProgramAnalysis {
    let mut acc = 0;
    let mut program_pointer: usize = 0;
    let mut visited: Vec<bool> = vec![false; program.len()];
    loop {
        if program_pointer == program.len() {            
            return ProgramAnalysis::Success(acc);
        } else if program_pointer > program.len() {            
            return ProgramAnalysis::OutOfBounds;
        }

        if visited[program_pointer] {            
            return ProgramAnalysis::Loop(acc);
        }
        visited[program_pointer] = true;

        let (op, arg) = &program[program_pointer];        
        let swapped = match swap {
            Some(p) if p == program_pointer => true,
            _ => false,
        };
        let swapped_op = match op.as_str() {
            "nop" => 
                if swapped {
                    "jmp"
                } else {
                    "nop"
                },
            "acc" => "acc",
            "jmp" => {
                if swapped {
                    "nop"
                } else {
                    "jmp"
                }
            }
            _ => panic!("What is wrong with you"),
        };
        match swapped_op {
            "nop" => program_pointer += 1,
            "acc" => {
                acc += arg;
                program_pointer += 1;
            }
            "jmp" => {
                let new_pointer = program_pointer as i32 + arg;
                if new_pointer < 0 {                    
                    return ProgramAnalysis::OutOfBounds;
                }
                program_pointer = new_pointer as usize;
            }
            _ => panic!("Invalid op-code"),
        }
    }
}

fn main() {
    // let program : Vec<(String, i32)> = Vec::new();
    let lines = file_to_vec("input.txt").unwrap();

    let program: Vec<(String, i32)> = lines
        .iter()
        .map(|line| {
            let mut split = line.split(" ");
            let operation = String::from(split.next().unwrap());
            let value: i32 = split.next().unwrap().parse().unwrap();
            (operation, value)
        })
        .collect();

    if let ProgramAnalysis::Loop(acc) = run_program(&program, None) {
        println!("Loop detected - accumulator {}", acc)
    }
    for idx_op in 0..program.len() {
        match run_program(&program, Some(idx_op)) {
            ProgramAnalysis::Success(acc) => println!("Swaping at {} ran to finish with acc {}", idx_op, acc),
            _ => (),
        }
    }
    println!("Hello, world!");
}
