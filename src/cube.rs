use crate::side::Side;

pub enum Rotation {
    U,
    Up,
    L,
    Lp,
    F,
    Fp,
    R,
    Rp,
    B,
    Bp,
    D,
    Dp,
}

pub struct CubeState {
    top: Side,
    left: Side,
    front: Side,
    right: Side,
    back: Side,
    bottom: Side,
}

impl CubeState {
    pub fn rotate(&mut self, rotation: Rotation) -> CubeState {
        match rotation {
            Rotation::U => CubeState {
                top: self.top.rotate_clockwise(),
                left: self.left.replace_top(&self.back.top()),
                front: self.front.replace_top(&self.left.top()),
                right: self.right.replace_top(&self.front.top()),
                back: self.back.replace_top(&self.right.top()),
                bottom: self.bottom,
            },
            Rotation::Up => CubeState {
                top: self.top.rotate_counterclockwise(),
                left: self.left.replace_top(&self.front.top()),
                front: self.front.replace_top(&self.right.top()),
                right: self.right.replace_top(&self.back.top()),
                back: self.back.replace_top(&self.left.top()),
                bottom: self.bottom,
            },
            Rotation::L => CubeState {
                top: self.top.replace_left(&self.back.right()),
                left: self.left.rotate_clockwise(),
                front: self.front.replace_left(&self.top.left()),
                right: self.right,
                back: self.back.replace_right(&self.bottom.left()),
                bottom: self.bottom.replace_left(&self.front.left()),
            },
            Rotation::Lp => CubeState {
                top: self.top.replace_left(&self.front.left()),
                left: self.left.rotate_counterclockwise(),
                front: self.front.replace_left(&self.bottom.left()),
                right: self.right,
                back: self.back.replace_right(&self.top.left()),
                bottom: self.bottom.replace_left(&self.back.right()),
            },
            Rotation::F => CubeState {
                top: self.top.replace_bottom(&self.left.right()),
                left: self.left.replace_right(&self.bottom.top()),
                front: self.front.rotate_clockwise(),
                right: self.right.replace_left(&self.top.bottom()),
                back: self.back,
                bottom: self.bottom.replace_top(&self.right.left()),
            },
            Rotation::Fp => CubeState {
                top: self.top.replace_bottom(&self.right.left()),
                left: self.left.replace_right(&self.top.bottom()),
                front: self.front.rotate_counterclockwise(),
                right: self.right.replace_left(&self.bottom.top()),
                back: self.back,
                bottom: self.bottom.replace_top(&self.left.right()),
            },
            Rotation::R => CubeState {
                top: self.top.replace_right(&self.front.right()),
                left: self.left,
                front: self.front.replace_right(&self.bottom.right()),
                right: self.right.rotate_clockwise(),
                back: self.back.replace_left(&self.top.right()),
                bottom: self.bottom.replace_right(&self.back.left()),
            },
            Rotation::Rp => CubeState {
                top: self.top.replace_right(&self.back.left()),
                left: self.left,
                front: self.front.replace_right(&self.top.right()),
                right: self.right.rotate_counterclockwise(),
                back: self.back.replace_left(&self.bottom.right()),
                bottom: self.bottom.replace_right(&self.front.right()),
            },
            Rotation::B => CubeState {
                top: self.top.replace_top(&self.right.right()),
                left: self.left.replace_left(&self.top.top()),
                front: self.front,
                right: self.right.replace_right(&self.bottom.bottom()),
                back: self.back.rotate_clockwise(),
                bottom: self.bottom.replace_bottom(&self.left.left()),
            },
            Rotation::Bp => CubeState {
                top: self.top.replace_top(&self.left.left()),
                left: self.left.replace_left(&self.bottom.bottom()),
                front: self.front,
                right: self.right.replace_right(&self.top.top()),
                back: self.back.rotate_counterclockwise(),
                bottom: self.bottom.replace_bottom(&self.right.right()),
            },
            Rotation::D => CubeState {
                top: self.top,
                left: self.left.replace_bottom(&self.back.bottom()),
                front: self.front.replace_bottom(&self.left.bottom()),
                right: self.right.replace_bottom(&self.front.bottom()),
                back: self.back.replace_bottom(&self.right.bottom()),
                bottom: self.bottom.rotate_clockwise(),
            },
            Rotation::Dp => CubeState {
                top: self.top,
                left: self.left.replace_bottom(&self.front.bottom()),
                front: self.front.replace_bottom(&self.right.bottom()),
                right: self.right.replace_bottom(&self.back.bottom()),
                back: self.back.replace_bottom(&self.left.bottom()),
                bottom: self.bottom.rotate_counterclockwise(),
            },
        }
    }
}
