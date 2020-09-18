
use leopoldh_chess::Game;
fn main(){
    let mut game = Game::new();
    let _board = game.get_board(); //Vec<Vec<(String, i32)>> Name of piece and color(0 = white)
    game.move_piece((0, 1), (2, 0), None);//promote = None or Some(name of piece to promote if needed| "queen", "knight", "rook", "bishop")
    game.small_castling();
    game.large_castling();
    game.check();
    game.checkmate();
    game.stalemate();
    game.print();//print the board (obs A1/(0,0) bottom left corner)
    game.undo();
    game.possible_moves((0, 0));
}
/* 
piece names:
"king" 
"queen" 
"pawn" 
"rook" 
"bishop"
"knight"
piece color:
0 = white
1 = black
2 = empty
*/
