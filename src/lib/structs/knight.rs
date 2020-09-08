use super::general;
use super::diagonal;
#[allow(dead_code)]
pub fn find_moves(piece: &mut general::Piece, board: &general::Board) {
    piece.moves = vec![];
    diagonal::find_moves(piece, &board);
}