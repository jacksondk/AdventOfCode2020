mod tests;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

#[derive(Copy, Clone)]
struct Range {
    min: u32,
    max: u32,
}

struct Rules {
    rule_set: Vec<Range>,
}

struct Task {
    rules: Vec<Rules>,
    your_ticket: Vec<u32>,
    other_tickets: Vec<Vec<u32>>,
}

fn read_task<P>(filename: P) -> Task
    where
        P: AsRef<Path> {
    let file_in = File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);
    let mut lines = file_reader.lines().map(|l| l.unwrap());

    let mut rules: Vec<Rules> = Vec::new();
    let range_regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    loop {
        if let Some(line) = lines.next() {
            if line.len() > 0 {
                let rule_set: Vec<Range> = range_regex.captures_iter(&line).map(|c| {
                    Range { min: c[1].parse().unwrap(), max: c[2].parse().unwrap() }
                }).collect();
                rules.push(Rules { rule_set });
            } else {
                break;
            }
        }
    }
    lines.next(); // your ticket
    let your_ticket: Vec<u32> = lines.next().unwrap().split(",").map(|part| {
        part.parse().unwrap()
    }).collect();
    lines.next(); // Blank
    lines.next(); // nearby tickets

    let mut other_tickets: Vec<Vec<u32>> = Vec::new();

    loop {
        if let Some(line) = lines.next() {
            other_tickets.push(line.split(",").map(|part| {
                part.parse().unwrap()
            }).collect())
        } else {
            break;
        }
    }

    Task {
        rules,
        your_ticket,
        other_tickets,
    }
}

fn count_errors(rules: &Vec<Rules>, ticket: &Vec<u32>) -> u32 {
    rules.iter().zip(ticket.iter()).filter(
        |(rule, ticket)| {
            rule.rule_set.iter().all(|rs| {
                rs.min > **ticket || **ticket > rs.max
            })
        }).count() as u32
}

fn error_field_sum(rules: &Vec<Rules>, ticket: &Vec<u32>) -> u32 {
    rules.iter().zip(ticket.iter()).filter(
        |(rule, ticket)| {
            rule.rule_set.iter().all(|rs| {
                rs.min > **ticket || **ticket > rs.max
            })
        }).map(|(_rule, ticket)| {
        *ticket
    }).sum::<u32>() as u32
}

fn error_all_rules(rules: &Vec<Rules>, ticket: &Vec<u32>) -> u32 {
    let all_rules: Vec<Range> = rules.iter().flat_map(|r| r.rule_set.clone()).collect();
    ticket.iter().filter(|t| {
        let failtest = all_rules.iter().all(|r| {
            let test = r.min > **t || **t > r.max;
            test
        });
        if failtest {
            println!("{} does not fit any rules", **t);
        }
        failtest
    }).sum::<u32>()
}

fn possibly_valid_ticket(all_rules: &Vec<Range>, ticket: &Vec<u32>) -> bool {
    ticket.iter().any(|t| {
        all_rules.iter().all(|r| {
            r.min > *t || *t > r.max
        })
    })
}

fn all_possible_valid_tickets(rules: &Vec<Rules>, tickets: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let all_rules: Vec<Range> = rules.iter().flat_map(|r| r.rule_set.clone()).collect();
    tickets.into_iter().filter(|t| { !possibly_valid_ticket(&all_rules, t)}).collect()
}

fn main() {
    println!("Hello, world!");
}
