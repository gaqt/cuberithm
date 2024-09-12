use bon::bon;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::solution;
use crate::{cube::CubeState, rotation::Rotation, solution::Solution};

#[derive(Debug)]
pub struct SolveInstance {
    pub middle_states: HashSet<CubeState>,
    pub found_solutions: HashSet<Solution>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub move_count: u8,
    pub first_pass_states: u64,
    pub second_pass_states: u64,
}

#[bon]
impl SolveInstance {
    #[builder]
    pub fn new(initial_state: CubeState, desired_state: CubeState, move_count: u8) -> Self {
        SolveInstance {
            middle_states: HashSet::new(),
            found_solutions: HashSet::new(),
            initial_state,
            desired_state,
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

        if path.len() as u8 == self.move_count / 2 {
            self.middle_states.insert(state);
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
            self.first_pass_(new_state, prev_states, path);
            prev_states.pop();
            path.pop();
        }
    }

    fn first_pass(&mut self) {
        self.first_pass_(
            self.initial_state,
            &mut vec![self.initial_state],
            &mut Vec::new(),
        );
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

        if (path.len() as u8) == (self.move_count + 1) / 2 {
            if !self.middle_states.contains(&state) {
                return;
            }

            let mut l_solver = SolveInstance::builder()
                .initial_state(self.initial_state)
                .desired_state(state)
                .move_count(self.move_count / 2)
                .build();

            l_solver.solve();

            let l_solutions = l_solver.found_solutions;
            let right: Vec<Rotation> = path.iter().map(|it| it.reverse()).rev().collect();

            for left in l_solutions {
                let mut union = left;
                union.append(&mut right.clone());
                if !solution::has_useless_moves(self.initial_state, &union) {
                    self.found_solutions.insert(union);
                }
            }

            self.first_pass_states += l_solver.first_pass_states;
            self.second_pass_states += l_solver.second_pass_states;

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
            self.second_pass_(new_state, prev_states, path);
            path.pop();
            prev_states.pop();
        }
    }

    pub fn second_pass(&mut self) {
        self.second_pass_(
            self.desired_state,
            &mut vec![self.desired_state],
            &mut Vec::new(),
        );
    }

    pub fn solve(&mut self) {
        // --- Edge cases
        if self.move_count == 0u8 {
            if self.initial_state == self.desired_state {
                self.found_solutions.insert(Vec::new());
            }
            return;
        }

        if self.move_count == 1u8 {
            for rot in Rotation::iter() {
                let state = self.initial_state.rotate(rot);
                if state == self.desired_state {
                    self.found_solutions.insert(vec![rot]);
                }
            }
            return;
        }
        // ---

        self.first_pass();
        self.second_pass();
    }
}
