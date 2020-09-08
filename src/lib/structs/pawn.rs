use super::general;
#[allow(unused)]
pub struct Pawn {
    pub moves: Vec<general::Position>,
    pub piece_info: general::PieceInfo,
    pub name: String,
}
#[allow(dead_code)]
impl Pawn {
    pub fn find_moves(&mut self, board: &general::Board) {
        self.moves = vec![];
        {   
            let mut cur_position = general::Position {
                number: self.piece_info.position.number,
                letter: self.piece_info.position.letter,
            };
            if self.piece_info.color == 0{
                cur_position.number += 1;
            }else{
                cur_position.number -= 1;
            }
            if cur_position.inside() {
                if board.get(&cur_position) == 2 {
                    self.moves.push(cur_position);
                }
            }
        }
        {   
            let mut cur_position = general::Position {
                number: self.piece_info.position.number,
                letter: 1 + self.piece_info.position.letter,
            };
            if self.piece_info.color == 0{
                cur_position.number += 1;
            }else{
                cur_position.number -= 1;
            }
            if cur_position.inside() {
                if self.piece_info.color == 0{
                    if board.get(&cur_position) == 1 {
                        self.moves.push(cur_position);
                    }
                }else{
                    if board.get(&cur_position) == 0 {
                        self.moves.push(cur_position);
                    }
                }
            }
        }
        {   
            let mut cur_position = general::Position {
                number: 1 + self.piece_info.position.number,
                letter: self.piece_info.position.letter -1,
            };
            if self.piece_info.color == 0{
                cur_position.number += 1;
            }else{
                cur_position.number -= 1;
            }
            if cur_position.inside() {
                if self.piece_info.color == 0{
                    if board.get(&cur_position) == 1 {
                        self.moves.push(cur_position);
                    }
                }else{
                    if board.get(&cur_position) == 0 {
                        self.moves.push(cur_position);
                    }
                }
            }
        }
        
    }

    pub fn move_player(
        &mut self,
        board: &mut general::Board,
        from: &general::Position,
        to: &general::Position,
    ) {
        board.change(from, 2);
        board.change(to, self.piece_info.color);
    }
}


