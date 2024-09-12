use clap::Parser;

use cuberithm::{cube::CubeState, solution::Solution, solver::Solver};
use std::{collections::BTreeSet, time::Instant};

/// Simple algorithm generator for a 3x3x3 Rubik's Cube
///
/// Format of states passed in arguments is a 54 character long string composed of:
/// characters: Y (yellow), B (blue), G (green), R (red), W (white), O (orange)
/// arranged from left to right, bottom to top, in the order of faces:
/// white -> orange -> green -> red -> blue -> yellow
/// example: WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY (solved cube)
///          WWWWWWWWWOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBOOOYYYYYYYYY (after U move)
#[derive(Parser)]
#[command(version, about, verbatim_doc_comment)]
struct Args {
    /// Initial Cube state, right->left bottom->top green on front white on top
    #[arg(short, long)]
    initial_state: String,
    /// Desired Cube state
    #[arg(short, long)]
    desired_state: String,
    /// Min moves for algorithms to be generated
    #[arg(long)]
    min_moves: u8,
    /// Max moves for algorithsm to be generated
    #[arg(long)]
    max_moves: u8,
}

fn main() {
    let args = Args::parse();

    let initial_state = CubeState::from_str(&args.initial_state).unwrap();
    let desired_state = CubeState::from_str(&args.desired_state).unwrap();
    let min_moves = args.min_moves;
    let max_moves = args.max_moves;

    let mut first_pass_states: u64 = 0;
    let mut second_pass_states: u64 = 0;
    let initial_time = Instant::now();

    let mut solutions: BTreeSet<Solution> = BTreeSet::new();

    for i in min_moves..=max_moves {
        let mut solver = Solver::new(&initial_state, &desired_state, i);
        solver.solve();

        for solution in &solver.found_solutions {
            solutions.insert(solution.clone());
        }

        first_pass_states = solver.first_pass_states;
        second_pass_states = solver.second_pass_states;
    }

    let solutions_filtered: Vec<&Solution> = solutions
        .iter()
        .filter(|it| !it.has_useless_moves())
        .collect();

    let final_time = Instant::now();
    let elapsed_time = final_time.duration_since(initial_time);

    let mut idx: u16 = 0;
    for solution in &solutions_filtered {
        print!("Solution {}: ", idx);
        for rot in 0..solution.seq.len() {
            print!("{} ", solution.seq[rot]);
        }
        println!();
        idx += 1;
    }
    println!("\nDone.");

    println!("Elapsed Time: {:.3}s", elapsed_time.as_secs_f64());
    println!("First Pass States: {}", first_pass_states);
    println!("Second Pass States: {}", second_pass_states);
    println!(
        "States Processed: {}",
        first_pass_states + second_pass_states
    );
    println!("Solutions Found: {}", solutions_filtered.len());
}

