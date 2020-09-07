mod chess_struct;

fn main() {
    let king = chess_struct::PlayerType{
        name : "king",
        moves : vec![(1, 0),(-1, 0),(0, 1), (0,-1)]
    };
    let white_king = chess_struct::Player{
        position: chess_struct::Position{letter:'a', number:5},
        player_type: king,
        team: true,
        moved_last: false,
        moved: false,
        movements: vec![]
    };
    println!("{:?}", white_king.position.letter);
}
