#[derive(Clone, Copy)]
pub struct Position {
    pub letter: i32,
    pub number: i32,
}
#[allow(dead_code)]
impl Position {
    pub fn val(&self) -> (i32, i32) {
        (self.letter, self.number)
    }
    pub fn new(position: (i32, i32)) -> Position {
        Position {
            letter: position.0,
            number: position.1,
        }
    }
    pub fn not_inside(&self) -> bool {
        ((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7))
    }
    pub fn inside(&self) -> bool {
        !(((self.number < 0) || (self.letter < 0)) || ((self.number > 7) || (self.letter > 7)))
    }
    pub fn same(&self, cmp: &Position) -> bool {
        self.panic();cmp.panic();
        (self.number == cmp.number) && (self.letter == cmp.letter)
    }
    pub fn panic(&self) -> (){
        if self.not_inside(){
            panic!("bad position");
        }
    }
}
