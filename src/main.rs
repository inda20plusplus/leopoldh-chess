mod chess;

fn main(){
    let mut game = chess::Gamestate::new();
    game.start();
    let moves = game.all_possible_moves();
    for i in moves.iter(){
        println!("{}->{:?}", i.0, i.1);
    }
}