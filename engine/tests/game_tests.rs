use engine;
use engine::Kind;
#[test]
fn it_adds_two() {
    let mut game = engine::Game::new();
    assert!(game.move_piece((1, 0), (3, 0), Kind::None));
    assert!(game.move_piece((6, 1), (4, 1), Kind::None));
    assert!(game.move_piece((3, 0), (4, 1), Kind::None));
    assert!(game.move_piece((6, 0), (4, 0), Kind::None));
    assert!(game.move_piece((4, 1), (5, 0), Kind::None));
    assert!(game.move_piece((7, 1), (5, 0), Kind::None));
    assert!(game.move_piece((0, 0), (5, 0), Kind::None));
    assert!(game.move_piece((7, 2), (5, 0), Kind::None));
    assert!(game.move_piece((0, 6), (2, 7), Kind::None));
    assert!(game.move_piece((6, 2), (4, 2), Kind::None));
    assert!(game.move_piece((1, 4), (2, 4), Kind::None));
    assert!(game.move_piece((7, 3), (6, 2), Kind::None));
    assert!(game.move_piece((0, 5), (4, 1), Kind::None));
    assert!(game.move_piece((6, 7), (4, 7), Kind::None));
    assert!(game.move_piece((4, 1), (6, 3), Kind::None));
    assert!(game.move_piece((7, 4), (7, 3), Kind::None));
    assert!(game.move_piece((6, 3), (5, 4), Kind::None));
    assert!(game.move_piece((7, 3), (7, 4), Kind::None));
    assert!(game.small_castling());
    assert!(!game.large_castling());
    assert!(game.move_piece((4, 2), (3, 2), Kind::None));
    assert!(game.move_piece((1, 1), (2, 1), Kind::None));
    assert!(game.move_piece((3, 2), (2, 1), Kind::None));
    assert!(game.move_piece((0, 1), (2, 0), Kind::None));
    assert!(game.move_piece((2, 1), (1, 1), Kind::None));
    assert!(game.move_piece((2, 0), (4, 1), Kind::None));
    assert!(game.move_piece((1, 1), (0, 1), Kind::Queen));
    assert!(game.move_piece((4, 1), (2, 0), Kind::None));
    assert!(game.move_piece((0, 1), (0, 2), Kind::None));
    assert!(game.move_piece((2, 0), (4, 1), Kind::None));
    assert!(game.move_piece((0, 2), (0, 3), Kind::None));
    assert!(game.move_piece((0, 5), (0, 4), Kind::None));
    assert!(game.move_piece((0, 3), (0, 4), Kind::None));
    assert!(!game.move_piece((0, 6), (0, 5), Kind::None));
    game.undo();
    game.print();
    println!("{}", game.stalemate());
}
#[test]
fn gamestate_and_moves() {
    let mut game = engine::Game::new();
    let _board = game.get_board();

    assert!(game.move_piece((0, 1), (2, 0), Kind::None));
    assert!(!game.small_castling());
    assert!(!game.large_castling());
    assert!(!game.check());
    assert!(!game.checkmate());
    assert!(!game.stalemate());
    game.print();
    assert!(game.move_piece((6, 0), (5, 0), Kind::None));
    assert_eq!(game.possible_moves((0, 0)).len(), 1);
    game.undo();
    game.undo();
    assert_eq!(game.possible_moves((0, 0)).len(), 0);
}
