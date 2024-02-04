use crate::{cube::CubeState, solution::Solution, solver::Solver};
use std::{collections::BTreeSet, env, time::Instant};

pub mod cube;
pub mod rotation;
pub mod solution;
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

fn get_initial_state(args: &Vec<String>) -> CubeState {
    for idx in 0..args.len() {
        if args[idx] == "initial-state" {
            return CubeState::from_str(&args[idx + 1]);
        }
    }
    panic!("Initial state unspecified");
}

fn get_desired_state(args: &Vec<String>) -> CubeState {
    for idx in 0..args.len() {
        if args[idx] == "desired-state" {
            return CubeState::from_str(&args[idx + 1]);
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

    let mut first_pass_states: u64 = 0;
    let mut second_pass_states: u64 = 0;
    let initial_time = Instant::now();

    let mut solutions: BTreeSet<Solution> = BTreeSet::new();

    for i in min_moves..(max_moves + 1) {
        let mut solver = Solver::new(&initial_state, &desired_state, i);
        solver.solve();

        for solution in &solver.found_solutions {
            solutions.insert(solution.clone());
        }

        first_pass_states = solver.first_pass_states;
        second_pass_states = solver.second_pass_states;
    }

    // for i in 0..max_moves {
    //     let mut solver = Solver::new(&initial_state, &desired_state, i);
    //     solver.solve();
    //     for solution in &solver.found_solutions {
    //         solutions_raw.push(solution.clone());
    //     }
    //     states_processed += solver.states_processed;
    // }
    //
    // solutions_raw.sort();
    //
    // let mut solutions: BTreeSet<Solution> = BTreeSet::new();
    //
    // /*
    //  * Removes dead solutions
    //  * Also removes solutions with len < min_moves
    //  */
    // for solution in &solutions_raw {
    //     if (solution.seq.len() as u8) < min_moves {
    //         continue;
    //     }
    //
    //     if !solution.is_dead(&solutions_raw) {
    //         solutions.insert(solution.clone());
    //     }
    // }

    let final_time = Instant::now();
    let elapsed_time = final_time.duration_since(initial_time);

    let mut idx: u16 = 0;
    for solution in &solutions {
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
    println!("States Processed: {}", first_pass_states + second_pass_states);
    println!("Solutions Found: {}", solutions.len());
}

#[cfg(test)]
mod tests {
    use crate::{cube::CubeState, rotation::Rotation};

    fn _solved_cube() -> CubeState {
        return CubeState::from_str(&String::from(
            "WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY",
        ));
    }

    /*
     * Cube in a cube in a cube
     */
    fn ccc() -> CubeState {
        return CubeState::from_str(&String::from(
            "RWGRWWRRRYOBOOBBBBWWWWGGWGRGGGRRGWRGYBOYBBYYYBYOYYOOOO",
        ));
    }

    /*
     * Applies cube in a cube in a cube, then tests the moves
     */
    #[test]
    fn check_cube_rotations() {
        let mut state: CubeState;

        let u_state = CubeState::from_str(&String::from(
            "GWRWWRRRRYOBOOBWGRWWWWGGWRGGGGRRGYYYYBOYBBBBBBYOYYOOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::U);
        assert_eq!(u_state, state);

        let up_state = CubeState::from_str(&String::from(
            "RRRRWWRWGYOBOOBYYYWWWWGGBBBGGGRRGWGRYBOYBBWRGBYOYYOOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Up);
        assert_eq!(up_state, state);

        let l_state = CubeState::from_str(&String::from(
            "YWGBWWORRBBBOOBYOBRWWRGGRGRGGGRRGWRGYBOYBYYYBWYOWYOWOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::L);
        assert_eq!(l_state, state);

        let lp_state = CubeState::from_str(&String::from(
            "WWGWWWWRRBOYBOOBBBBWWYGGOGRGGGRRGWRGYBRYBRYYRYYOBYOOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Lp);
        assert_eq!(lp_state, state);

        let f_state = CubeState::from_str(&String::from(
            "BBBRWWRRRYOOOOOBBOWGRWGGWWWGGGWRGRRGYBOYBBYYYBYOYYOGRW",
        ));
        state = ccc().clone();
        state.rotate(Rotation::F);
        assert_eq!(f_state, state);

        let fp_state = CubeState::from_str(&String::from(
            "WRGRWWRRRYOROOWBBGWWWGGWRGWOGGORGORGYBOYBBYYYBYOYYOBBB",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Fp);
        assert_eq!(fp_state, state);

        let r_state = CubeState::from_str(&String::from(
            "RWWRWGRRRYOBOOBBBBWWOWGOWGOGGGGRRGRWRBOWBBGYYBYYYYYOOY",
        ));
        state = ccc().clone();
        state.rotate(Rotation::R);
        assert_eq!(r_state, state);

        let rp_state = CubeState::from_str(&String::from(
            "RWYRWYRRYYOBOOBBBBWWGWGWWGRWRGRRGGGGOBOOBBOYYBYWYYGOOR",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Rp);
        assert_eq!(rp_state, state);

        let b_state = CubeState::from_str(&String::from(
            "RWGRWWGGGROBROBRBBWWWWGGWGRGGBRRYWROOBYBBYYYYBOYYYOOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::B);
        assert_eq!(b_state, state);

        let bp_state = CubeState::from_str(&String::from(
            "RWGRWWYOBOOBYOBBBBWWWWGGWGRGGRRRRWRRYYYYBBYBOGGGYYOOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Bp);
        assert_eq!(bp_state, state);

        let d_state = CubeState::from_str(&String::from(
            "RWGRWWRRRYBOOOBBBBYOBWGGWGRWWWRRGWRGGGGYBBYYYOOOYYOBYO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::D);
        assert_eq!(d_state, state);

        let dp_state = CubeState::from_str(&String::from(
            "RWGRWWRRRWWWOOBBBBGGGWGGWGRYBORRGWRGYOBYBBYYYOYBOYYOOO",
        ));
        state = ccc().clone();
        state.rotate(Rotation::Dp);
        assert_eq!(dp_state, state);
    }
}
