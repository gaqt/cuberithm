use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::{cube::CubeState, rotation::Rotation, solution::Solution};

#[derive(Debug)]
pub struct Solver {
    pub middle_states: HashSet<CubeState>,
    pub found_solutions: HashSet<Solution>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub move_count: u8,
    pub first_pass_states: u64,
    pub second_pass_states: u64,
}

impl Solver {
    pub fn new(initial_state: &CubeState, desired_state: &CubeState, move_count: u8) -> Solver {
        Solver {
            middle_states: HashSet::new(),
            found_solutions: HashSet::new(),
            initial_state: initial_state.clone(),
            desired_state: desired_state.clone(),
            move_count,
            first_pass_states: 0,
            second_pass_states: 0,
        }
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner,
     * stops when reaching a solution or path.len() == move_count/2 (meet in the middle)
     */
    fn first_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        self.first_pass_states += 1;

        // println!("Processing first pass state: {:?}", &path);

        if path.len() as u8 == self.move_count / 2 {
            self.middle_states.insert(state.clone());
            return;
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            let mut new_state = state.clone();
            new_state.rotate(rotation);
            let mut contains = false;
            for prev in &mut *prev_states {
                if prev == &new_state {
                    contains = true;
                    break;
                }
            }
            if contains {
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
     * when reaching a previously reached state or path.len() > (max_moves+1)/2 (meet in the middle)
     */
    fn second_pass_(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        self.second_pass_states += 1;

        // println!("Processing second pass state: {:?}", &path);


        if (path.len() as u8) == (self.move_count + 1) / 2 && self.middle_states.contains(&state) {
            // Saving up memory by only calculating path when needed

            let mut l_solver = Solver::new(&self.initial_state, &state, self.move_count / 2);

            l_solver.solve();

            let l_solutions = l_solver.found_solutions;
            let mut right = path.clone();
            right.reverse();

            for left in l_solutions {
                let mut union = left.clone();
                union.seq.append(&mut right.clone());
                self.found_solutions.insert(union);
            }

            self.first_pass_states += l_solver.first_pass_states;
            self.second_pass_states += l_solver.second_pass_states;

            return;
        }

        if (path.len() as u8) == (self.move_count + 1) / 2 {
            return;
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            let mut new_state = state.clone();
            new_state.rotate(rotation);
            let mut contains = false;
            for prev in &mut *prev_states {
                if prev == &new_state {
                    contains = true;
                    break;
                }
            }
            if contains {
                continue;
            }
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
        // println!("Solving for {:#?}", self);

        // --- Edge cases
        if self.move_count == 0u8 {
            if self.initial_state == self.desired_state {
                self.found_solutions.insert(Solution { seq: vec![] });
            }
            return;
        }

        if self.move_count == 1u8 {
            for rotation in Rotation::iter() {
                let mut state = self.initial_state.clone();
                state.rotate(rotation);
                if state == self.desired_state {
                    self.found_solutions.insert(Solution {
                        seq: vec![rotation],
                    });
                }
            }
            return;
        }
        // ---

        self.first_pass();
        self.second_pass();

        // println!("Done: {:#?}", self)
    }
}
