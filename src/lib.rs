pub mod cube;
pub mod face;
pub mod rotation;
pub mod solution;
pub mod solver;

#[cfg(test)]
mod tests {
    use crate::{cube::CubeState, rotation::Rotation};

    fn _solved_cube() -> CubeState {
        return CubeState::from_str(&String::from(
            "WWWWWWWWWOOOOOOOOOGGGGGGGGGRRRRRRRRRBBBBBBBBBYYYYYYYYY",
        ))
        .unwrap();
    }

    /*
     * Cube in a cube in a cube
     */
    fn ccc() -> CubeState {
        return CubeState::from_str(&String::from(
            "RWGRWWRRRYOBOOBBBBWWWWGGWGRGGGRRGWRGYBOYBBYYYBYOYYOOOO",
        ))
        .unwrap();
    }

    /*
     * Applies cube in a cube in a cube, then tests the moves
     */
    #[test]
    fn check_cube_rotations() {
        let mut state: CubeState;

        let u_state = CubeState::from_str(&String::from(
            "GWRWWRRRRYOBOOBWGRWWWWGGWRGGGGRRGYYYYBOYBBBBBBYOYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::U);
        assert_eq!(u_state, state);

        let up_state = CubeState::from_str(&String::from(
            "RRRRWWRWGYOBOOBYYYWWWWGGBBBGGGRRGWGRYBOYBBWRGBYOYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Up);
        assert_eq!(up_state, state);

        let l_state = CubeState::from_str(&String::from(
            "YWGBWWORRBBBOOBYOBRWWRGGRGRGGGRRGWRGYBOYBYYYBWYOWYOWOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::L);
        assert_eq!(l_state, state);

        let lp_state = CubeState::from_str(&String::from(
            "WWGWWWWRRBOYBOOBBBBWWYGGOGRGGGRRGWRGYBRYBRYYRYYOBYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Lp);
        assert_eq!(lp_state, state);

        let f_state = CubeState::from_str(&String::from(
            "BBBRWWRRRYOOOOOBBOWGRWGGWWWGGGWRGRRGYBOYBBYYYBYOYYOGRW",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::F);
        assert_eq!(f_state, state);

        let fp_state = CubeState::from_str(&String::from(
            "WRGRWWRRRYOROOWBBGWWWGGWRGWOGGORGORGYBOYBBYYYBYOYYOBBB",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Fp);
        assert_eq!(fp_state, state);

        let r_state = CubeState::from_str(&String::from(
            "RWWRWGRRRYOBOOBBBBWWOWGOWGOGGGGRRGRWRBOWBBGYYBYYYYYOOY",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::R);
        assert_eq!(r_state, state);

        let rp_state = CubeState::from_str(&String::from(
            "RWYRWYRRYYOBOOBBBBWWGWGWWGRWRGRRGGGGOBOOBBOYYBYWYYGOOR",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Rp);
        assert_eq!(rp_state, state);

        let b_state = CubeState::from_str(&String::from(
            "RWGRWWGGGROBROBRBBWWWWGGWGRGGBRRYWROOBYBBYYYYBOYYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::B);
        assert_eq!(b_state, state);

        let bp_state = CubeState::from_str(&String::from(
            "RWGRWWYOBOOBYOBBBBWWWWGGWGRGGRRRRWRRYYYYBBYBOGGGYYOOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Bp);
        assert_eq!(bp_state, state);

        let d_state = CubeState::from_str(&String::from(
            "RWGRWWRRRYBOOOBBBBYOBWGGWGRWWWRRGWRGGGGYBBYYYOOOYYOBYO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::D);
        assert_eq!(d_state, state);

        let dp_state = CubeState::from_str(&String::from(
            "RWGRWWRRRWWWOOBBBBGGGWGGWGRYBORRGWRGYOBYBBYYYOYBOYYOOO",
        ))
        .unwrap();
        state = ccc().clone();
        state.rotate(Rotation::Dp);
        assert_eq!(dp_state, state);
    }
}
