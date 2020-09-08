use super::general;
#[allow(dead_code)]
pub fn find_moves(piece :&mut general::Piece, board: &general::Board) {
    piece.moves = vec![];
    for delta_number in [-1, 1].iter() {
        for delta_letter in [-1, 1].iter() {
            {
                let cur_position = general::Position {
                number: (2*delta_number + piece.piece_info.position.number),
                letter: (delta_letter + piece.piece_info.position.letter),
                };
                if cur_position.inside() {
                    if board.get(&cur_position) != piece.piece_info.color {
                        piece.moves.push(cur_position);
                    }
                }
                
            }
            {
                let cur_position = general::Position {
                number: (delta_number + piece.piece_info.position.number),
                letter: (2*delta_letter + piece.piece_info.position.letter),
                };
                if cur_position.inside() {
                    if board.get(&cur_position) != piece.piece_info.color {
                        piece.moves.push(cur_position);
                    }
                }
            }
        }
    }
}
