use super::general;
use super::gamestate;
#[allow(dead_code)]
pub fn diagonal_moves(piece :&mut general::Piece, gamestate: &gamestate::Gamestate) {
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
                    if gamestate.get(&cur_position) != 2 {
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

#[allow(dead_code)]
pub fn horse_moves(piece :&mut general::Piece, gamestate: &gamestate::Gamestate) {
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

#[allow(dead_code)]
pub fn king_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
    piece.moves = vec![];
    for delta_number in [-1, 0, 1].iter() {
        for delta_letter in [-1, 0, 1].iter() {
            let cur_position = general::Position {
                number: (delta_number + piece.piece_info.position.number),
                letter: (delta_letter + piece.piece_info.position.letter),
            };
            if cur_position.inside() {
                if board.get(&cur_position) != piece.piece_info.color {
                    piece.moves.push(cur_position);
                }
            }
            
        }
    }
}
pub fn pawn_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
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

pub fn strait_moves(piece :&mut general::Piece, gamestate: &gamestate::Gamestate) {
    for d_number in [-1, 1,0].iter(){
        for d_letter in [-1,1,0].iter(){
            if d_number*d_letter != 0{
                continue;
            }
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
#[allow(dead_code)]
pub fn queen_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
    piece.moves = vec![];
    diagonal_moves(piece, &board);
    strait_moves(piece, &board);
}
#[allow(dead_code)]
pub fn knight_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
    piece.moves = vec![];
    diagonal_moves(piece, &board);
}
#[allow(dead_code)]
pub fn tower_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
    piece.moves = vec![];
    strait_moves(piece, &board);
}
#[allow(dead_code)]
pub fn empty_moves(piece: &mut general::Piece, gamestate: &gamestate::Gamestate) {
    piece.moves = vec![];
}

