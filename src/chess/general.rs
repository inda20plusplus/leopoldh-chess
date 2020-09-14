

#[allow(unused)]
pub struct OneMove{
    type_of_move: String,
    from: Position,
    to: Position,
    optional_from:Position,
    optional_to:Position,
    optional_clear:Position,
}
#[derive(Clone, Copy)]
pub struct Position {
    pub letter: i32,
    pub number: i32,
}
#[allow(dead_code)]
impl Position {
    pub fn val(&self) -> (i32, i32) {
        if self.not_inside() {
            panic!("bad position")
        }
        (self.letter, self.number)
    }
    pub fn new(position: (i32, i32)) -> Position {
        Position{
            letter: position.0,
            number: position.1
        }
    }
    pub fn not_inside(&self) -> bool {
        ((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7))
    }
    pub fn inside(&self) -> bool {
        !(((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7)))
    }
    pub fn same(&self, cmp: &Position) -> bool {
        if self.not_inside() || cmp.not_inside() {
            panic!("bad position");
        }
        (self.number == cmp.number) && (self.letter == cmp.letter)
    }
}

#[derive(Clone)]
pub struct Piece {
    pub color: i32,
    pub name: String,
    pub moved: bool,
}
#[allow(dead_code)]
impl Piece {
    pub fn color(&self) -> String {
        if self.color == 1 {
            return "black".to_string();
        }
        if self.color == 0 {
            return "white".to_string();
        }
        return "empty".to_string();
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn opponent(&self) -> i32 {
        if self.color == 0 {
            return 1;
        }
        if self.color == 1 {
            return 0;
        }
        panic!("error")
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

    pub fn full_name(&self) -> String {
        let mut ret = "".to_owned();
        if self.color == 1 {
            ret.push_str("black ");
        }
        if self.color == 0 {
            ret.push_str("white ");
        }
        ret.push_str(Box::leak(self.name().into_boxed_str()));
        ret
    }
    pub fn mv(&mut self, new: Piece){
        self.name = new.name;
        self.color = new.color;
        self.moved = true;
    }
}
