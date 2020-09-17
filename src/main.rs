use leopoldh_chess::Game;
fn main() {
    let mut game = Game::new();
    game.print();
    println!("{}", game.small_castling());
    game.print();
    println!("{}", game.large_castling());
    game.print();
}
