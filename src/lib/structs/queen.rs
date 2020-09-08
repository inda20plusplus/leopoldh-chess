use super::general;
use super::diagonal;
use super::strait;
#[allow(dead_code)]
pub fn find_moves(piece: &mut general::Piece, board: &general::Board) {
    piece.moves = vec![];
    diagonal::find_moves(piece, &board);
    strait::find_moves(piece, &board);
}