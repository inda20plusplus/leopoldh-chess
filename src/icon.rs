pub fn icon(color: i32, name: String) -> String {
    match name.as_str() {
        "king" if color == 0 => return "♔".to_owned(),
        "queen" if color == 0 => return "♕".to_owned(),
        "pawn" if color == 0 => return "♙".to_owned(),
        "rook" if color == 0 => return "♖".to_owned(),
        "bishop" if color == 0 => return "♗".to_owned(),
        "knight" if color == 0 => return "♘".to_owned(),
        "king" if color == 1 => return "♚".to_owned(),
        "queen" if color == 1 => return "♛".to_owned(),
        "pawn" if color == 1 => return "♟".to_owned(),
        "rook" if color == 1 => return "♜".to_owned(),
        "bishop" if color == 1 => return "♝".to_owned(),
        "knight" if color == 1 => return "♞".to_owned(),
        _ if color == 1 || color == 0 => panic!("bad name"),
        _ => return "_".to_owned(),
    }
}
