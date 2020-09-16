mod chess;

fn main() {
    let mut game = chess::Game::new();
    game.print();
    game.move_piece((0, 1), (2, 0));
    game.print();
    game.undo();
    game.print();
}
