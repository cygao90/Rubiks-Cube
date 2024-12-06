use bevy::color::Color;
use kewb::{CubieCube, DataTable, FaceCube, Move, Solver};
use crate::cube::{Cube, Direction, Face, Movement, RotateAxis};
//              |************|
//              |*U1**U2**U3*|
//              |************|
//              |*U4**U5**U6*|
//              |************|
//              |*U7**U8**U9*|
//              |************|
// |************|************|************|************|
// |*L1**L2**L3*|*F1**F2**F3*|*R1**R2**R3*|*B1**B2**B3*|
// |************|************|************|************|
// |*L4**L5**L6*|*F4**F5**F6*|*R4**R5**R6*|*B4**B5**B6*|
// |************|************|************|************|
// |*L7**L8**L9*|*F7**F8**F9*|*R7**R8**R9*|*B7**B8**B9*|
// |************|************|************|************|
//              |************|
//              |*D1**D2**D3*|
//              |************|
//              |*D4**D5**D6*|
//              |************|
//              |*D7**D8**D9*|
//              |************|
// A cube definition string "UBL..." means for example: In position U1 we have the U-color, in position U2 we have the
// B-color, in position U3 we have the L color etc. according to the order U1, U2, U3, U4, U5, U6, U7, U8, U9, R1, R2,
// R3, R4, R5, R6, R7, R8, R9, F1, F2, F3, F4, F5, F6, F7, F8, F9, D1, D2, D3, D4, D5, D6, D7, D8, D9, L1, L2, L3, L4,
// L5, L6, L7, L8, L9, B1, B2, B3, B4, B5, B6, B7, B8, B9 of the enum constants.

struct CubeState {
    color_up: Color,
    color_down: Color,
    color_front: Color,
    color_back: Color,
    color_left: Color,
    color_right: Color,
}

impl From<&Vec<Cube>> for CubeState {
    fn from(value: &Vec<Cube>) -> Self {
        let [mut f, mut b, mut u, mut d, mut l, mut r] = [Color::BLACK; 6];
        for cube in value {
            let [x, y, z] = cube.coord;
            if x == 2 && y == 1 && z == 1 {
                r = *cube.colors.get(&Face::RIGHT).unwrap();
            } else if x == 0 && y == 1 && z == 1 {
                l = *cube.colors.get(&Face::LEFT).unwrap();
            } else if y == 2 && x == 1 && z == 1 {
                u = *cube.colors.get(&Face::UP).unwrap();
            } else if y == 0 && x == 1 && z == 1 {
                d = *cube.colors.get(&Face::DOWN).unwrap();
            } else if z == 2 && x == 1 && y == 1 {
                f = *cube.colors.get(&Face::FRONT).unwrap();
            } else if z == 0 && x == 1 && y == 1 {
                b = *cube.colors.get(&Face::BACK).unwrap();
            }
        }

        Self { color_up: u, color_down: d, color_front: f, color_back: b, color_left: l, color_right: r }
    }
}

fn index_u(x: i32, z: i32) -> usize { (z * 3 + x) as usize }
fn index_d(x: i32, z: i32) -> usize { ((-z + 2) * 3 + x) as usize }
fn index_f(x: i32, y: i32) -> usize { ((-y + 2) * 3 + x) as usize }
fn index_b(x: i32, y: i32) -> usize { ((-y + 2) * 3 + (-x + 2)) as usize }
fn index_l(y: i32, z: i32) -> usize { ((-y + 2) * 3 + z) as usize }
fn index_r(y: i32, z: i32) -> usize { ((-y + 2) * 3 + (-z + 2)) as usize }

fn get_original_position(cube_state: &CubeState, color: &Color) -> char {
    if cube_state.color_up == *color {
        'U'
    } else if cube_state.color_down == *color {
        'D'
    } else if cube_state.color_front == *color {
        'F'
    } else if cube_state.color_back == *color {
        'B'
    } else if cube_state.color_left == *color {
        'L'
    } else if cube_state.color_right == *color {
        'R'
    } else {
        unreachable!()
    }
}

fn cube_state_to_string(
    cubes: &Vec<Cube>,
    cube_state: &CubeState,
) -> String {

    let mut ret: Vec<char> = vec!['0'; 54];

    for cube in cubes {
        let [x, y, z] = cube.coord;
        // U
        if y == 2 {
            let start: usize = 0;
            ret[start + index_u(x, z)] = get_original_position(&cube_state, cube.colors.get(&Face::UP).unwrap());
        } 
        
        // D
        if y == 0 {
            let start: usize = 27;
            ret[start + index_d(x, z)] = get_original_position(&cube_state, cube.colors.get(&Face::DOWN).unwrap());
        }

        // F
        if z == 2 {
            let start: usize = 18;
            ret[start + index_f(x, y)] = get_original_position(&cube_state, cube.colors.get(&Face::FRONT).unwrap());
        }

        // B
        if z == 0 {
            let start: usize = 45;
            ret[start + index_b(x, y)] = get_original_position(&cube_state, cube.colors.get(&Face::BACK).unwrap());
        }

        // L
        if x == 0 {
            let start: usize = 36;
            ret[start + index_l(y, z)] = get_original_position(&cube_state, cube.colors.get(&Face::LEFT).unwrap());
        }

        // R
        if x == 2 {
            let start: usize = 9;
            ret[start + index_r(y, z)] = get_original_position(&cube_state, cube.colors.get(&Face::RIGHT).unwrap());
        }
    }

    ret.iter().collect()
}

fn generate_moves_from_string(moves: Vec<Move>) -> Vec<Movement> {
    let mut ret = Vec::new();
    for step in moves {
        let mut m = Movement {
            axis: RotateAxis::X,
            layer: 0,
            direction: Direction::Clockwise
        };

        let mut double = false;

        match step {
            Move::U | Move::U2 | Move::U3 => {
                m.axis = RotateAxis::Y;
                m.layer = 2;
                if step == Move::U2 {
                    double = true;
                }
                if step == Move::U3 {
                    m.direction = Direction::CounterClockwise;
                }
            },
            Move::D | Move::D2 | Move::D3 => {
                m.axis = RotateAxis::Y;
                m.layer = 0;
                if step == Move::D2 {
                    double = true;
                }
                if step == Move::D {
                    m.direction = Direction::CounterClockwise;
                }
            },
            Move::R | Move::R2 | Move::R3 => {
                m.axis = RotateAxis::X;
                m.layer = 2;
                if step == Move::R2 {
                    double = true;
                }
                if step == Move::R3 {
                    m.direction = Direction::CounterClockwise;
                }
            },
            Move::L | Move::L2 | Move::L3 => {
                m.axis = RotateAxis::X;
                m.layer = 0;
                if step == Move::L2 {
                    double = true;
                }
                if step == Move::L {
                    m.direction = Direction::CounterClockwise;
                }
            },
            Move::F | Move::F2 | Move::F3 => {
                m.axis = RotateAxis::Z;
                m.layer = 2;
                if step == Move::F2 {
                    double = true;
                }
                if step == Move::F3 {
                    m.direction = Direction::CounterClockwise;
                }
            },
            Move::B | Move::B2 | Move::B3 => {
                m.axis = RotateAxis::Z;
                m.layer = 0;
                if step == Move::B2 {
                    double = true;
                }
                if step == Move::B {
                    m.direction = Direction::CounterClockwise;
                }
            },
        }
        
        ret.push(m);
        if double {
            ret.push(m);
        }
    }
    ret
}

pub async fn solve(
    cubes: Vec<Cube>,
) -> Vec<Movement> {
    let cube_state = CubeState::from(&cubes);
    let state_str = cube_state_to_string(&cubes, &cube_state);
    let state = CubieCube::try_from(&FaceCube::try_from(state_str.as_str()).unwrap()).unwrap();
    let table = DataTable::default();
    let mut solver = Solver::new(&table, 23);
    generate_moves_from_string(solver.solve(state).unwrap().get_all_moves())
}