
use crate::squaretypes::*;

pub struct TetrisBoard {
    
    pub height: i32,
    pub width: i32,

    pub board: Vec<Vec<SquareType>>,

    pub tetromino_pos: (i32, i32),
    pub curr_tetromino: TetrominoType,

}

impl TetrisBoard {

    pub fn new(width: i32, height: i32) {
        let mut board: Vec<&Vec<SquareType>> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                // TODO not done
            }
        }
    }

}

