use super::general;
#[allow(dead_code)]
pub fn find_moves(piece :&mut general::Piece, board: &general::Board) {
    for d_number in [-1, 1].iter(){
        for d_letter in [-1,1].iter(){
            let mut moving = true;
            let mut cur_position = general::Position {
                number:  piece.piece_info.position.number ,
                letter: piece.piece_info.position.letter ,
            };
            while moving {
                cur_position.number+=d_number;
                cur_position.letter+=d_letter;
                if cur_position.inside() {
                    if board.get(&cur_position) != 2 {
                        moving = false;
                    }
                    if board.get(&cur_position) == piece.piece_info.color{
                        piece.moves.push(cur_position.clone());
                    }
                }
            }
        }
    }
}