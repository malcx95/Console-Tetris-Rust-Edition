
use crate::squaretypes::*;

pub struct TetrisBoard {
    
    pub height: i32,
    pub width: i32,

    pub board: Vec<Vec<SquareType>>,

    pub tetromino_pos: Option<(i32, i32, TetrominoType)>,

}

impl TetrisBoard {

    pub fn new(width: i32, height: i32) -> Self {
        let mut board: Vec<Vec<SquareType>> = Vec::new();
        for _ in 0..height {
            let mut row: Vec<SquareType> = Vec::new();
            for _ in 0..width {
                row.push(SquareType::EMPTY);
            }
            board.push(row);
        }
        Self {
            height: height,
            width: width,
            board: board,
            tetromino_pos: None,
        }
    }

}

