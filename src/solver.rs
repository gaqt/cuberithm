use std::collections::HashSet;
use std::thread;
use strum::IntoEnumIterator;

use crate::solution;
use crate::{cube::CubeState, rotation::Rotation, solution::Solution};

///
/// Goes through all possible "rotation paths" in a DFS manner,
/// stops when reaching a solution or path.len() == move_count/2 (meet in the middle)
///
fn first_pass(
    move_count: u8,
    middle_states: &mut HashSet<CubeState>,
    state: CubeState,
    prev_states: &mut Vec<CubeState>,
    path: &mut Vec<Rotation>,
) {
    if path.len() as u8 == move_count / 2 {
        middle_states.insert(state);
        return;
    }

    for rot in Rotation::iter() {
        let new_state = state.rotate(rot);

        if prev_states.iter().rev().any(|&x| x == new_state) {
            continue;
        }

        if solution::is_rot_useless(path, rot) {
            continue;
        }

        path.push(rot);
        prev_states.push(new_state);
        first_pass(move_count, middle_states, new_state, prev_states, path);
        prev_states.pop();
        path.pop();
    }
}

///
/// Goes through all possible "rotation paths" in a DFS manner
/// stops when reaching a solution (doesnt save this time) or when reaching
/// a previously reached state or path.len() > (max_moves+1)/2 (meet in the middle)
///
/// When it reaches a state saved in the middle states, it generates a new solver from the
/// initial state to the current state, with a halved move count. Since the complexity of
/// solving grows exponentially with the move count, recursively halving it should have a
/// negligible peformance impact, and saves a lot of memory by not having to store the path to
/// each middle state
///
fn second_pass(
    move_count: u8,
    middle_states: &HashSet<CubeState>,
    found_solutions: &mut Vec<Solution>,
    initial_state: CubeState,
    state: CubeState,
    prev_states: &mut Vec<CubeState>,
    path: &mut Vec<Rotation>,
) {
    if (path.len() as u8) == (move_count + 1) / 2 {
        if !middle_states.contains(&state) {
            return;
        }

        let l_solutions = solve(initial_state, state, move_count / 2, false);
        let right: Vec<Rotation> = path.iter().map(|it| it.reverse()).rev().collect();

        for left in l_solutions {
            let mut union = left;
            union.append(&mut right.clone());
            if !solution::has_useless_moves(initial_state, &union) {
                found_solutions.push(union);
            }
        }

        return;
    }

    for rot in Rotation::iter() {
        let new_state = state.rotate(rot);

        if prev_states.iter().rev().any(|&x| x == new_state) {
            continue;
        }

        if solution::is_rot_useless(path, rot) {
            continue;
        }

        prev_states.push(new_state);
        path.push(rot);
        second_pass(
            move_count,
            middle_states,
            found_solutions,
            initial_state,
            new_state,
            prev_states,
            path,
        );
        path.pop();
        prev_states.pop();
    }
}

pub fn solve(
    initial_state: CubeState,
    desired_state: CubeState,
    move_count: u8,
    multi_threaded: bool,
) -> Vec<Solution> {
    // --- Edge cases
    if move_count == 0u8 {
        if initial_state == desired_state {
            return vec![vec![]];
        } else {
            return vec![];
        }
    }

    if move_count == 1u8 {
        let mut found_solutions = Vec::new();
        for rot in Rotation::iter() {
            let state = initial_state.rotate(rot);
            if state == desired_state {
                found_solutions.push(vec![rot]);
            }
        }
        return found_solutions;
    }
    // ---

    if !multi_threaded {
        let mut middle_states = HashSet::new();
        let mut found_solutions = Vec::new();
        first_pass(
            move_count,
            &mut middle_states,
            initial_state,
            &mut vec![initial_state],
            &mut Vec::new(),
        );
        second_pass(
            move_count,
            &middle_states,
            &mut found_solutions,
            initial_state,
            desired_state,
            &mut vec![desired_state],
            &mut Vec::new(),
        );
        return found_solutions;
    }

    let handlers = Rotation::iter()
        .map(|rot| {
            thread::spawn(move || {
                let state = initial_state.rotate(rot);
                let mut middle_states: HashSet<CubeState> = HashSet::new();
                first_pass(
                    move_count,
                    &mut middle_states,
                    state,
                    &mut vec![initial_state, state],
                    &mut vec![rot],
                );

                middle_states
            })
        })
        .collect::<Vec<_>>();

    let mut middle_states = HashSet::new();
    handlers
        .into_iter()
        .map(|h| h.join().unwrap())
        .for_each(|generated| middle_states.extend(generated));

    struct MiddleStates(*const HashSet<CubeState>);
    unsafe impl Send for MiddleStates {}
    unsafe impl Sync for MiddleStates {}
    impl AsRef<HashSet<CubeState>> for MiddleStates {
        fn as_ref(&self) -> &HashSet<CubeState> {
            unsafe { self.0.as_ref().unwrap() }
        }
    }

    let handlers = Rotation::iter()
        .map(|rot| {
            let middle_states_ptr = MiddleStates(&middle_states);
            thread::spawn(move || {
                let state = desired_state.rotate(rot);
                let mut found_solutions: Vec<Solution> = Vec::new();
                let middle_states = middle_states_ptr.as_ref();
                second_pass(
                    move_count,
                    middle_states,
                    &mut found_solutions,
                    initial_state,
                    state,
                    &mut vec![desired_state, state],
                    &mut vec![rot],
                );

                found_solutions
            })
        })
        .collect::<Vec<_>>();

    let mut found_solutions = Vec::new();
    handlers
        .into_iter()
        .map(|h| h.join().unwrap())
        .for_each(|generated| found_solutions.extend(generated));

    found_solutions
}
