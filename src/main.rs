mod chess;

fn main(){
    let mut game = chess::Game::new();
    game.current.start();
    let moves = game.current.all_possible_moves();
    for i in moves.iter(){
        println!("{}->{:?}", i.0, i.1);
    }
    let a = game.current.move_piece(0, 1, 2, 0);
    println!("{:?}", a);
}