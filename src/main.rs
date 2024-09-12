use clap::Parser;

use crate::{cube::CubeState, solution::Solution, solver::Solver};
use std::{collections::BTreeSet, time::Instant};

pub mod cube;
pub mod face;
pub mod rotation;
pub mod solution;
pub mod solver;

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

#[cfg(test)]
mod tests {
    use crate::{cube::CubeState, rotation::Rotation};

    fn _solved_cube() -> CubeState {
        return CubeState::from_str(&String::from(
            "WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY",
        ))
        .unwrap();
    }

    /*
     * Cube in a cube in a cube
     */
    fn ccc() -> CubeState {
        return CubeState::from_str(&String::from(
            "RWGRWWRRRYOBOOBBBBWWWWGGWGRGGGRRGWRGYBOYBBYYYBYOYYOOOO",
        ))
        .unwrap();
    }

    /*
     * Applies cube in a cube in a cube, then tests the moves
     */
    #[test]
    fn check_cube_rotations() {
        let mut state: CubeState;

        let u_state = CubeState::from_str(&String::from(
            "GWRWWRRRRYOBOOBWGRWWWWGGWRGGGGRRGYYYYBOYBBBBBBYOYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::U);
        assert_eq!(u_state, state);

        let up_state = CubeState::from_str(&String::from(
            "RRRRWWRWGYOBOOBYYYWWWWGGBBBGGGRRGWGRYBOYBBWRGBYOYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Up);
        assert_eq!(up_state, state);

        let l_state = CubeState::from_str(&String::from(
            "YWGBWWORRBBBOOBYOBRWWRGGRGRGGGRRGWRGYBOYBYYYBWYOWYOWOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::L);
        assert_eq!(l_state, state);

        let lp_state = CubeState::from_str(&String::from(
            "WWGWWWWRRBOYBOOBBBBWWYGGOGRGGGRRGWRGYBRYBRYYRYYOBYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Lp);
        assert_eq!(lp_state, state);

        let f_state = CubeState::from_str(&String::from(
            "BBBRWWRRRYOOOOOBBOWGRWGGWWWGGGWRGRRGYBOYBBYYYBYOYYOGRW",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::F);
        assert_eq!(f_state, state);

        let fp_state = CubeState::from_str(&String::from(
            "WRGRWWRRRYOROOWBBGWWWGGWRGWOGGORGORGYBOYBBYYYBYOYYOBBB",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Fp);
        assert_eq!(fp_state, state);

        let r_state = CubeState::from_str(&String::from(
            "RWWRWGRRRYOBOOBBBBWWOWGOWGOGGGGRRGRWRBOWBBGYYBYYYYYOOY",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::R);
        assert_eq!(r_state, state);

        let rp_state = CubeState::from_str(&String::from(
            "RWYRWYRRYYOBOOBBBBWWGWGWWGRWRGRRGGGGOBOOBBOYYBYWYYGOOR",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Rp);
        assert_eq!(rp_state, state);

        let b_state = CubeState::from_str(&String::from(
            "RWGRWWGGGROBROBRBBWWWWGGWGRGGBRRYWROOBYBBYYYYBOYYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::B);
        assert_eq!(b_state, state);

        let bp_state = CubeState::from_str(&String::from(
            "RWGRWWYOBOOBYOBBBBWWWWGGWGRGGRRRRWRRYYYYBBYBOGGGYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Bp);
        assert_eq!(bp_state, state);

        let d_state = CubeState::from_str(&String::from(
            "RWGRWWRRRYBOOOBBBBYOBWGGWGRWWWRRGWRGGGGYBBYYYOOOYYOBYO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::D);
        assert_eq!(d_state, state);

        let dp_state = CubeState::from_str(&String::from(
            "RWGRWWRRRWWWOOBBBBGGGWGGWGRYBORRGWRGYOBYBBYYYOYBOYYOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Dp);
        assert_eq!(dp_state, state);
    }
}
