use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    White,
    Orange,
    Green,
    Red,
    Blue,
    Yellow,
    #[default]
    Unspecified,
}

impl Color {
    pub fn from_char(c: char) -> Color {
        match c {
            'W' => Color::White,
            'O' => Color::Orange,
            'G' => Color::Green,
            'R' => Color::Red,
            'B' => Color::Blue,
            'Y' => Color::Yellow,
            'N' => Color::Unspecified,
            _ => panic!("Invalid color"),
        }
    }
}

#[derive(Clone, Default, Eq, Hash, Debug)]
pub struct Side {
    pub colors: [[Color; 3]; 3],
}

impl PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if self.colors[x][y] != other.colors[x][y] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Index<usize> for Side {
    type Output = [Color; 3];

    fn index(&self, x: usize) -> &Self::Output {
        &self.colors[x]
    }
}

impl IndexMut<usize> for Side {
    fn index_mut(&mut self, x: usize) -> &mut Self::Output {
        &mut self.colors[x]
    }
}

impl Side {
    /*
     * uses complex plane rotation
     * i(x + yi) is equivalent to rotating (xi + y) 90 degrees counterclockwise,
     * where x is the x component and y is the y component.
     * lets denote counterclockwise rotation with rot' and clockwise rotation with rot
     * note that complex plane rotation assumes the axis is at 0, but our axis is at 1,
     * since 0, 1, 2 are the indexes on the array, not -1, 0, 1.
     * Then, we must subtract 1 from the components before doing complex plane multiplication,
     * then add 1 again on each component once we are done:
     * rot'(x, y) = i((x-1) + i(y-1)) + (1+i) = i(x-1) + i²(y-1) + (1+i) = i(x-1) - (y-1) + (1+i)
     * = i(x-1) + (1-y) + (1+i) = ix + (2-y) =>
     * rot'(x, y) = (2-y, x) (point representation)
     * in other words, the new y component is x and the new x component is 2-y
     * rot(x, y) = rot'(rot'(rot'(x, y))) = rot'(rot'(2-y, x)) = rot'(2-x, 2-y) = (y, 2-x)
     * rot(x, y) = (y, 2-x)
     *
     * conclusion:
     *
     * rot(x, y) = (y, 2-x)
     * rot'(x, y) = (2-y, x)
     */

    pub fn rotate_clockwise(&mut self) {
        let mut new_colors: [[Color; 3]; 3] = Default::default();
        for x in 0..3 {
            for y in 0..3 {
                new_colors[y][2 - x] = self.colors[x][y];
            }
        }
        self.colors = new_colors;
    }

    pub fn rotate_counterclockwise(&mut self) {
        let mut new_colors: [[Color; 3]; 3] = Default::default();
        for x in 0..3 {
            for y in 0..3 {
                new_colors[2 - y][x] = self.colors[x][y];
            }
        }
        self.colors = new_colors;
    }
}
