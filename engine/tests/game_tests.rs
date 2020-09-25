use engine;

#[test]
fn it_adds_two() {
    let mut game = engine::Game::new();
    assert!(game.move_piece((1, 0), (3, 0), None));
    assert!(game.move_piece((6, 1), (4, 1), None));
    assert!(game.move_piece((3, 0), (4, 1), None));
    assert!(game.move_piece((6, 0), (4, 0), None));
    assert!(game.move_piece((4, 1), (5, 0), None));
    assert!(game.move_piece((7, 1), (5, 0), None));
    assert!(game.move_piece((0, 0), (5, 0), None));
    assert!(game.move_piece((7, 2), (5, 0), None));
    assert!(game.move_piece((0, 6), (2, 7), None));
    assert!(game.move_piece((6, 2), (4, 2), None));
    assert!(game.move_piece((1, 4), (2, 4), None));
    assert!(game.move_piece((7, 3), (6, 2), None));
    assert!(game.move_piece((0, 5), (4, 1), None));
    assert!(game.move_piece((6, 7), (4, 7), None));
    assert!(game.move_piece((4, 1), (6, 3), None));
    assert!(game.move_piece((7, 4), (7, 3), None));
    assert!(game.move_piece((6, 3), (5, 4), None));
    assert!(game.move_piece((7, 3), (7, 4), None));
    assert!(game.small_castling());
    assert!(!game.large_castling());
    assert!(game.move_piece((4, 2), (3, 2), None));
    assert!(game.move_piece((1, 1), (2, 1), None));
    assert!(game.move_piece((3, 2), (2, 1), None));
    assert!(game.move_piece((0, 1), (2, 0), None));
    assert!(game.move_piece((2, 1), (1, 1), None));
    assert!(game.move_piece((2, 0), (4, 1), None));
    assert!(game.move_piece((1, 1), (0, 1), Some("queen".to_string())));
    assert!(game.move_piece((4, 1), (2, 0), None));
    assert!(game.move_piece((0, 1), (0, 2), None));
    assert!(game.move_piece((2, 0), (4, 1), None));
    assert!(game.move_piece((0, 2), (0, 3), None));
    assert!(game.move_piece((0, 5), (0, 4), None));
    assert!(game.move_piece((0, 3), (0, 4), None));
    assert!(!game.move_piece((0, 6), (0, 5), None));
    game.undo();
    game.print();
    println!("{}", game.stalemate());
}
