pub mod structs;
#[allow(dead_code)]
fn not_inside(&a: &i32, &b: &i32) -> bool {
    ((a < 0) || (b < 0)) || ((a > 7) || (b > 7))
}

pub fn generate_king(
    letter: i32,
    number: i32,
    color: i32,
    possible_enpassant: bool,
    moved: bool,
    board: &Vec<Vec<i32>>) -> structs::King {
    let mut king: structs::King = structs::King {
        name: "king".to_string(),
        piece_info: structs::PieceInfo{
            position: structs::Position{
                letter,
                number,
            },
            color,
            possible_enpassant,
            moved,
        },
        moves: vec![],
    };
    for delta_number in [-1, 0, 1].iter() {
        for delta_letter in [-1, 0, 1].iter() {
            if not_inside(
                &(delta_number + king.piece_info.position.number),
                &(delta_letter + king.piece_info.position.letter),
            ) {
                continue;
            }
            let cur_number: usize = (delta_number + king.piece_info.position.number) as usize;
            let cur_letter: usize = (delta_letter + king.piece_info.position.letter) as usize;
            if board[cur_letter][cur_number] == king.piece_info.color {
                continue;
            }
            king.moves.push(structs::Position {
                letter: cur_letter as i32,
                number: cur_number as i32,
            });
        }
    }
    king
}