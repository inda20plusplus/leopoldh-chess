use leopoldh_chess::Game;
fn main() {
    let mut game = Game::new();
    game.print();
    game.move_piece((1, 0), (3, 0), None);
    game.move_piece((6, 1), (4, 1), None);
    game.move_piece((3, 0), (4,1), None);
    game.move_piece((7, 1), (5,0), None);
    game.move_piece((4, 1), (5,1), None);
    game.move_piece((7, 0), (7,1), None);
    game.move_piece((5, 1), (6,1), None);
    game.move_piece((7, 1), (7,0), None);
    game.move_piece((6, 1), (7,1), Some("queen".to_owned()));
    game.move_piece((7, 2), (6,1), None);
    game.move_piece((7, 1), (7,3), None);
    game.move_piece((7, 4), (7,3), None);
    game.print();
}
