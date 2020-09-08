use super::general;

pub fn find_moves(piece: &mut general::Piece, board: &general::Board) {
    piece.moves = vec![]; 
    {
        let mut cur_position = general::Position {
            number: piece.piece_info.position.number,
            letter: piece.piece_info.position.letter,
        };
        if piece.piece_info.color == 0{
            cur_position.number += 1;
        }else{
            cur_position.number -= 1;
        }
        if cur_position.inside() {
            if board.get(&cur_position) == 2 {
                piece.moves.push(cur_position);
            }
        } 
    }
    {
        for d_letter in [1, -1].iter(){
            let mut cur_position = general::Position {
                number: piece.piece_info.position.number,
                letter: piece.piece_info.position.letter,
            };
            if piece.piece_info.color == 0{
                cur_position.number += 1;
            }else{
                cur_position.number -= 1;
            }
            cur_position.letter += d_letter;
            if cur_position.inside() {
                if board.get(&cur_position) == piece.opponent() {
                    piece.moves.push(cur_position);
                }
            }
        }
    }
}

    


