mod tests;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bus {
    None,
    Minute(u32),
}

struct Task {
    minute: u32,
    busses: Vec<Bus>,
}

fn read_task<P>(filename: P) -> Task
    where
        P: AsRef<Path> {
    let file_in = File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);
    let mut lines = file_reader.lines().map(|l| l.unwrap());
    let minutes: u32 = lines.next().unwrap().parse().unwrap();
    let mut table: Vec<Bus> = Vec::new();
    for b in lines.next().unwrap().split(',') {
        table.push(match b {
            "x" => Bus::None,
            _ => Bus::Minute(b.parse().unwrap())
        })
    }

    Task {
        minute: minutes,
        busses: table,
    }
}

fn task1(t: &Task) -> u32 {
    let mut c: Vec<(u32, u32, u32)> = t.busses.iter()
        .filter(|&b| *b != Bus::None)
        .map(|b| match b {
            Bus::Minute(v) => {
                let m = (v - (t.minute % v)) as u32;
                (t.minute, m, *v)
            }
            _ => panic!("Filter did not work!")
        }).collect();
    c.sort_by(|a, b| a.1.cmp(&b.1));

    let min = c[0];
    println!("Found {} {} {}", min.0, min.1, min.2);
    min.1 * min.2
}

fn task2_sub(guess: usize, index: usize, st: &Vec<Bus>) -> Option<usize> {
    if index == st.len() {
        Some(guess)
    } else {
        let b = &st[index];
        match &b {
            Bus::None => task2_sub(guess, index + 1, st),
            Bus::Minute(v) => {
                // println!("Testing {} mod {} = {}", guess+index, v, (guess+index)%(*v as usize));
                let fits = (guess + index) % (*v as usize) == 0;
                if fits {
                    task2_sub(guess, index + 1, st)
                } else {
                    None
                }
            }
        }
    }
    // None
}

fn task2(t: &Vec<Bus>) -> usize {
    let mut busses: Vec<(usize,&Bus)> = t.iter().enumerate().filter(|(index, bus)| **bus != Bus::None).collect();
    let remainders: Vec<i64> = busses.iter().map(|(index, bus)| {
        if let Bus::Minute(v) = bus {
            println!("{} {}", v, *index);
            let vv = *v as i64;
            (((vv - *index as i64) + vv) % vv) as i64
        } else {
            panic!("What")
        }
    }).collect();
    let moduli: Vec<i64> = busses.iter().map(|(_, bus)| {
        if let Bus::Minute(v) = bus {
            *v as i64
        } else {
            panic!("What")
        }
    }).collect();

    match chinese_remainder(remainders.as_slice(), moduli.as_slice()) {
        Some(v) => v as usize,
        None => panic!("No result")
    }
}

fn new_mod(a: &Vec<(usize, usize)>) -> (usize, usize) {
    let prod = &a.iter().fold(1, |acc, v| acc * v.1);
    let mut p = 0;
    let mut sm = 0;
    for x in a.iter() {
        p = prod / x.1;
        sm += x.0 * mod_inverse(p, x.1) * p;
    }
    (sm % prod, *prod)
}

fn mod_inverse(a: usize, m: usize) -> usize {
    let b = a % m;
    for x in 1..m {
        if (b * x) % m == 1 {
            return x;
        }
    }
    1
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn task22(t: &Vec<Bus>) -> usize {
    let mut p: (usize, usize) = (0, 0);
    for bus in t.iter().enumerate() {
        match bus {
            (index, Bus::Minute(v)) => {
                println!("Before {} {} apply {} {}", p.0, p.1, index, v);
                let pp = p.1 * (*v as usize);
                let aa = p.1 * (index - p.0) - p.0;
                println!("Yeilds {} {}", aa, pp);
            }
            _ => (),
        }
    }
    100
    //p.0 % p.1
}

fn main() {}


