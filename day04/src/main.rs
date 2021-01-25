use regex::Regex;
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

fn print_passport(passport: &HashMap<String, String>) {
    println!("Passport:");
    for entry in passport.iter() {
        println!("{} : {}", entry.0, entry.1);
    }
}

fn validate_byr(passport: &HashMap<String, String>) -> bool {
    match passport.get("byr") {
        Some(val) => {
            let int_val: u32 = val.parse().unwrap();
            int_val >= 1920 && int_val <= 2002
        }
        None => false,
    }
}
fn validate_iyr(passport: &HashMap<String, String>) -> bool {
    match passport.get("iyr") {
        Some(val) => {
            let int_val: u32 = val.parse().unwrap();
            int_val >= 2010 && int_val <= 2020
        }
        None => false,
    }
}
fn validate_eyr(passport: &HashMap<String, String>) -> bool {
    match passport.get("eyr") {
        Some(val) => {
            let int_val: u32 = val.parse().unwrap();
            int_val >= 2020 && int_val <= 2030
        }
        None => false,
    }
}

fn validate_hgt(passport: &HashMap<String, String>) -> bool {
    let hgt_rgx = Regex::new(r"(?P<val>\d+)(?P<unit>(in)|(cm))").unwrap();
    match passport.get("hgt") {
        Some(val) => match hgt_rgx.captures(val) {
            Some(capt) => {
                let int_hgt: u32 = capt.name("val").unwrap().as_str().parse().unwrap();
                match capt.name("unit").unwrap().as_str() {
                    "in" => int_hgt >= 59 && int_hgt <= 76,
                    "cm" => int_hgt >= 150 && int_hgt <= 193,
                    _ => false,
                }
            }
            None => false,
        },
        None => false,
    }
}

fn validate_hcl(passport: &HashMap<String, String>) -> bool {
    let hcl_rgx = Regex::new(r"#[0-9a-f]{6}").unwrap();
    match passport.get("hcl") {
        Some(val) => match hcl_rgx.captures(val) {
            Some(capt) => true,
            None => false,
        },
        None => false,
    }
}

fn validate_ecl(passport: &HashMap<String, String>) -> bool {
    let ecl_rgx = Regex::new(r"(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)").unwrap();
    match passport.get("ecl") {
        Some(val) => match ecl_rgx.captures(val) {
            Some(capt) => true,
            None => false,
        },
        None => false,
    }
}
fn validate_pid(passport: &HashMap<String, String>) -> bool {
    let pid_rgx = Regex::new(r"^[0-9]{9}$").unwrap();
    match passport.get("pid") {
        Some(val) => match pid_rgx.captures(val) {
            Some(capt) => true,
            None => false,
        },
        None => false,
    }
}

fn main() {
    let lines = read_lines("./input.txt").unwrap();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    let mut entry: HashMap<String, String> = HashMap::new();
    for line_optional in lines {
        if let Ok(line) = line_optional {
            if line == "" {
                passports.push(entry);
                entry = HashMap::new();
            } else {
                let password_parts = line.split(" ");
                for part in password_parts {
                    let keyvalue_split: Vec<&str> = part.split(":").collect();
                    entry.insert(
                        String::from(keyvalue_split[0]),
                        String::from(keyvalue_split[1]),
                    );
                }
            }
        }
    }
    passports.push(entry);

    println!("Read {} passwords", passports.len());

    let mut valid_passport_count = 0;
    for passport in &passports {
        let has_byr = passport.contains_key("byr");
        let has_iyr = passport.contains_key("iyr");
        let has_eyr = passport.contains_key("eyr");
        let has_hgt = passport.contains_key("hgt");
        let has_hcl = passport.contains_key("hcl");
        let has_ecl = passport.contains_key("ecl");
        let has_pid = passport.contains_key("pid");
        let has_cid = passport.contains_key("cid");

        if has_byr && has_iyr && has_eyr && has_hgt && has_hcl && has_ecl && has_pid {
            valid_passport_count += 1;
        }
    }
    println!("Number of 'valid' passports {}", valid_passport_count);

    let mut valid_passport_count_2 = 0;
    for passport in &passports {
        let has_byr = validate_byr(&passport);
        let has_iyr = validate_iyr(&passport);
        let has_eyr = validate_eyr(&passport);
        let has_hgt = validate_hgt(&passport);
        let has_hcl = validate_hcl(&passport);
        let has_ecl = validate_ecl(&passport);
        let has_pid = validate_pid(&passport);
        let has_cid = passport.contains_key("cid");

        if has_byr && has_iyr && has_eyr && has_hgt && has_hcl && has_ecl && has_pid {
            valid_passport_count_2 += 1;
        } else {
            println!("Fail {} {} {} {} {} {} {}",has_byr , has_iyr , has_eyr , has_hgt , has_hcl , has_ecl , has_pid )
        }
    }
    println!("Number of 'valid' passports {}", valid_passport_count_2);
}
