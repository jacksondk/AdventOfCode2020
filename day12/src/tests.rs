use crate::{read_and_parse_instructions, Direction, turn, update_state, ShipState, update_state_2, turn_waypoint};
use num_traits::abs;

#[test]
fn load_test() {
    let instructions = read_and_parse_instructions("example.txt");
    assert_eq!(5, instructions.len());
}

#[test]
fn turn_test() {
    assert_eq!(Direction::EAST, turn(Direction::EAST, 0));
    assert_eq!(Direction::NORTH, turn(Direction::EAST, 90));
    assert_eq!(Direction::WEST, turn(Direction::EAST, 180));
    assert_eq!(Direction::SOUTH, turn(Direction::EAST, 270));
    assert_eq!(Direction::EAST, turn(Direction::EAST, 360));
}
#[test]
fn turn_test_2() {
    assert_eq!(turn_waypoint((1,0),90), (0,1));
    assert_eq!(turn_waypoint((1,0),180), (-1,0));
    assert_eq!(turn_waypoint((1,0),270), (0,-1));
    assert_eq!(turn_waypoint((1,0),360), (1,0));
}

#[test]
fn test_example() {
    let instructions = read_and_parse_instructions("example.txt");
    let mut state = ShipState {
        north_south: 0,
        east_west: 0,
        facing: Direction::EAST,
        waypoint: (0,0)
    };
    for instruction in instructions {
        state = update_state(state, instruction);
    }

    println!("Final state {} {} {}", state.north_south, state.east_west, state.facing);
    let answer = abs(state.north_south)+abs(state.east_west);
    println!("Answer {}", answer);
    assert_eq!(25, answer);
}

#[test]
fn test_example_2() {
    let instructions = read_and_parse_instructions("example.txt");
    let mut state = ShipState {
        north_south: 0,
        east_west: 0,
        facing: Direction::EAST,
        waypoint: (10,1)
    };
    for instruction in instructions {
        state = update_state_2(state, instruction);
    }

    println!("Final state {} {} {}", state.north_south, state.east_west, state.facing);
    let answer = abs(state.north_south)+abs(state.east_west);
    println!("Answer {}", answer);
    assert_eq!(286, answer);
}

#[test]
fn test_input() {
    let instructions = read_and_parse_instructions("input.txt");
    let mut state = ShipState {
        north_south: 0,
        east_west: 0,
        facing: Direction::EAST,
        waypoint: (0,0)
    };
    for instruction in instructions {
        state = update_state(state, instruction);
    }

    println!("Final state {} {} {}", state.north_south, state.east_west, state.facing);
    let answer = abs(state.north_south)+abs(state.east_west);
    println!("Answer {}", answer);
    assert_eq!(362, answer);
}

#[test]
fn test_input_2() {
    let instructions = read_and_parse_instructions("input.txt");
    let mut state = ShipState {
        north_south: 0,
        east_west: 0,
        facing: Direction::EAST,
        waypoint: (10,1)
    };
    for instruction in instructions {
        state = update_state_2(state, instruction);
    }

    println!("Final state {} {} {}", state.north_south, state.east_west, state.facing);
    let answer = abs(state.north_south)+abs(state.east_west);
    println!("Answer {}", answer);
    assert_eq!(29895, answer);
}

