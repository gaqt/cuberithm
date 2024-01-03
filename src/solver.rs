use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};
use strum::IntoEnumIterator;

use crate::{cube::CubeState, rotation::Rotation};

#[derive(PartialEq, Eq, Clone)]
pub struct Solution {
    pub seq: Vec<Rotation>,
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.seq.len().cmp(&other.seq.len()))
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seq.len().cmp(&other.seq.len())
    }
}

pub struct Solver {
    pub middle_states: HashMap<CubeState, Solution>,
    pub found_solutions: BTreeSet<Solution>,
    pub initial_state: CubeState,
    pub desired_state: CubeState,
    pub move_count: u8,
}

impl Solver {
    pub fn new(initial_state: &CubeState, desired_state: &CubeState, move_count: u8) -> Solver {
        Solver {
            middle_states: HashMap::new(),
            found_solutions: BTreeSet::new(),
            initial_state: initial_state.clone(),
            desired_state: desired_state.clone(),
            move_count,
        }
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner,
     * stops when reaching a solution or path.len() == (move_count+1)/2 (meet in the middle)
     */
    fn first_pass_(
        &mut self,
        state: &mut CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        //dbg!(state);
        if *state == self.desired_state && path.len() as u8 == self.move_count {
            self.found_solutions.insert(Solution { seq: path.clone() });
            return;
        }

        if *state == self.desired_state {
            return;
        }

        if path.len() as u8 == (self.move_count + 1) / 2 {
            self.middle_states
                .insert(state.clone(), Solution { seq: path.clone() });
            return;
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            if !path.is_empty() && *path.last().unwrap() == rotation.reverse() {
                continue;
            }
            state.rotate(rotation);
            if prev_states.contains(state) {
                state.rotate(rotation.reverse());
                continue;
            }
            path.push(rotation);
            self.first_pass_(state, prev_states, path);
            state.rotate(rotation.reverse());
            path.pop();
        }

        prev_states.pop();
    }

    pub fn first_pass(&mut self) {
        self.first_pass_(
            &mut self.initial_state.clone(),
            &mut Vec::new(),
            &mut Vec::new(),
        );
    }

    /*
     * Goes through all possible "rotation paths" in a DFS manner
     * stops when reaching a solution (doesnt save this time) or
     * when reaching a previously reached state or path.len() > max_moves/2 (meet in the middle)
     */
    fn second_pass_(
        &mut self,
        state: &mut CubeState,
        prev_states: &mut Vec<CubeState>,
        path: &mut Vec<Rotation>,
    ) {
        //dbg!(state);
        if *state == self.initial_state {
            let mut complete_path = path.clone();
            complete_path.reverse();
            self.found_solutions.insert(Solution { seq: complete_path });
            return;
        }

        if path.len() as u8 > self.move_count / 2 {
            return;
        }

        let optional_path = self.middle_states.get(state);

        match optional_path {
            Some(found_path) => {
                if (found_path.seq.len() as u8 + path.len() as u8) != self.move_count {
                    return;
                }
                let mut complete_path = found_path.clone();
                for idx in (0..path.len()).rev() {
                    complete_path.seq.push(path[idx]);
                }
                self.found_solutions.insert(complete_path);
                return;
            }
            None => {}
        }

        prev_states.push(state.clone());

        for rotation in Rotation::iter() {
            if !path.is_empty() && *path.last().unwrap() == rotation.reverse() {
                continue;
            }
            state.rotate(rotation);
            if prev_states.contains(state) {
                state.rotate(rotation.reverse());
                continue;
            };
            path.push(rotation.reverse());
            self.second_pass_(state, prev_states, path);
            state.rotate(rotation.reverse());
            path.pop();
        }

        prev_states.pop();
    }

    pub fn second_pass(&mut self) {
        self.second_pass_(
            &mut self.desired_state.clone(),
            &mut Vec::new(),
            &mut Vec::new(),
        );
    }

    pub fn solve(&mut self) {
        self.first_pass();
        self.second_pass();
    }
}
