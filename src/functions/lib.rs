mod functions;
#[allow(dead_code)]

fn main() {
    let mut board = vec![vec![2; 8]; 8];
    board[2][2] = 0;
    board[3][2] = 1;
    let white_king = functions::builder::generate_king(
        3,
        3,
        0,
        false,
        true,
        &board
    );
    let black_king = functions::builder::generate_king(
        functions::builder::structs::PieceInfo {
            position: functions::builder::structs::Position {
                letter: 3,
                number: 2,
            },
            color: 1,
            possible_enpassant: false,
            moved: true,
        },
        &board,
    );
    for i in white_king.moves.iter(){
        println!("{:?} , {:?}", i.letter, i.number);
    }
    for i in black_king.moves.iter(){
        println!("{:?} , {:?}", i.letter, i.number);
    }
}
