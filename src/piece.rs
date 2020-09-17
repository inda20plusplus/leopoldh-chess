#[derive(Clone)]
pub struct Piece {
    color: i32,
    name: String,
    moved: bool,
}
#[allow(dead_code)]
impl Piece {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn color(&self) -> i32 {
        self.color
    }
    pub fn moved(&self) -> bool {
        self.moved
    }
    pub fn change_name(&mut self, new: String) -> () {
        self.name = new;
    }
    pub fn opponent(&self) -> i32 {
        match self.color {
            1 => 0,
            0 => 1,
            _ => panic!("bad color"),
        }
    }
    pub fn clear(&mut self) {
        self.color = 2;
        self.name = "empty".to_string();
        self.moved = false;
    }
    pub fn new(name: String, color: i32) -> Piece {
        Piece {
            color,
            name,
            moved: false,
        }
    }
    pub fn mv(&mut self, new: Piece) {
        self.name = new.name;
        self.color = new.color;
        self.moved = true;
    }
    pub fn promote(&mut self, new: String) -> bool {
        if self.name == "pawn".to_owned() {
            self.name = new;
            return true;
        }
        false
    }
}
