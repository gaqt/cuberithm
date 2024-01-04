use std::{
    cmp::{self},
    collections::{BTreeSet, HashSet},
};
use strum::IntoEnumIterator;

use crate::{cube::CubeState, rotation::Rotation, solution::Solution};
pub struct Solver {
    pub middle_states: HashSet<CubeState>,
    pub found_solutions: BTreeSet<Solution>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub move_count: u8,
    pub states_processed: u64,
}

impl Solver {
    pub fn new(initial_state: &CubeState, desired_state: &CubeState, move_count: u8) -> Solver {
        Solver {
            middle_states: HashSet::new(),
            found_solutions: BTreeSet::new(),
            initial_state: initial_state.clone(),
            desired_state: desired_state.clone(),
            move_count,
            states_processed: 0,
        }
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner,
     * stops when reaching a solution or path.len() == (move_count+1)/2 (meet in the middle)
     */
    fn first_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        self.states_processed += 1;

        if state == self.desired_state && path.len() as u8 == self.move_count {
            self.found_solutions.insert(Solution { seq: path.clone() });
            return;
        }

        if state == self.desired_state {
            return;
        }

        if path.len() as u8 == cmp::max(1, (self.move_count + 1) / 2) {
            self.middle_states.insert(state.clone());
            return;
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            if !path.is_empty() && *path.last().unwrap() == rotation.reverse() {
                continue;
            }
            let mut new_state = state.clone();
            new_state.rotate(rotation);
            if prev_states.contains(&new_state) {
                continue;
            }
            path.push(rotation);
            self.first_pass_(new_state, prev_states, path);
            path.pop();
        }

        prev_states.pop();
    }

    pub fn first_pass(&mut self) {
        self.first_pass_(self.initial_state.clone(), &mut Vec::new(), &mut Vec::new());
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner
     * stops when reaching a solution (doesnt save this time) or
     * when reaching a previously reached state or path.len() > max_moves/2 (meet in the middle)
     */
    fn second_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        self.states_processed += 1;

        if state == self.initial_state {
            let mut complete_path = path.clone();
            complete_path.reverse();
            self.found_solutions.insert(Solution { seq: complete_path });
            return;
        }

        if self.middle_states.contains(&state) {
            if (self.move_count + 1) / 2 + (path.len() as u8) != self.move_count {
                return;
            }

            let mut solver_0 = Solver::new(&self.initial_state, &state, (self.move_count + 1) / 2);
            let mut solver_1 = Solver::new(&state, &self.desired_state, self.move_count / 2);

            solver_0.solve();
            solver_1.solve();

            let solutions_0 = &solver_0.found_solutions;
            let solutions_1 = &solver_1.found_solutions;

            for left in solutions_0 {
                for right in solutions_1 {
                    let mut union = left.clone();
                    union.seq.append(&mut right.seq.clone());
                    self.found_solutions.insert(union);
                }
            }

            self.states_processed += solver_0.states_processed;
            self.states_processed += solver_1.states_processed;

            return;
        }

        if path.len() as u8 > cmp::max(1, self.move_count / 2) - 1 {
            return;
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            if !path.is_empty() && *path.last().unwrap() == rotation.reverse() {
                continue;
            }
            let mut new_state = state.clone();
            new_state.rotate(rotation);
            if prev_states.contains(&new_state) {
                continue;
            };
            path.push(rotation.reverse());
            self.second_pass_(new_state, prev_states, path);
            path.pop();
        }

        prev_states.pop();
    }

    pub fn second_pass(&mut self) {
        self.second_pass_(self.desired_state.clone(), &mut Vec::new(), &mut Vec::new());
    }

    pub fn solve(&mut self) {
        self.first_pass();
        self.second_pass();
    }
}
