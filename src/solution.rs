use crate::rotation::Rotation;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Solution {
    pub seq: Vec<Rotation>,
}

impl Solution {
    pub fn has_useless_moves(&self) -> bool {
        for i in 0..self.seq.len() {
            for j in (i + 1)..self.seq.len() {
                if self.seq[j] == self.seq[i].reverse() {
                    return true;
                }

                if self.seq[j].face() != self.seq[i].face() {
                    break;
                }
            }
        }

        false
    }
}
