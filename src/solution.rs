use std::cmp::Ordering;

use crate::rotation::Rotation;

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

impl Solution {
    /*
     * Removing dead solutions, as in, if there is a solution smaller than X
     * that is a subsequence of X, X is a dead solution
     */
    pub fn is_dead(&self, others: &Vec<Solution>) -> bool {
        for smaller in others {
            if smaller == self {
                continue;
            }
            let mut idx: usize = 0;
            for rot in &self.seq {
                if *rot == smaller.seq[idx] {
                    idx += 1;
                    if idx == smaller.seq.len() {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
