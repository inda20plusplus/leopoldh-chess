mod chess;

fn main(){
    let mut game = chess::Gamestate::new();
    game.start();
    let moves = game.all_possible_moves();
    for i in moves.iter(){
        println!("{}->{:?}", i.0, i.1);
    }
    let a = game.move_piece(0, 1, 2, 0);
    println!("{:?}", a);
}