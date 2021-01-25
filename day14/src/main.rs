mod tests;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use crate::ProgramLine::{MASK, MEM};
use std::collections::HashMap;

pub enum ProgramLine {
    MASK(u64, u64, String),
    // Zeros and ones
    MEM(u64, u64),
}

fn read_input<P>(filename: P) -> Vec<ProgramLine>
    where
        P: AsRef<Path>,
{
    let file_in = File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);
    let lines = file_reader.lines().map(|l| l.unwrap());

    let mask_regex = Regex::new("mask = (.*)").unwrap();
    let mem_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();


    let program: Vec<ProgramLine> = lines.map(|s| {
        let mask_match = mask_regex.captures(&s);
        let mem_match = mem_regex.captures(&s);
        if let Some(c) = mask_match {
            let mut zero_mask: u64 = 0;
            let mut one_mask: u64 = 0;
            for m in c[1].chars().into_iter() {
                zero_mask = zero_mask << 1;
                one_mask = one_mask << 1;

                match m {
                    '0' => zero_mask += 1,
                    '1' => one_mask += 1,

                    _ => (),
                }
            }
            MASK(zero_mask, one_mask, c[1].to_string())
        } else if let Some(c) = mem_match {
            let mem_location = c[1].parse::<u64>().unwrap();
            let value = c[2].parse::<u64>().unwrap();
            MEM(mem_location, value)
        } else {
            panic!("Parse error");
        }
    }).collect();

    program
}

fn mask_value(value: u64, zero: u64, one: u64) -> u64 {
    (value & (!zero) & (!one)) + ((!value) & (!zero) & one) + (value & (!zero) & one)
}

fn run_program(program: &Vec<ProgramLine>) -> HashMap<u64, u64> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut zero_mask: u64 = 0;
    let mut one_mask: u64 = 0;
    for instruction in program {
        match instruction {
            MASK(zero, one, _) => {
                zero_mask = *zero;
                one_mask = *one;
            }
            MEM(location, value) => {
                memory.insert(*location, mask_value(*value, zero_mask, one_mask));
                ()
            }
        }
    }

    memory
}

struct Acc {
    addresses: Vec<u64>,
    instruction_address: u64,
}

fn run_program_2(program: &Vec<ProgramLine>) -> HashMap<u64, u64> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut scrample_mask: String = "".to_string();
    for instruction in program {
        match instruction {
            MASK(_, _, mask) => {
                scrample_mask = mask.clone();
            }
            MEM(location, value) => {
                let acc = Acc { addresses: vec![0], instruction_address: *location };
                let addresses = scrample_mask.chars().rfold(acc, |acc, v| {
                    let membit = acc.instruction_address & 1;
             //        println!("  Bit is {} and mask is {}", membit, v);
                    let addr_list = match v {
                        '0' => acc.addresses.iter().map(|a| (*a << 1) + membit).collect(),
                        '1' => acc.addresses.iter().map(|a| (*a << 1) + 1).collect(),
                        'X' => {
                            let mut a0: Vec<u64> = acc.addresses.iter().map(|a| (*a << 1)).collect();
                            let mut a1: Vec<u64> = acc.addresses.iter().map(|a| (*a << 1) + 1).collect();
                            a0.append(&mut a1);
                            a0
                        },
                        _ => panic!("Must not happen")
                    };
                    Acc { addresses: addr_list, instruction_address: acc.instruction_address >> 1 }
                });

                for addr in addresses.addresses.iter() {
                    // println!("  [{}] = {}", *addr, *value);
                    memory.insert(*addr, *value);
                }
                ()
            }
        }
    }

    memory
}

fn main() {
    println!("Hello, world!");
}
