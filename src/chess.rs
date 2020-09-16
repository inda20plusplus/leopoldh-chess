mod gamestate;
mod piece;
mod position;
mod icon;
use gamestate::Gamestate;
use position::Position;
pub struct Game {
    game: Vec<Gamestate>
}
#[allow(unused)]
//game commands
impl Game {
    pub fn turn(&self)->i32{
        self.game.len() as i32
    }
    pub fn current(&mut self)->Gamestate{
        self.game[self.game.len()-1].clone()
    }
    pub fn new() -> Game {
        let mut ret = Game {
            game: vec![Gamestate::new()]
        };
        ret.default();
        ret
    }
    fn default(&mut self)-> (){
        if self.turn() ==  1{
            self.game[0].populate("default".to_owned());
        }else{
            panic!("reset while game is ongoing");
        }
    }
    pub fn special(&mut self, val: String)-> (){
        if self.turn() ==  1{
            self.game[0].populate("default".to_owned());
        }else{
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
        return  false;
    }
    pub fn promote(&mut self, position: (i32, i32), to: String) -> bool {
        let mut round = self.current();
        let mut piece = round.get_piece(&Position::new(position));
        if round.check{
            return false;
        }
        match to.as_str() {
            "queen" | "bishop" | "knight" | "rook" => 
                match piece.color() {
                    0 | 1 if position.0 == 7 || position.0 == 0 => {
                        if piece.promote(to){
                            self.game.push(round);
                            return true;
                        }
                    } ,
                    _ => return false,
                },
            _ => return false,    
        }
        false
    }
    pub fn small_castling(&mut self) -> bool{
        if !self.small_castling_available(){
            return false;
        }
        false
    }
    pub fn large_castling(&mut self) -> bool{
        if !self.large_castling_available(){
            return false;
        }
        false
    }
    fn valid_move(&mut self) -> bool {
        if self.large_castling_available(){
            return true;
        }
        if self.small_castling_available(){
            return true;
        }
        for i in 0..8 {
            if self.promote((i as i32, 0), "queen".to_string()){
                self.undo();
                return true;
            }
            for j in 0..8 {
                if self.current().calc_moves(&Position::new((i, j))).len() != 0{
                    true;
                }
            }
        }
        false
    }
}
//debug and information
impl Game {
    pub fn print(&mut self) -> (){
        for i in 0..8 {
            for j in 0..8 {
                let piece = self.current().get_piece(&Position::new((i as i32, j as i32)));
                print!(" {} ", icon::icon(piece.color(), piece.name()));
            }
            println!("");
        }
    }
    pub fn possible_moves(&mut self, position: (i32, i32)) -> Vec<(i32, i32)> {
        let ret = self.current().possible_moves(&Position::new(position));
        ret
    }
    pub fn small_castling_available(&mut self) -> bool{
        false
    }
    pub fn large_castling_available(&mut self) -> bool{
        false
    }
    pub fn stalemate(&mut self) -> bool {
        if self.current().check() == false && self.valid_move() == false {
            return true;
        }
        false
    }
    pub fn check(&self)->bool{
        self.current.check
    }
}
