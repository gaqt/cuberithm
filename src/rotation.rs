use std::fmt;
use strum_macros::EnumIter;

use crate::face::Face;

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, EnumIter, Debug, PartialOrd, Ord,
)]
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

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rot = match self {
            Rotation::U => "U",
            Rotation::Up => "U'",
            Rotation::L => "L",
            Rotation::Lp => "L'",
            Rotation::F => "F",
            Rotation::Fp => "F'",
            Rotation::R => "R",
            Rotation::Rp => "R'",
            Rotation::B => "B",
            Rotation::Bp => "B'",
            Rotation::D => "D",
            Rotation::Dp => "D'",
        };
        write!(f, "{}", rot)
    }
}

impl Rotation {
    pub fn reverse(&self) -> Rotation {
        match self {
            Rotation::U => Rotation::Up,
            Rotation::Up => Rotation::U,
            Rotation::L => Rotation::Lp,
            Rotation::Lp => Rotation::L,
            Rotation::F => Rotation::Fp,
            Rotation::Fp => Rotation::F,
            Rotation::R => Rotation::Rp,
            Rotation::Rp => Rotation::R,
            Rotation::B => Rotation::Bp,
            Rotation::Bp => Rotation::B,
            Rotation::D => Rotation::Dp,
            Rotation::Dp => Rotation::D,
        }
    }

    pub fn opposite_face(&self) -> Face {
        match self {
            Rotation::U => Face::D,
            Rotation::Up => Face::D,
            Rotation::L => Face::R,
            Rotation::Lp => Face::R,
            Rotation::F => Face::B,
            Rotation::Fp => Face::B,
            Rotation::R => Face::L,
            Rotation::Rp => Face::L,
            Rotation::B => Face::F,
            Rotation::Bp => Face::F,
            Rotation::D => Face::U,
            Rotation::Dp => Face::U,
        }
    }

    pub fn face(&self) -> Face {
        match self {
            Rotation::U => Face::U,
            Rotation::Up => Face::U,
            Rotation::L => Face::L,
            Rotation::Lp => Face::L,
            Rotation::F => Face::F,
            Rotation::Fp => Face::F,
            Rotation::R => Face::R,
            Rotation::Rp => Face::R,
            Rotation::B => Face::B,
            Rotation::Bp => Face::B,
            Rotation::D => Face::D,
            Rotation::Dp => Face::D,
        }
    }
}
