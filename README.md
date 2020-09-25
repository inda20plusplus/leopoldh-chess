# leopoldh-chess
Rust chess library to build great chess games with.
Following the rules from [Rules of chess](https://en.wikipedia.org/wiki/Rules_of_chess).




## Docs
### Commands
```rust
let mut game = Game::new(); //returns a board populated with default setup
let board = game.get_board(); //Vec<Vec<(String, i32)>> Name of piece and color
game.move_piece((0, 1), (2, 0), Kind::None);//promote = Kind
game.small_castling();//executes move if possible
game.large_castling();//executes move if possible
game.small_castling_available();//checks if possible
game.large_castling_available();//checks if possible
game.check();//current player in check
game.checkmate();//game ended current player lost
game.stalemate();//game draw 
game.print();//print the board (obs A1/(0,0) bottom left corner)
game.undo();//undo last move
game.possible_moves((0, 0));//possible moves for a position
```
