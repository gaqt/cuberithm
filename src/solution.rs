use crate::cube::CubeState;
use crate::rotation::Rotation;

pub type Solution = Vec<Rotation>;

pub fn has_useless_moves(initial_state: CubeState, solution: &[Rotation]) -> bool {
    if solution.len() <= 1 {
        return false;
    }

    let mut path = vec![initial_state];
    for &rot in solution {
        let new_state = path.last().unwrap().rotate(rot);
        if path.iter().rposition(|&x| x == new_state).is_some() {
            return true;
        }
        path.push(new_state);
    }

    for (idx, &rot) in solution.iter().enumerate() {
        if is_rot_useless(&solution[..idx], rot) {
            return true
        }
    }

    false
}

pub fn is_rot_useless(solution: &[Rotation], rot: Rotation) -> bool {
    if solution.len() == 0 {
        return false;
    }

    let face = rot.face();
    let mut fnet = if rot.is_prime() { -1i8 } else { 1i8 };
    let mut ftot = 1i8;
    let mut onet = 0i8;
    let mut otot = 0i8;

    for &rot in solution.iter().rev() {
        if rot.face() != face && rot.opposite_face() != face {
            break
        }

        if rot.face() == face {
            ftot += 1;
            if rot.is_prime() {
                fnet -= 1;
            } else {
                fnet += 1;
            }
        } else {
            otot += 1;
            if rot.is_prime() {
                onet -= 1;
            } else {
                onet += 1;
            }
        }
    }

    (fnet.abs() != ftot || ftot > 2) || (onet.abs() != otot || otot > 2)
}
