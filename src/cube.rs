use crate::{rotation::Rotation, side::Side};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CubeState {
    pub top: Side,
    pub left: Side,
    pub front: Side,
    pub right: Side,
    pub back: Side,
    pub bottom: Side,
}

impl CubeState {
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::U => {
                self.top.rotate_clockwise();
                for x in 0..3 {
                    let aux = self.left[x][2];
                    self.left[x][2] = self.front[x][2];
                    self.front[x][2] = self.right[x][2];
                    self.right[x][2] = self.back[x][2];
                    self.back[x][2] = aux;
                }
            }
            Rotation::Up => {
                self.top.rotate_counterclockwise();
                for x in 0..3 {
                    let aux = self.left[x][2];
                    self.left[x][2] = self.back[x][2];
                    self.back[x][2] = self.right[x][2];
                    self.right[x][2] = self.front[x][2];
                    self.front[x][2] = aux;
                }
            }
            Rotation::L => {
                self.left.rotate_clockwise();
                for i in 0..3 {
                    let aux = self.top[0][i];
                    self.top[0][i] = self.back[2][2 - i];
                    self.back[2][2 - i] = self.bottom[0][i];
                    self.bottom[0][i] = self.front[0][i];
                    self.front[0][i] = aux;
                }
            }
            Rotation::Lp => {
                self.left.rotate_counterclockwise();
                for i in 0..3 {
                    let aux = self.top[0][i];
                    self.top[0][i] = self.front[0][i];
                    self.front[0][i] = self.bottom[0][i];
                    self.bottom[0][i] = self.back[2][2 - i];
                    self.back[2][2 - i] = aux;
                }
            }
            Rotation::F => {
                self.front.rotate_clockwise();
                for i in 0..3 {
                    let aux = self.top[i][0];
                    self.top[i][0] = self.left[2][i];
                    self.left[2][i] = self.bottom[2 - i][2];
                    self.bottom[2 - i][2] = self.right[0][2 - i];
                    self.right[0][2 - i] = aux;
                }
            }
            Rotation::Fp => {
                self.front.rotate_counterclockwise();
                for i in 0..3 {
                    let aux = self.top[i][0];
                    self.top[i][0] = self.right[0][2 - i];
                    self.right[0][2 - i] = self.bottom[2 - i][2];
                    self.bottom[2 - i][2] = self.left[2][i];
                    self.left[2][i] = aux;
                }
            }
            Rotation::R => {
                self.right.rotate_clockwise();
                for i in 0..3 {
                    let aux = self.top[2][i];
                    self.top[2][i] = self.front[2][i];
                    self.front[2][i] = self.bottom[2][i];
                    self.bottom[2][i] = self.back[0][2 - i];
                    self.back[0][2 - i] = aux;
                }
            }
            Rotation::Rp => {
                self.right.rotate_counterclockwise();
                for i in 0..3 {
                    let aux = self.top[2][i];
                    self.top[2][i] = self.back[0][2 - i];
                    self.back[0][2 - i] = self.bottom[2][i];
                    self.bottom[2][i] = self.front[2][i];
                    self.front[2][i] = aux;
                }
            }
            Rotation::B => {
                self.back.rotate_clockwise();
                for i in 0..3 {
                    let aux = self.top[i][2];
                    self.top[i][2] = self.right[2][2 - i];
                    self.right[2][2 - i] = self.bottom[2 - i][0];
                    self.bottom[2 - i][0] = self.left[0][i];
                    self.left[0][i] = aux;
                }
            }
            Rotation::Bp => {
                self.back.rotate_counterclockwise();
                for i in 0..3 {
                    let aux = self.top[i][2];
                    self.top[i][2] = self.left[0][i];
                    self.left[0][i] = self.bottom[2 - i][0];
                    self.bottom[2 - i][0] = self.right[2][2 - i];
                    self.right[2][2 - i] = aux;
                }
            }
            Rotation::D => {
                self.bottom.rotate_clockwise();
                for x in 0..3 {
                    let aux = self.front[x][0];
                    self.front[x][0] = self.left[x][0];
                    self.left[x][0] = self.back[x][0];
                    self.back[x][0] = self.right[x][0];
                    self.right[x][0] = aux;
                }
            }
            Rotation::Dp => {
                self.bottom.rotate_counterclockwise();
                for x in 0..3 {
                    let aux = self.front[x][0];
                    self.front[x][0] = self.right[x][0];
                    self.right[x][0] = self.back[x][0];
                    self.back[x][0] = self.left[x][0];
                    self.left[x][0] = aux;
                }
            }
        }
    }
}
