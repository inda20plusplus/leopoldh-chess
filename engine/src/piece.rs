use super::Color;
use super::Kind;
#[derive(Clone)]
pub struct Piece {
    color: Color,
    kind: Kind,
    moved: bool,
}
#[allow(dead_code)]
impl Piece {
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn moved(&self) -> bool {
        self.moved
    }
    pub fn change_name(&mut self, new: Kind) -> () {
        self.kind = new;
    }
    pub fn opponent(&self) -> Color {
        match self.color {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => panic!("bad color"),
        }
    }
    pub fn clear(&mut self) {
        self.color = Color::None;
        self.kind = Kind::None;
        self.moved = false;
    }
    pub fn new(kind: Kind, color: Color) -> Piece {
        Piece { color, kind, moved: false }
    }
    pub fn mv(&mut self, new: Piece) {
        self.kind = new.kind;
        self.color = new.color;
        self.moved = true;
    }
    pub fn promote(&mut self, new: Kind) -> bool {
        if self.kind == Kind::Pawn {
            self.kind = new;
            return true;
        }
        false
    }
}
