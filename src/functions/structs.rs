use std::char;
pub mod players;
pub mod general;

pub struct Position {
    pub letter: i32,
    pub number: i32,
}
impl Position {
    pub fn print(&self) -> String{
        let mut ret: String = char::from_u32((0x41) as u32).unwrap().to_string();
        println!("{}", char::from_u32((0x41) as u32).unwrap().to_string());
        //ret.push_str(&self.number.to_string());
        ret
    }
}
pub struct PieceInfo {
    pub position: Position,
    pub color: i32,
    pub possible_enpassant: bool,
    pub moved: bool,
}

pub struct Board{
    pub board: Vec<Vec<i32>>
}
impl Board {
    pub fn change(&mut self, position: &Position, value: i32){
        self.board[position.letter as usize][position.number as usize] = value;
    }
}
