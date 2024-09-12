/*
Representing the cube using just 20 bytes with u32 array and 24 bytes with u64 array
Very fast cube state comparisons and fast traversal

lets define a cell as a "color" on the cube

cell = [bit; 3]

unspecified = 000
white = 001
orange = 010
green = 011
red = 100
blue = 101
yellow = 110

state = [cell; 50] (8 on each side ignoring the immobile centers + 2 extra)


colors of the cube:

  W
O G R B
  Y

cell indexes in state number:

00: extra
49: extra

          01 05 04
          06    08
          02 07 03

09 13 12  17 21 20  25 29 28  33 37 36
14    16  22    24  30    32  38    40
10 15 11  18 23 19  26 31 27  34 39 35

          41 45 44
          46    48
          42 47 43



now lets say we want to perform an U move

first, lets take the cells from the white side using a mask

white_mask = (2^0 + 2^1 + 2^2) * (2^(3*0) + 2^(3*1) + 2^(3*2) + 2^(3*3) + 2^(3*4) + 2^(3*5) + 2^(3*6) ^ 2^(3*7))
              the 3 cell bits       the indexes of the cells (taking into account cell size, which is 3 bits)

overflow_mask = (2^0 + 2^1 + 2^2) * (2^(3*0) + 2^(3*4))

white_cells = state & white_mask
new_white_cells = white_cells >> 3
overflow_cells = new_white_cells & overflow_mask
new_overflow_cells = overflow_cells << 12
new_white_cells ^= overflow_cells
new_white_cells ^= new_overflow_cells


now, we take the surrounding cells using another mask

side_mask = (2^0 + 2^1 + 2^2) * (2^(3*8) + 2^(3*12) + 2^(3*11) + ... + 2^(3*36) + 2^(3*35))

overflow_mask = (2^0 + 2^1 + 2^2) * (2^(3*1) + 2^(3*5) + 2^(3*4))

side_cells = state & side_mask
new_side_cells = side_cells >> 24
overflow_cells = new_side_cells & overflow_mask
new_overflow_cells = overflow_cells << 96
new_side_cells ^= overflow_cells
new_side_cells ^= new_overflow_cells

then:

state ^= white_cells
state ^= new_white_cells

state ^= side_cells
state ^= new_side_cells


since both masks are known at compile time, all these operations are very fast

horizontal moves (U U' D D') require the least computations,
whereas side moves (R R' L L') require a bit more and
front moves (F F' B B') require the most.
*/

use std::str::FromStr;
use std::{fmt::Display, ops::Shl};

use bnum::BUint;
use thiserror::Error;

use crate::rotation::Rotation;

const CELL_MASK: BUint<3> = BUint::<3>::parse_str_radix("111", 2);

// -----------------------

// 2**(3*1) + 2**(3*2) + 2**(3*3) + 2**(3*4) + 2**(3*5) + 2**(3*6) + 2**(3*7) + 2**(3*8)
const UP_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1249248", 16));

// 2**(3*5) + 2**(3*9)
const UP_OVERFLOW_MASK_REV: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8008000", 16));

// 2**(3*0) + 2**(3*4)
const UP_OVERFLOW_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001", 16));

// 2**(3*9) + 2**(3*13) + 2**(3*12) + 2**(3*17) + 2**(3*21) + 2**(3*20) + 2**(3*25) + 2**(3*29) + 2**(3*28) + 2**(3*33) + 2**(3*37) + 2**(3*36)
const UP_SIDE_MASK: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "9008009008009008009008000000",
    16,
));

// 2**(3*1) + 2**(3*5) + 2**(3*4)
const UP_SIDE_OVERFLOW_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("9008", 16));

// 2**(3*41) + 2**(3*45) + 2**(2*44)
const UP_SIDE_OVERFLOW_MASK_REV: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("9008000000000000000000000000000000", 16),
);

// -----------------------
// -----------------------
// -----------------------

// 2**(3*9) + 2**(3*10) + 2**(3*11) + 2**(3*12) + 2**(3*13) + 2**(3*14) + 2**(3*15) + 2**(3*16)
const LEFT_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1249248000000", 16));

// 2**(3*13) + 2**(3*17)
const LEFT_OVERFLOW_MASK_REV: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8008000000000", 16));

// 2**(3*8) + 2**(3*12)
const LEFT_OVERFLOW_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001000000", 16));

// 2**(3*1) + 2**(3*6) + 2**(3*2)
const LEFT_SIDE_MASK_0: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40048", 16));

// 2**(3*17) + 2**(3*22) + 2**(3*18)
const LEFT_SIDE_MASK_1: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40048000000000000", 16));

// 2**(3*41) + 2**(3*46) + 2**(3*42)
const LEFT_SIDE_MASK_2: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "40048000000000000000000000000000000",
    16,
));

// 2**(3*35) + 2**(3*40) + 2**(3*36)
const LEFT_SIDE_MASK_3: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "1001200000000000000000000000000",
    16,
));

const LEFT_SIDE_MASK_ALL: BUint<3> = LEFT_SIDE_MASK_0
    .bitor(LEFT_SIDE_MASK_1)
    .bitor(LEFT_SIDE_MASK_2)
    .bitor(LEFT_SIDE_MASK_3);

// -----------------------
// -----------------------
// -----------------------

// 2**(3*17) + 2**(3*18) + 2**(3*19) + 2**(3*20) + 2**(3*21) + 2**(3*22) + 2**(3*23) + 2**(3*24)
const FRONT_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1249248000000000000", 16));

// 2**(3*21) + 2**(3*25)
const FRONT_OVERFLOW_MASK_REV: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8008000000000000000", 16));

// 2**(3*16) + 2**(3*20)
const FRONT_OVERFLOW_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001000000000000", 16));

// 2**(3*2) + 2**(3*7) + 2**(3*3)
const FRONT_SIDE_MASK_0: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("200240", 16));

// 2**(3*25) + 2**(3*30) + 2**(3*26)
const FRONT_SIDE_MASK_1: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40048000000000000000000", 16));

// 2**(3*30) + 2**(3*26)
const FRONT_SIDE_MASK_1A: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40040000000000000000000", 16));

// 2**(3*25)
const FRONT_SIDE_MASK_1B: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8000000000000000000", 16));

// 2**(3*45) + 2**(3*41)
const FRONT_SIDE_MASK_2A: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("8008000000000000000000000000000000", 16),
);

// 2**(3*44)
const FRONT_SIDE_MASK_2B: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("1000000000000000000000000000000000", 16),
);

// 2**(3*11) + 2**(3*16) + 2**(3*12)
const FRONT_SIDE_MASK_3: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001200000000", 16));

// 2**(3*12) + 2**(3*16)
const FRONT_SIDE_MASK_3A: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001000000000", 16));

// 2**(3**11)
const FRONT_SIDE_MASK_3B: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("200000000", 16));

const FRONT_SIDE_MASK_ALL: BUint<3> = (FRONT_SIDE_MASK_0)
    .bitor(FRONT_SIDE_MASK_1)
    .bitor(FRONT_SIDE_MASK_2A)
    .bitor(FRONT_SIDE_MASK_2B)
    .bitor(FRONT_SIDE_MASK_3);

// -----------------------
// -----------------------
// -----------------------

// 2**(3*25) + 2**(3*26) + 2**(3*27) + 2**(3*28) + 2**(3*29) + 2**(3*30) + 2**(3*31) + 2**(3*32)
const RIGHT_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1249248000000000000000000", 16));

// 2**(3*33) + 2**(3*29)
const RIGHT_OVERFLOW_MASK_REV: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8008000000000000000000000", 16));

// 2**(3*24) + 2**(3*28)
const RIGHT_OVERFLOW_MASK: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001000000000000000000", 16));

// 2**(3*4) + 2**(3*8) + 2**(3*3)
const RIGHT_SIDE_MASK_0: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001200", 16));

// 2**(3*33) + 2**(3*38) + 2**(3*34)
const RIGHT_SIDE_MASK_1: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "40048000000000000000000000000",
    16,
));

// 2**(3*44) + 2**(3*48) + 2**(3*43)
const RIGHT_SIDE_MASK_2: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "1001200000000000000000000000000000000",
    16,
));

// 2**(3*20) + 2**(3*24) + 2**(3*19)
const RIGHT_SIDE_MASK_3: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001200000000000000", 16));

const RIGHT_SIDE_MASK_ALL: BUint<3> = (RIGHT_SIDE_MASK_0)
    .bitor(RIGHT_SIDE_MASK_1)
    .bitor(RIGHT_SIDE_MASK_2)
    .bitor(RIGHT_SIDE_MASK_3);

// -----------------------
// -----------------------
// -----------------------

// 2**(3*33) + 2**(3*34) + 2**(3*35) + 2**(3*36) + 2**(3*37) + 2**(3*38) + 2**(3*39) + 2**(3*40)
const BACK_MASK: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "1249248000000000000000000000000",
    16,
));

// 2**(3*37) + 2**(3*41)
const BACK_OVERFLOW_MASK_REV: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("8008000000000000000000000000000", 16),
);

// 2**(3*32) + 2**(3*36)
const BACK_OVERFLOW_MASK: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("1001000000000000000000000000", 16),
);

// 2**(3*5) + 2**(3*1)
const BACK_SIDE_MASK_0A: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8008", 16));

// 2**(3*4)
const BACK_SIDE_MASK_0B: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1000", 16));

// 2**(3*9) + 2**(3*14) + 2**(3*10)
const BACK_SIDE_MASK_1: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40048000000", 16));

// 2**(3*10) + 2**(3*14)
const BACK_SIDE_MASK_1A: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("40040000000", 16));

// 2**(3*9)
const BACK_SIDE_MASK_1B: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("8000000", 16));

// 2**(3*42) + 2**(3*47) + 2**(3*43)
const BACK_SIDE_MASK_2: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "200240000000000000000000000000000000",
    16,
));
// 2**(3*28) + 2**(3*32) + 2**(3*27)
const BACK_SIDE_MASK_3: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001200000000000000000000", 16));

// 2**(3*28) + 2**(3*32)
const BACK_SIDE_MASK_3A: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("1001000000000000000000000", 16));

// 2**(3*27)
const BACK_SIDE_MASK_3B: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("200000000000000000000", 16));

const BACK_SIDE_MASK_ALL: BUint<3> = (BACK_SIDE_MASK_0A)
    .bitor(BACK_SIDE_MASK_0B)
    .bitor(BACK_SIDE_MASK_1)
    .bitor(BACK_SIDE_MASK_2)
    .bitor(BACK_SIDE_MASK_3);

// -----------------------
// -----------------------
// -----------------------

// 2**(3*41) + 2**(3*42) + 2**(3*43) + 2**(3*44) + 2**(3*45) + 2**(3*46) + 2**(3*47) + 2**(3*48)
const DOWN_MASK: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "1249248000000000000000000000000000000",
    16,
));

// 2**(3*49) + 2**(3*45)
const DOWN_OVERFLOW_MASK_REV: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("8008000000000000000000000000000000000", 16),
);

// 2**(3*40) + 2**(3*44)
const DOWN_OVERFLOW_MASK: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("1001000000000000000000000000000000", 16),
);

// 2**(3*10) + 2**(3*15) + 2**(3*11) + 2**(3*18) + 2**(3*23) + 2**(3*19) + 2**(3*26) + 2**(3*31) + 2**(3*27) + 2**(3*34) + 2**(3*39) + 2**(3*35)
const DOWN_SIDE_MASK: BUint<3> = CELL_MASK.mul(BUint::<3>::parse_str_radix(
    "200240200240200240200240000000",
    16,
));

// 2**(3*42) + 2**(3*47) + 2**(3*43)
const DOWN_SIDE_OVERFLOW_MASK: BUint<3> = CELL_MASK.mul(
    BUint::<3>::parse_str_radix("200240000000000000000000000000000000", 16),
);

// 2**(3*2) + 2**(3*7) + 2**(3*3)
const DOWN_SIDE_OVERFLOW_MASK_REV: BUint<3> =
    CELL_MASK.mul(BUint::<3>::parse_str_radix("200240", 16));

const DISPLAYIDX_TO_CELLIDX: [usize; 54] = [
    2, 7, 3, 6, 0, 8, 1, 5, 4, 10, 15, 11, 14, 0, 16, 9, 13, 12, 18, 23, 19,
    22, 0, 24, 17, 21, 20, 26, 31, 27, 30, 0, 32, 25, 29, 28, 34, 39, 35, 38,
    0, 40, 33, 37, 36, 42, 47, 43, 46, 0, 48, 41, 45, 44,
];

const ONE: BUint<3> = BUint::ONE;

// const _CELLIDX_TO_DISPLAYIDX: [usize; 50] = {
//     let mut result: [usize; 50] = [0; 50];
//
//     for cellidx in 1..49 {
//         for crr in 0..DISPLAYIDX_TO_CELLIDX.len() {
//             if cellidx == DISPLAYIDX_TO_CELLIDX[crr] {
//                 result[cellidx] = crr;
//                 break;
//             }
//         }
//     }
//
//     result
// };

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CubeState {
    state: BUint<3>,
}

impl Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_unwrapped_cube_str())
    }
}

#[derive(Debug, Error)]
pub enum FromStrErr {
    #[error("Invalid string length")]
    InvalidLen,
    #[error("Invalid character: {0}")]
    InvalidChar(char),
}

impl FromStr for CubeState {
    type Err = FromStrErr;

    fn from_str(cube_str: &str) -> Result<CubeState, Self::Err> {
        if cube_str.len() != DISPLAYIDX_TO_CELLIDX.len() {
            return Err(FromStrErr::InvalidLen);
        }

        DISPLAYIDX_TO_CELLIDX
            .iter()
            .zip(cube_str.chars())
            .try_fold(BUint::<3>::ZERO, |state, (&cellidx, color)| {
                if cellidx == 0 {
                    return Ok(state);
                }

                let cell: u8 = match color {
                    'N' => 0b000,
                    'W' => 0b001,
                    'O' => 0b010,
                    'G' => 0b011,
                    'R' => 0b100,
                    'B' => 0b101,
                    'Y' => 0b110,
                    _ => return Err(FromStrErr::InvalidChar(color)),
                };

                Ok(state | (&ONE).shl(3 * cellidx).mul(cell.into()))
            })
            .map(|state| CubeState { state })
    }
}

impl CubeState {
    /// # Panics
    /// Will panic if idx is out of bounds
    fn cell(&self, idx: u8) -> u8 {
        (self.state).shr(3 * idx as u32).bitand(CELL_MASK).digits()[0] as u8
    }

    /// # Panics
    /// Will panic if idx is out of bounds or if the cell has invalid bits
    fn cell_char(&self, idx: u8) -> char {
        match self.cell(idx) {
            0b000 => 'N',
            0b001 => 'W',
            0b010 => 'O',
            0b011 => 'G',
            0b100 => 'R',
            0b101 => 'B',
            0b110 => 'Y',
            _ => panic!("Invalid state"),
        }
    }

    /// # Panics
    /// Will panic if start or slice are out of bounds
    fn get_face_slice_str(&self, start: u8, slice: u8) -> String {
        match slice {
            0 => format!(
                "{} {} {}",
                self.cell_char(start),
                self.cell_char(start + 4),
                self.cell_char(start + 3)
            ),
            1 => format!(
                "{}   {}",
                self.cell_char(start),
                self.cell_char(start + 2)
            ),
            2 => format!(
                "{} {} {}",
                self.cell_char(start),
                self.cell_char(start + 5),
                self.cell_char(start + 1)
            ),
            _ => panic!("Invalid slice index"),
        }
    }

    pub fn get_unwrapped_cube_str(&self) -> String {
        let padding = "      ";
        let mut result = String::new();
        result += padding;
        result.push_str(&self.get_face_slice_str(1, 0));
        result += "\n";

        result += padding;
        result.push_str(&self.get_face_slice_str(6, 1));
        result += "\n";

        result += padding;
        result.push_str(&self.get_face_slice_str(2, 2));
        result += "\n";

        result.push_str(&self.get_face_slice_str(9, 0));
        result += " ";
        result.push_str(&self.get_face_slice_str(17, 0));
        result += " ";
        result.push_str(&self.get_face_slice_str(25, 0));
        result += " ";
        result.push_str(&self.get_face_slice_str(33, 0));
        result += "\n";

        result.push_str(&self.get_face_slice_str(14, 1));
        result += " ";
        result.push_str(&self.get_face_slice_str(22, 1));
        result += " ";
        result.push_str(&self.get_face_slice_str(30, 1));
        result += " ";
        result.push_str(&self.get_face_slice_str(38, 1));
        result += "\n";

        result.push_str(&self.get_face_slice_str(10, 2));
        result += " ";
        result.push_str(&self.get_face_slice_str(18, 2));
        result += " ";
        result.push_str(&self.get_face_slice_str(26, 2));
        result += " ";
        result.push_str(&self.get_face_slice_str(34, 2));
        result += "\n";

        result += padding;
        result.push_str(&self.get_face_slice_str(41, 0));
        result += "\n";

        result += padding;
        result.push_str(&self.get_face_slice_str(46, 1));
        result += "\n";

        result += padding;
        result.push_str(&self.get_face_slice_str(42, 2));
        result += "\n";

        result
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::U => {
                let mut up_cells = self.state & UP_MASK;
                self.state ^= up_cells;
                up_cells >>= 3;
                let mut overflow_cells = up_cells & UP_OVERFLOW_MASK;
                up_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                up_cells ^= overflow_cells;
                self.state ^= up_cells;

                let mut side_cells = self.state & UP_SIDE_MASK;
                self.state ^= side_cells;
                side_cells >>= 3 * 8;
                let mut side_overflow_cells =
                    side_cells & UP_SIDE_OVERFLOW_MASK;
                side_cells ^= side_overflow_cells;
                side_overflow_cells <<= 3 * 32;
                side_cells ^= side_overflow_cells;
                self.state ^= side_cells;
            }
            Rotation::Up => {
                let mut up_cells = self.state & UP_MASK;
                self.state ^= up_cells;
                up_cells <<= 3;
                let mut overflow_cells = up_cells & UP_OVERFLOW_MASK_REV;
                up_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                up_cells ^= overflow_cells;
                self.state ^= up_cells;

                let mut side_cells = self.state & UP_SIDE_MASK;
                self.state ^= side_cells;
                side_cells <<= 3 * 8;
                let mut side_overflow_cells =
                    side_cells & UP_SIDE_OVERFLOW_MASK_REV;
                side_cells ^= side_overflow_cells;
                side_overflow_cells >>= 3 * 32;
                side_cells ^= side_overflow_cells;
                self.state ^= side_cells;
            }
            Rotation::L => {
                let mut left_cells = self.state & LEFT_MASK;
                self.state ^= left_cells;
                left_cells >>= 3;
                let mut overflow_cells = left_cells & LEFT_OVERFLOW_MASK;
                left_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                left_cells ^= overflow_cells;
                self.state ^= left_cells;

                let mut side0_cells = self.state & LEFT_SIDE_MASK_0;
                let mut side1_cells = self.state & LEFT_SIDE_MASK_1;
                let mut side2_cells = self.state & LEFT_SIDE_MASK_2;
                let mut side3_cells = self.state & LEFT_SIDE_MASK_3;
                side0_cells <<= 3 * 16;
                side1_cells <<= 3 * 24;
                side2_cells >>= 3 * 6;
                side3_cells >>= 3 * 34;
                self.state ^= self.state & LEFT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1_cells;
                self.state ^= side2_cells;
                self.state ^= side3_cells;
            }
            Rotation::Lp => {
                let mut left_cells = self.state & LEFT_MASK;
                self.state ^= left_cells;
                left_cells <<= 3;
                let mut overflow_cells = left_cells & LEFT_OVERFLOW_MASK_REV;
                left_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                left_cells ^= overflow_cells;
                self.state ^= left_cells;

                let mut side0_cells = self.state & LEFT_SIDE_MASK_0;
                let mut side1_cells = self.state & LEFT_SIDE_MASK_1;
                let mut side2_cells = self.state & LEFT_SIDE_MASK_2;
                let mut side3_cells = self.state & LEFT_SIDE_MASK_3;
                side0_cells <<= 3 * 34;
                side1_cells >>= 3 * 16;
                side2_cells >>= 3 * 24;
                side3_cells <<= 3 * 6;
                self.state ^= self.state & LEFT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1_cells;
                self.state ^= side2_cells;
                self.state ^= side3_cells;
            }
            Rotation::F => {
                let mut front_cells = self.state & FRONT_MASK;
                self.state ^= front_cells;
                front_cells >>= 3;
                let mut overflow_cells = front_cells & FRONT_OVERFLOW_MASK;
                front_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                front_cells ^= overflow_cells;
                self.state ^= front_cells;

                let mut side0_cells = self.state & FRONT_SIDE_MASK_0;
                let mut side1a_cells = self.state & FRONT_SIDE_MASK_1A;
                let mut side1b_cells = self.state & FRONT_SIDE_MASK_1B;
                let mut side2a_cells = self.state & FRONT_SIDE_MASK_2A;
                let mut side2b_cells = self.state & FRONT_SIDE_MASK_2B;
                let mut side3_cells = self.state & FRONT_SIDE_MASK_3;
                side0_cells <<= 3 * 23;
                side1a_cells <<= 3 * 15;
                side1b_cells <<= 3 * 19;
                side2a_cells >>= 3 * 29;
                side2b_cells >>= 3 * 33;
                side3_cells >>= 3 * 9;
                self.state ^= self.state & FRONT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1a_cells;
                self.state ^= side1b_cells;
                self.state ^= side2a_cells;
                self.state ^= side2b_cells;
                self.state ^= side3_cells;
            }
            Rotation::Fp => {
                let mut front_cells = self.state & FRONT_MASK;
                self.state ^= front_cells;
                front_cells <<= 3;
                let mut overflow_cells = front_cells & FRONT_OVERFLOW_MASK_REV;
                front_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                front_cells ^= overflow_cells;
                self.state ^= front_cells;

                let mut side0_cells = self.state & FRONT_SIDE_MASK_0;
                let mut side1_cells = self.state & FRONT_SIDE_MASK_1;
                let mut side2a_cells = self.state & FRONT_SIDE_MASK_2A;
                let mut side2b_cells = self.state & FRONT_SIDE_MASK_2B;
                let mut side3a_cells = self.state & FRONT_SIDE_MASK_3A;
                let mut side3b_cells = self.state & FRONT_SIDE_MASK_3B;
                side0_cells <<= 3 * 9;
                side1_cells >>= 3 * 23;
                side2a_cells >>= 3 * 15;
                side2b_cells >>= 3 * 19;
                side3a_cells <<= 3 * 29;
                side3b_cells <<= 3 * 33;
                self.state ^= self.state & FRONT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1_cells;
                self.state ^= side2a_cells;
                self.state ^= side2b_cells;
                self.state ^= side3a_cells;
                self.state ^= side3b_cells;
            }
            Rotation::R => {
                let mut right_cells = self.state & RIGHT_MASK;
                self.state ^= right_cells;
                right_cells >>= 3;
                let mut overflow_cells = right_cells & RIGHT_OVERFLOW_MASK;
                right_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                right_cells ^= overflow_cells;
                self.state ^= right_cells;

                let mut side0_cells = self.state & RIGHT_SIDE_MASK_0;
                let mut side1_cells = self.state & RIGHT_SIDE_MASK_1;
                let mut side2_cells = self.state & RIGHT_SIDE_MASK_2;
                let mut side3_cells = self.state & RIGHT_SIDE_MASK_3;
                side0_cells <<= 3 * 30;
                side1_cells <<= 3 * 10;
                side2_cells >>= 3 * 24;
                side3_cells >>= 3 * 16;
                self.state ^= self.state & RIGHT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1_cells;
                self.state ^= side2_cells;
                self.state ^= side3_cells;
            }
            Rotation::Rp => {
                let mut right_cells = self.state & RIGHT_MASK;
                self.state ^= right_cells;
                right_cells <<= 3;
                let mut overflow_cells = right_cells & RIGHT_OVERFLOW_MASK_REV;
                right_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                right_cells ^= overflow_cells;
                self.state ^= right_cells;

                let mut side0_cells = self.state & RIGHT_SIDE_MASK_0;
                let mut side1_cells = self.state & RIGHT_SIDE_MASK_1;
                let mut side2_cells = self.state & RIGHT_SIDE_MASK_2;
                let mut side3_cells = self.state & RIGHT_SIDE_MASK_3;
                side0_cells <<= 3 * 16;
                side1_cells >>= 3 * 30;
                side2_cells >>= 3 * 10;
                side3_cells <<= 3 * 24;
                self.state ^= self.state & RIGHT_SIDE_MASK_ALL;
                self.state ^= side0_cells;
                self.state ^= side1_cells;
                self.state ^= side2_cells;
                self.state ^= side3_cells;
            }
            Rotation::B => {
                let mut back_cells = self.state & BACK_MASK;
                self.state ^= back_cells;
                back_cells >>= 3;
                let mut overflow_cells = back_cells & BACK_OVERFLOW_MASK;
                back_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                back_cells ^= overflow_cells;
                self.state ^= back_cells;

                let mut side0a_cells = self.state & BACK_SIDE_MASK_0A;
                let mut side0b_cells = self.state & BACK_SIDE_MASK_0B;
                let mut side1_cells = self.state & BACK_SIDE_MASK_1;
                let mut side2_cells = self.state & BACK_SIDE_MASK_2;
                let mut side3a_cells = self.state & BACK_SIDE_MASK_3A;
                let mut side3b_cells = self.state & BACK_SIDE_MASK_3B;
                side0a_cells <<= 3 * 9;
                side0b_cells <<= 3 * 5;
                side1_cells <<= 3 * 33;
                side2_cells >>= 3 * 15;
                side3a_cells >>= 3 * 27;
                side3b_cells >>= 3 * 23;
                self.state ^= self.state & BACK_SIDE_MASK_ALL;
                self.state ^= side0a_cells;
                self.state ^= side0b_cells;
                self.state ^= side1_cells;
                self.state ^= side2_cells;
                self.state ^= side3a_cells;
                self.state ^= side3b_cells;
            }
            Rotation::Bp => {
                let mut back_cells = self.state & BACK_MASK;
                self.state ^= back_cells;
                back_cells <<= 3;
                let mut overflow_cells = back_cells & BACK_OVERFLOW_MASK_REV;
                back_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                back_cells ^= overflow_cells;
                self.state ^= back_cells;

                let mut side0a_cells = self.state & BACK_SIDE_MASK_0A;
                let mut side0b_cells = self.state & BACK_SIDE_MASK_0B;
                let mut side1a_cells = self.state & BACK_SIDE_MASK_1A;
                let mut side1b_cells = self.state & BACK_SIDE_MASK_1B;
                let mut side2_cells = self.state & BACK_SIDE_MASK_2;
                let mut side3_cells = self.state & BACK_SIDE_MASK_3;
                side0a_cells <<= 3 * 27;
                side0b_cells <<= 3 * 23;
                side1a_cells >>= 3 * 9;
                side1b_cells >>= 3 * 5;
                side2_cells >>= 3 * 33;
                side3_cells <<= 3 * 15;
                self.state ^= self.state & BACK_SIDE_MASK_ALL;
                self.state ^= side0a_cells;
                self.state ^= side0b_cells;
                self.state ^= side1a_cells;
                self.state ^= side1b_cells;
                self.state ^= side2_cells;
                self.state ^= side3_cells;
            }
            Rotation::D => {
                let mut down_cells = self.state & DOWN_MASK;
                self.state ^= down_cells;
                down_cells >>= 3;
                let mut overflow_cells = down_cells & DOWN_OVERFLOW_MASK;
                down_cells ^= overflow_cells;
                overflow_cells <<= 3 * 4;
                down_cells ^= overflow_cells;
                self.state ^= down_cells;

                let mut side_cells = self.state & DOWN_SIDE_MASK;
                self.state ^= side_cells;
                side_cells <<= 3 * 8;
                let mut side_overflow_cells =
                    side_cells & DOWN_SIDE_OVERFLOW_MASK;
                side_cells ^= side_overflow_cells;
                side_overflow_cells >>= 3 * 32;
                side_cells ^= side_overflow_cells;
                self.state ^= side_cells;
            }
            Rotation::Dp => {
                let mut down_cells = self.state & DOWN_MASK;
                self.state ^= down_cells;
                down_cells <<= 3;
                let mut overflow_cells = down_cells & DOWN_OVERFLOW_MASK_REV;
                down_cells ^= overflow_cells;
                overflow_cells >>= 3 * 4;
                down_cells ^= overflow_cells;
                self.state ^= down_cells;

                let mut side_cells = self.state & DOWN_SIDE_MASK;
                self.state ^= side_cells;
                side_cells >>= 3 * 8;
                let mut side_overflow_cells =
                    side_cells & DOWN_SIDE_OVERFLOW_MASK_REV;
                side_cells ^= side_overflow_cells;
                side_overflow_cells <<= 3 * 32;
                side_cells ^= side_overflow_cells;
                self.state ^= side_cells;
            }
        }
    }
}
