use std::char;
#[allow(unused)]
#[allow(dead_code)]
pub struct Position {
    pub letter: i32,
    pub number: i32,
}
impl Position {
    pub fn print(&self) -> String {
        let mut ret: String = char::from_u32((self.letter + 0x41) as u32)
            .unwrap()
            .to_string();
        ret.push_str(&(self.number + 1).to_string());
        ret
    }
    pub fn not_inside(&self) -> bool {
        ((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7))
    }
    pub fn inside(&self) -> bool {
        !((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7))
    }
}
pub struct PieceInfo {
    pub position: Position,
    pub color: i32,
    pub possible_enpassant: bool,
    pub moved: bool,
}

pub struct Board {
    pub board: Vec<Vec<i32>>,
}
impl Board {
    pub fn change(&mut self, position: &Position, value: i32) {
        self.board[position.letter as usize][position.number as usize] = value;
    }
    pub fn get(&self, position: &Position) -> i32 {
        self.board[position.letter as usize][position.number as usize]
    }
}

pub struct piece {
    pub moves: Vec<general::Position>,
    pub piece_info: general::PieceInfo,
    pub name: String,
    pub find_moves: fn(&mut self, board: &general::Board)
}