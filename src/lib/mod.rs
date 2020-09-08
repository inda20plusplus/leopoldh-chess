pub mod structs;
pub mod test;
pub fn main() {
    let mut black_king = structs::king::King{
        moves: vec![],
        piece_info: structs::general::PieceInfo{
            position: structs::general::Position{
                letter: 0,
                number: 0
            },
            possible_enpassant: false,
            moved: true,
            color: 1
        },
        name: "black_king".to_string(),
    };
    let mut board = structs::general::Board{
        board : vec![vec![2; 8]; 8],
    };
    board.change(&black_king.piece_info.position, black_king.piece_info.color);
    black_king.find_moves(&board);
    println!("king on {} can move to:", black_king.piece_info.position.print());
    for i in black_king.moves.iter(){
        println!("{}", i.print());
    }
    println!("test print");
}
