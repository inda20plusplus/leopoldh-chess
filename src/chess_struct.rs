fn notInside(&a: &i32, &b: &i32) -> bool {
    ((a < 0) || (b < 0)) || ((a > 7) || (b > 7))
}

pub fn generateKing(info: PieceInfo, &board: &Vec<Vec<i32>>) -> King {
    let mut king: King = King {
        name: "king".to_string(),
        pieceInfo: info,
        moves: vec![],
    };
    for delta_number in [-1, 0, 1].iter() {
        for delta_letter in [-1, 0, 1].iter() {
            if delta_letter == &1 && delta_number == &1 {
                continue;
            }
            if notInside(
                &(delta_number + king.pieceInfo.position.number),
                &(delta_letter + king.pieceInfo.position.letter),
            ) {
                continue;
            }
            let cur_number: usize = (delta_number + king.pieceInfo.position.number) as usize;
            let cur_letter: usize = (delta_letter + king.pieceInfo.position.letter) as usize;
            if board[cur_letter][cur_number] == king.pieceInfo.color as i32 {
                continue;
            }
        }
    }
    king
}

pub struct King {
    pub moves: Vec<Position>,
    pub pieceInfo: PieceInfo,
    pub name: String,
}

pub struct Position {
    pub letter: i32,
    pub number: i32,
}
pub struct PieceInfo {
    pub position: Position,
    pub color: bool,
    pub possible_enpassant: bool,
    pub moved: bool,
}
