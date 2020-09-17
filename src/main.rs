mod chess;

fn main() {
    let mut game = chess::Game::new();
    game.print();
    println!("{}",game.small_castling());
    game.print();
    println!("{}",game.large_castling());
    game.print();
}
