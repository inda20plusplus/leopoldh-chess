mod gamestate;
mod icon;
mod piece;
mod position;
use gamestate::Gamestate;
use position::Position;
#[derive(Clone, PartialEq, Copy)]
pub enum Color {
    Black,
    White,
    None,
}
#[derive(Clone, PartialEq)]
pub enum Kind {
    King,
    Queen,
    Rook,
    Bishop,
    Pawn,
    Knight,
    None,
}
impl Kind {
    pub fn name(&self) -> String {
        match self {
            Kind::King => "king".to_string(),
            Kind::Queen => "queen".to_string(),
            Kind::Pawn => "pawn".to_string(),
            Kind::Knight => "knight".to_string(),
            Kind::Bishop => "bishop".to_string(),
            Kind::Rook => "rook".to_string(),
            _ => "empty".to_string(),
        }
    }
}
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
    pub fn undo(&mut self) -> bool {
        if self.turn() >= 2 {
            self.game.pop();
            return true;
        }
        false
    }
    pub fn move_piece(&mut self, from: (i32, i32), to: (i32, i32), promote: Kind) -> bool {
        let mut from = Position::new(from);
        let mut to = Position::new(to);
        let mut current = self.current();
        if current.move_piece(from, to, promote) {
            current.next();
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
        self.current().print()
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
    pub fn checkmate(&mut self) -> bool {
        let mut current = self.current();
        current.checkmate()
    }
    pub fn get_board(&mut self) -> Vec<Vec<(Kind, Color)>> {
        let mut ret: Vec<Vec<(Kind, Color)>> = vec![vec![(Kind::None, Color::None); 8]; 8];
        let mut current = self.current();
        for i in 0..8 {
            for j in 0..8 {
                let piece = current.get_piece(&Position::new((i as i32, j as i32)));
                ret[i][j] = (piece.kind().clone(), piece.color().clone());
            }
        }
        ret
    }
}
