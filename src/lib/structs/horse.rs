use super::general;
#[allow(unused)]
pub struct Horse {
    pub moves: Vec<general::Position>,
    pub piece_info: general::PieceInfo,
    pub name: String,
}
#[allow(dead_code)]
impl Horse {
    pub fn find_moves(&mut self, board: &general::Board) {
        self.moves = vec![];
        for delta_number in [-1, 1].iter() {
            for delta_letter in [-1, 1].iter() {
                {
                    let cur_position = general::Position {
                    number: (2*delta_number + self.piece_info.position.number),
                    letter: (delta_letter + self.piece_info.position.letter),
                    };
                    if cur_position.not_inside() {
                        continue;
                    }
                    if board.get(&cur_position) == self.piece_info.color {
                        continue;
                    }
                    self.moves.push(cur_position);
                }
                {
                    let cur_position = general::Position {
                    number: (delta_number + self.piece_info.position.number),
                    letter: (2*delta_letter + self.piece_info.position.letter),
                    };
                    if cur_position.not_inside() {
                        continue;
                    }
                    if board.get(&cur_position) == self.piece_info.color {
                        continue;
                    }
                    self.moves.push(cur_position);
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
