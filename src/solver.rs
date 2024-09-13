use bon::bon;
use std::collections::HashSet;
use std::thread;
use strum::IntoEnumIterator;

use crate::solution;
use crate::{cube::CubeState, rotation::Rotation, solution::Solution};

#[derive(Debug, Clone)]
pub struct SolveInstance {
    pub middle_states: HashSet<CubeState>,
    pub found_solutions: HashSet<Solution>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub move_count: u8,
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
        }
    }

    ///
    /// Goes through all possible "rotation paths" in a DFS manner,
    /// stops when reaching a solution or path.len() == move_count/2 (meet in the middle)
    ///
    fn first_pass(
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
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
            self.first_pass(new_state, prev_states, path);
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
        &mut self,
        state: CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        if (path.len() as u8) == (self.move_count + 1) / 2 {
            if !self.middle_states.contains(&state) {
                return;
            }

            let mut l_solver = SolveInstance::builder()
                .initial_state(self.initial_state)
                .desired_state(state)
                .move_count(self.move_count / 2)
                .build();

            l_solver.solve(false);

            let right: Vec<Rotation> = path.iter().map(|it| it.reverse()).rev().collect();
            let l_solutions = l_solver.found_solutions;

            for left in l_solutions {
                let mut union = left;
                union.append(&mut right.clone());
                if !solution::has_useless_moves(self.initial_state, &union) {
                    self.found_solutions.insert(union.clone());
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
            self.second_pass(new_state, prev_states, path);
            path.pop();
            prev_states.pop();
        }
    }

    pub fn solve(&mut self, multi_threaded: bool) {
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

        if !multi_threaded {
            self.first_pass(
                self.initial_state,
                &mut vec![self.initial_state],
                &mut Vec::new(),
            );
            self.second_pass(
                self.desired_state,
                &mut vec![self.desired_state],
                &mut Vec::new(),
            );
            return;
        }

        let handlers = Rotation::iter()
            .map(|rot| {
                let mut cloned = self.clone();
                thread::spawn(move || {
                    let state = cloned.initial_state.rotate(rot);
                    cloned.first_pass(
                        cloned.initial_state,
                        &mut vec![cloned.initial_state, state],
                        &mut vec![rot],
                    );

                    cloned
                })
            })
            .collect::<Vec<_>>();

        handlers
            .into_iter()
            .map(|h| h.join().unwrap())
            .for_each(|solver| self.middle_states.extend(solver.middle_states));

        let handlers = Rotation::iter()
            .map(|rot| {
                let mut cloned = self.clone();
                thread::spawn(move || {
                    let state = cloned.desired_state.rotate(rot);
                    cloned.second_pass(
                        cloned.desired_state,
                        &mut vec![cloned.desired_state, state],
                        &mut vec![rot],
                    );

                    cloned
                })
            })
            .collect::<Vec<_>>();

        handlers
            .into_iter()
            .map(|h| h.join().unwrap())
            .for_each(|solver| self.middle_states.extend(solver.middle_states));
    }
}
