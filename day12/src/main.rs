use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use num_traits::FromPrimitive;
use num_derive::FromPrimitive;
use std::fmt;

pub enum Instruction {
    NORTH(i32),
    SOUTH(i32),
    EAST(i32),
    WEST(i32),
    LEFT(i32),
    RIGHT(i32),
    FORWARD(i32),
}

#[derive(FromPrimitive, Debug, PartialEq, Eq)]
pub enum Direction {
    EAST = 0,
    NORTH,
    WEST,
    SOUTH,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::EAST => "East",
            Direction::NORTH => "North",
            Direction::WEST => "West",
            Direction::SOUTH => "South",
        })
    }
}

struct ShipState {
    east_west: i32,
    north_south: i32,
    facing: Direction,
    waypoint: (i32, i32),
}

fn turn(state: Direction, degrees: i32) -> Direction {
    let steps = degrees / 90;
    let current_step = state as i32;

    FromPrimitive::from_i32(((current_step + steps) % 4 + 4) % 4).unwrap()
}

fn turn_waypoint(waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
    let steps = ((degrees / 90) % 4 + 4) % 4;
    match steps {
        0 => waypoint,
        1 => (-waypoint.1, waypoint.0),
        2 => (-waypoint.0, -waypoint.1),
        3 => (waypoint.1, -waypoint.0),
        _ => panic!("Must not happen")
    }
}

fn parse_instruction_string(s: &String) -> Instruction {
    let (i, a) = &s.split_at(1);

    let amount: i32 = a.parse().unwrap();
    match i.chars().next().unwrap() {
        'N' => Instruction::NORTH(amount),
        'S' => Instruction::SOUTH(amount),
        'E' => Instruction::EAST(amount),
        'W' => Instruction::WEST(amount),
        'L' => Instruction::LEFT(amount),
        'R' => Instruction::RIGHT(amount),
        'F' => Instruction::FORWARD(amount),
        _ => panic!("Unknown instruction {}", s),
    }
}

fn update_state(state: ShipState, instruction: Instruction) -> ShipState
{
    match instruction {
        Instruction::NORTH(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south + v, waypoint: (0, 0) },
        Instruction::SOUTH(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south - v, waypoint: (0, 0) },
        Instruction::EAST(v) => ShipState { facing: state.facing, east_west: state.east_west + v, north_south: state.north_south, waypoint: (0, 0) },
        Instruction::WEST(v) => ShipState { facing: state.facing, east_west: state.east_west - v, north_south: state.north_south, waypoint: (0, 0) },
        Instruction::LEFT(v) => ShipState { facing: turn(state.facing, v), east_west: state.east_west, north_south: state.north_south, waypoint: (0, 0) },
        Instruction::RIGHT(v) => ShipState { facing: turn(state.facing, -v), east_west: state.east_west, north_south: state.north_south, waypoint: (0, 0) },
        Instruction::FORWARD(v) => {
            match state.facing {
                Direction::NORTH => update_state(state, Instruction::NORTH(v)),
                Direction::EAST => update_state(state, Instruction::EAST(v)),
                Direction::SOUTH => update_state(state, Instruction::SOUTH(v)),
                Direction::WEST => update_state(state, Instruction::WEST(v))
            }
        }
    }
}

fn update_state_2(state: ShipState, instruction: Instruction) -> ShipState
{
    match instruction {
        Instruction::NORTH(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: (state.waypoint.0, state.waypoint.1 + v) },
        Instruction::SOUTH(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: (state.waypoint.0, state.waypoint.1 - v) },
        Instruction::EAST(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: (state.waypoint.0 + v, state.waypoint.1) },
        Instruction::WEST(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: (state.waypoint.0 - v, state.waypoint.1) },
        Instruction::LEFT(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: turn_waypoint(state.waypoint, v) },
        Instruction::RIGHT(v) => ShipState { facing: state.facing, east_west: state.east_west, north_south: state.north_south, waypoint: turn_waypoint(state.waypoint, -v) },
        Instruction::FORWARD(v) => {
            ShipState {
                facing: state.facing,
                east_west: state.east_west + v * state.waypoint.0,
                north_south: state.north_south + v * state.waypoint.1,
                waypoint: state.waypoint,
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn read_and_parse_instructions<P>(filename: P) -> Vec<Instruction>
    where
        P: AsRef<Path>
{
    let file_in = File::open(filename).unwrap();
    let file_reader = BufReader::new(file_in);
    file_reader.lines()
        .map(|l| parse_instruction_string(&l.unwrap())).collect()
}

#[cfg(test)]
mod tests;