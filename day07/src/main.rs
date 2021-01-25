use regex::Regex;
use std::collections::HashMap;
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

fn can_contain(
    color: &String,
    in_color: &String,
    config: &HashMap<String, HashMap<String, i32>>,
) -> bool {
    if in_color == color {
        true
    } else {
        match config.get(in_color) {
            Some(in_map) => {
                //println!("Recursive");
                in_map.iter().any(|c| can_contain(color, c.0, config))
            }
            _ => false,
        }
    }
}

fn total_bags(color: &String, config: &HashMap<String, HashMap<String, i32>>) -> i32 {
    match config.get(color) {
        Some(cfg) => cfg.iter().fold(0, |acc, v| acc + v.1 + v.1*total_bags(v.0,config)),
        None => 0
    }   
}

fn main() {
    let lines = file_to_vec("input.txt").unwrap();
    let re = Regex::new(
        r"(\w+ \w+) bags contain ((?P<other>no other bags)|(?P<cont>(\d+ \w+ \w+ bags?(, )?)+))\.",
    )
    .unwrap();
    let re2 = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    let mut bag_config: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in lines {
        match re.captures(&line) {
            Some(capt) => {
                let bag_color = capt.get(1);
                let mut contains: HashMap<String, i32> = HashMap::new();
                match capt.name("cont") {
                    Some(cont) => {
                        for in_bag_str in cont.as_str().split(", ") {
                            match re2.captures(&in_bag_str) {
                                Some(parsed) => {
                                    let count: i32 =
                                        parsed.get(1).unwrap().as_str().parse().unwrap();
                                    contains
                                        .insert(parsed.get(2).unwrap().as_str().to_string(), count);
                                }
                                None => println!("What"),
                            }
                        }
                    }
                    None => (),
                }

                bag_config.insert(bag_color.unwrap().as_str().to_string(), contains);
            }
            None => println!("Did not match on: {}", line),
        }
    }

    let color = String::from("shiny gold");
    let c = bag_config
        .iter()
        .filter(|bc| {
            if *bc.0 != color {
                //println!("Is {} in {}", color, bc.0);
                can_contain(&color, bc.0, &bag_config)
            } else {
                false
            }
        })
        .count();
    println!("Task 1 : {}", c);

    println!("Task 2 : {}", total_bags(&color, &bag_config));
}
