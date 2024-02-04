use std::cmp::Ordering;

use crate::rotation::Rotation;

#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
pub struct Solution {
    pub seq: Vec<Rotation>,
}


impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // smaller is "lexicographically" smaller
        
        let shorter: &Vec<Rotation>;
        let longer: &Vec<Rotation>;

        if self.seq.len() <= other.seq.len() {
            shorter = &self.seq;
            longer = &other.seq;
        } else {
            shorter = &other.seq;
            longer = &self.seq;
        }

        for idx in 0..shorter.len() {
            if shorter[idx] < longer[idx] {
                return Some(Ordering::Less);
            } else if shorter[idx] > longer[idx] {
                return Some(Ordering::Greater);
            }
        }

        if shorter.len() == longer.len() {
            return Some(Ordering::Equal);
        } else {
            return Some(Ordering::Less);
        }
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
