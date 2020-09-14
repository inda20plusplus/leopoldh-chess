mod chess;

fn main(){
    let mut game = chess::Game::new();
    let a = game.do_move((1, 1), (0,0));
    println!("{:?}", a);
}