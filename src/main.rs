use crate::{
    cube::CubeState,
    side::{Color, Side},
    solver::Solver,
};
use std::env;

pub mod cube;
pub mod rotation;
pub mod side;
pub mod solver;

/*
 * args format:
 * colors for the initial state (54 total) (left->right, bottom->top, white on top, green on front)
 * colors: Y (yellow), B (blue), G (green), R (red), W (white), O (orange) or N (none/unspecified)
 * 'initial-state' then
 * first 9 chars: colors in white face
 * next 9 chars: colors in orange face
 * next 9 chars: colors in green face
 * next 9 chars: colors in red face
 * next 9 chars: colors in blue face
 * next 9 chars: colors in yellow face
 *
 * note: solved cube: WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY
 * 1 move from being solved: WWWWWWWWWOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBOOOYYYYYYYYY
 *
 * then 'desired-state' and repeat for desired state
 *
 * then 'min-moves' followed by the minimum # of moves
 * then 'max-moves' followed by the maximum # of moves
 *
 * TODO: then 'include-2' to include double moves
 * TODO: then 'include-m' to include middle moves
 * TODO: then 'include-w' to include wide moves
 * TODO: then 'fingertricks' to exclude bad fingertrick algorithms
 * TODO: then 'max-mem' followed by the maximum amount of memory used by in megabytes
 */

fn get_side(side_str: &str) -> Side {
    assert_eq!(side_str.len(), 9);
    let mut colors: [[Color; 3]; 3] = Default::default();
    let mut chars = side_str.chars();
    for y in 0..3 {
        for x in 0..3 {
            colors[x][y] = Color::from_char(chars.next().unwrap());
        }
    }
    Side { colors }
}

fn get_cube_state(cube_str: &String) -> CubeState {
    assert_eq!(cube_str.len(), 54);
    CubeState {
        top: get_side(&cube_str[0..9]),
        left: get_side(&cube_str[9..18]),
        front: get_side(&cube_str[18..27]),
        right: get_side(&cube_str[27..36]),
        back: get_side(&cube_str[36..45]),
        bottom: get_side(&cube_str[45..54]),
    }
}

fn get_initial_state(args: &Vec<String>) -> CubeState {
    for idx in 0..args.len() {
        if args[idx] == "initial-state" {
            return get_cube_state(&args[idx + 1]);
        }
    }
    panic!("Initial state unspecified");
}

fn get_desired_state(args: &Vec<String>) -> CubeState {
    for idx in 0..args.len() {
        if args[idx] == "desired-state" {
            return get_cube_state(&args[idx + 1]);
        }
    }
    panic!("Desired state unspecified");
}

fn get_min_moves(args: &Vec<String>) -> u8 {
    for idx in 0..args.len() {
        if args[idx] == "min-moves" {
            return args[idx + 1].parse::<u8>().unwrap();
        }
    }
    panic!("Minimum moves unspecified");
}

fn get_max_moves(args: &Vec<String>) -> u8 {
    for idx in 0..args.len() {
        if args[idx] == "max-moves" {
            return args[idx + 1].parse::<u8>().unwrap();
        }
    }
    panic!("Maximum moves unspecified");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let initial_state = get_initial_state(&args);
    let desired_state = get_desired_state(&args);
    let min_moves = get_min_moves(&args);
    let max_moves = get_max_moves(&args);

    let mut solver = Solver::new(&initial_state, &desired_state, min_moves, max_moves);

    solver.solve();

    let solutions = solver.collect_solutions();

    for idx in 0..solutions.len() {
        print!("Solution {}: ", idx);
        for rot in 0..solutions[idx].len() {
            print!("{} ", solutions[idx][rot]);
        }
        println!();
    }
    println!("Done.");
}
