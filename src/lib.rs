pub mod cube;
pub mod face;
pub mod rotation;
pub mod solution;
pub mod solver;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{cube::CubeState, rotation::Rotation};

    fn _solved_cube() -> CubeState {
        CubeState::from_str(&String::from(
            "WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY",
        ))
        .unwrap()
    }

    /*
     * Cube in a cube in a cube
     */
    fn ccc() -> CubeState {
        CubeState::from_str(&String::from(
            "RWGRWWRRRYOBOOBBBBWWWWGGWGRGGGRRGWRGYBOYBBYYYBYOYYOOOO",
        ))
        .unwrap()
    }

    #[test]
    fn u_rotation() {
        let u_state = CubeState::from_str(&String::from(
            "GWRWWRRRRYOBOOBWGRWWWWGGWRGGGGRRGYYYYBOYBBBBBBYOYYOOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::U);
        assert_eq!(u_state, state);
    }

    #[test]
    fn up_rotation() {
        let up_state = CubeState::from_str(&String::from(
            "RRRRWWRWGYOBOOBYYYWWWWGGBBBGGGRRGWGRYBOYBBWRGBYOYYOOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Up);
        assert_eq!(up_state, state);
    }

    #[test]
    fn l_rotation() {
        let l_state = CubeState::from_str(&String::from(
            "YWGBWWORRBBBOOBYOBRWWRGGRGRGGGRRGWRGYBOYBYYYBWYOWYOWOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::L);
        assert_eq!(l_state, state);
    }

    #[test]
    fn lp_rotation() {
        let lp_state = CubeState::from_str(&String::from(
            "WWGWWWWRRBOYBOOBBBBWWYGGOGRGGGRRGWRGYBRYBRYYRYYOBYOOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Lp);
        assert_eq!(lp_state, state);
    }

    #[test]
    fn f_rotation() {
        let f_state = CubeState::from_str(&String::from(
            "BBBRWWRRRYOOOOOBBOWGRWGGWWWGGGWRGRRGYBOYBBYYYBYOYYOGRW",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::F);
        assert_eq!(f_state, state);
    }

    #[test]
    fn fp_rotation() {
        let fp_state = CubeState::from_str(&String::from(
            "WRGRWWRRRYOROOWBBGWWWGGWRGWOGGORGORGYBOYBBYYYBYOYYOBBB",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Fp);
        assert_eq!(fp_state, state);
    }

    #[test]
    fn r_rotation() {
        let r_state = CubeState::from_str(&String::from(
            "RWWRWGRRRYOBOOBBBBWWOWGOWGOGGGGRRGRWRBOWBBGYYBYYYYYOOY",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::R);
        assert_eq!(r_state, state);
    }

    #[test]
    fn rp_rotation() {
        let rp_state = CubeState::from_str(&String::from(
            "RWYRWYRRYYOBOOBBBBWWGWGWWGRWRGRRGGGGOBOOBBOYYBYWYYGOOR",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Rp);
        assert_eq!(rp_state, state);
    }

    #[test]
    fn b_rotation() {
        let b_state = CubeState::from_str(&String::from(
            "RWGRWWGGGROBROBRBBWWWWGGWGRGGBRRYWROOBYBBYYYYBOYYYOOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::B);
        assert_eq!(b_state, state);
    }

    #[test]
    fn bp_rotation() {
        let bp_state = CubeState::from_str(&String::from(
            "RWGRWWYOBOOBYOBBBBWWWWGGWGRGGRRRRWRRYYYYBBYBOGGGYYOOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Bp);
        assert_eq!(bp_state, state);
    }

    #[test]
    fn d_rotation() {
        let d_state = CubeState::from_str(&String::from(
            "RWGRWWRRRYBOOOBBBBYOBWGGWGRWWWRRGWRGGGGYBBYYYOOOYYOBYO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::D);
        assert_eq!(d_state, state);
    }

    #[test]
    fn dp_rotation() {
        let dp_state = CubeState::from_str(&String::from(
            "RWGRWWRRRWWWOOBBBBGGGWGGWGRYBORRGWRGYOBYBBYYYOYBOYYOOO",
        ))
        .unwrap();
        let state = ccc();
        let state = state.rotate(Rotation::Dp);
        assert_eq!(dp_state, state);
    }
}
