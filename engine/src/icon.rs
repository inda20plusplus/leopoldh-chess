use super::{Color, Kind};
pub fn icon(color: Color, kind: Kind) -> String {
    match kind {
        Kind::King if color == Color::White => return "♔".to_owned(),
        Kind::Queen if color == Color::White => return "♕".to_owned(),
        Kind::Pawn if color == Color::White => return "♙".to_owned(),
        Kind::Rook if color == Color::White => return "♖".to_owned(),
        Kind::Bishop if color == Color::White => return "♗".to_owned(),
        Kind::Knight if color == Color::White => return "♘".to_owned(),
        Kind::King if color == Color::Black => return "♚".to_owned(),
        Kind::Queen if color == Color::Black => return "♛".to_owned(),
        Kind::Pawn if color == Color::Black => return "♟".to_owned(),
        Kind::Rook if color == Color::Black => return "♜".to_owned(),
        Kind::Bishop if color == Color::Black => return "♝".to_owned(),
        Kind::Knight if color == Color::Black => return "♞".to_owned(),
        _ if color == Color::Black || color == Color::White => panic!("bad name"),
        _ => return "_".to_owned(),
    }
}
