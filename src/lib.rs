mod gamestate;
mod icon;
mod piece;
mod position;
use gamestate::Gamestate;
use position::Position;
pub struct Game {
    game: Vec<Gamestate>,
}
#[allow(unused)]
//game commands
impl Game {
    pub fn turn(&self) -> i32 {
        self.game.len() as i32
    }
    pub fn current(&mut self) -> Gamestate {
        self.game[self.game.len() - 1].clone()
    }
    pub fn new() -> Game {
        let mut ret = Game {
            game: vec![Gamestate::new()],
        };
        ret.default();
        ret
    }
    fn default(&mut self) -> () {
        if self.turn() == 1 {
            self.game[0].populate("default".to_owned());
        } else {
            panic!("reset while game is ongoing");
        }
    }
    pub fn special(&mut self, val: String) -> () {
        if self.turn() == 1 {
            self.game[0].populate("default".to_owned());
        } else {
            panic!("reset while game is ongoing");
        }
    }
    pub fn undo(&mut self) -> () {
        if self.turn() >= 2 {
            self.game.pop();
        }
    }
    pub fn move_piece(&mut self, from: (i32, i32), to: (i32, i32)) -> bool {
        let mut from = Position::new(from);
        let mut to = Position::new(to);
        let mut current = self.current();
        if current.move_piece(from, to) {
            self.game.push(current);
        }
        return false;
    }
    pub fn promote(&mut self, position: (i32, i32), to: String) -> bool {
        let mut current = self.current();
        if current.promote(Position::new(position), to) {
            self.game.push(current);
            return true;
        }
        false
    }
    pub fn small_castling(&mut self) -> bool {
        let mut current = self.current();
        if current.small_castling() {
            self.game.push(current);
            return true;
        }
        false
    }
    pub fn small_castling_available(&mut self) -> bool {
        let mut current = self.current();
        current.small_castling_available()
    }
    pub fn large_castling_available(&mut self) -> bool {
        let mut current = self.current();
        current.large_castling_available()
    }
    pub fn large_castling(&mut self) -> bool {
        let mut current = self.current();
        if current.large_castling() {
            self.game.push(current);
            return true;
        }
        false
    }
}
//debug and information
#[allow(dead_code)]
impl Game {
    pub fn print(&mut self) -> () {
        let mut current = self.current();
        for i in 0..8 {
            for j in 0..8 {
                let piece = current.get_piece(&Position::new((i as i32, j as i32)));
                print!(" {} ", icon::icon(piece.color(), piece.name()));
            }
            println!("");
        }
    }
    pub fn possible_moves(&mut self, position: (i32, i32)) -> Vec<(i32, i32)> {
        let ret = self.current().possible_moves(&Position::new(position));
        ret
    }
    pub fn stalemate(&mut self) -> bool {
        self.current().stalemate()
    }
    pub fn check(&mut self) -> bool {
        let mut current = self.current();
        current.check()
    }
}
