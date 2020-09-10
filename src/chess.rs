//mod moves;
mod general;
use general::Position;
use general::Piece;
//use std::mem;
pub struct Gamestate {
    pub board: Vec<Vec<Piece>>,
    pub turn: i32,
}
#[allow(dead_code)]
impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            board: vec![vec![Piece::new("empty".to_string(), 2); 8]; 8],
            turn: 0,
        }
    }
    pub fn start(&mut self) -> () {
        self.board[0][0] = Piece::new("tower".to_string(), 0);
        self.board[0][1] = Piece::new("horse".to_string(), 0);
        self.board[0][2] = Piece::new("knight".to_string(), 0);
        self.board[0][3] = Piece::new("queen".to_string(), 0);
        self.board[0][4] = Piece::new("king".to_string(), 0);
        self.board[0][5] = Piece::new("knight".to_string(), 0);
        self.board[0][6] = Piece::new("horse".to_string(), 0);
        self.board[0][7] = Piece::new("tower".to_string(), 0);
        self.board[7][0] = Piece::new("tower".to_string(), 1);
        self.board[7][1] = Piece::new("horse".to_string(), 1);
        self.board[7][2] = Piece::new("knight".to_string(), 1);
        self.board[7][3] = Piece::new("queen".to_string(), 1);
        self.board[7][4] = Piece::new("king".to_string(), 1);
        self.board[7][5] = Piece::new("knight".to_string(), 1);
        self.board[7][6] = Piece::new("horse".to_string(), 1);
        self.board[7][7] = Piece::new("tower".to_string(), 1);
        self.board[1][0] = Piece::new("pawn".to_string(), 0);
        self.board[1][1] = Piece::new("pawn".to_string(), 0);
        self.board[1][2] = Piece::new("pawn".to_string(), 0);
        self.board[1][3] = Piece::new("pawn".to_string(), 0);
        self.board[1][4] = Piece::new("pawn".to_string(), 0);
        self.board[1][5] = Piece::new("pawn".to_string(), 0);
        self.board[1][6] = Piece::new("pawn".to_string(), 0);
        self.board[1][7] = Piece::new("pawn".to_string(), 0);
        self.board[6][0] = Piece::new("pawn".to_string(), 1);
        self.board[6][1] = Piece::new("pawn".to_string(), 1);
        self.board[6][2] = Piece::new("pawn".to_string(), 1);
        self.board[6][3] = Piece::new("pawn".to_string(), 1);
        self.board[6][4] = Piece::new("pawn".to_string(), 1);
        self.board[6][5] = Piece::new("pawn".to_string(), 1);
        self.board[6][6] = Piece::new("pawn".to_string(), 1);
        self.board[6][7] = Piece::new("pawn".to_string(), 1);
    }

    pub fn color(&mut self, letter: i32, number: i32) -> String {
        let pos = Position { letter, number };
        if pos.not_inside() {
            panic!("bad position");
        }
        Gamestate::private_get_piece(self, &pos).color()
    }

    pub fn name(&mut self, letter: i32, number: i32) -> String {
        let pos = Position { letter, number };
        if pos.not_inside() {
            panic!("bad position");
        }
        Gamestate::private_get_piece(self, &pos).name()
    }
    pub fn full_name(&mut self, letter: i32, number: i32) -> String {
        let pos = Position{ letter, number };
        if pos.not_inside() {
            panic!("bad position");
        }
        Gamestate::private_get_piece(self, &pos).full_name()
    }

    pub fn possible_moves(&mut self, letter: i32, number: i32) -> Vec<String> {
        let position = Position{
            letter: letter,
            number: number
        };
        if position.not_inside(){
            panic!("bad position");
        }
        let possible = self.private_calc_move(&position);
        let mut ret:Vec<String> = vec![];
        for p in possible{
            ret.push(p.print());
        }
        ret
    }
    pub fn all_possible_moves(&mut self) -> Vec<(String, Vec<String>)> {
        let mut ret = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let full_name = self.board[i][j].full_name();
                let possible = self.private_calc_move(
                    &Position{
                        letter: i as i32,
                        number: j as i32
                    }
                );
                if possible.len() == 0{
                    continue;
                }
                let mut s:(String, Vec<String>) = (full_name, vec![]);
                for p in possible{
                    s.1.push(p.print());
                }
                ret.push(s);
            }
        }
        ret
    }

    pub fn move_piece(&mut self, from_letter: i32, from_number: i32, to_letter: i32, to_number: i32)->bool {
        let from = Position {
            letter: from_letter,
            number: from_number,
        };
        let to = Position {
            letter: to_letter,
            number: to_number,
        };
        if from.not_inside(){
            panic!("bad position")
        }
        if to.not_inside(){
            panic!("bad position")
        }
        self.private_move_piece(
            Position {
                letter: from_letter,
                number: from_number,
            },
            Position {
                letter: to_letter,
                number: to_number,
            },
        )
    }
}

impl Gamestate {
    fn private_get_piece(&mut self, pos: &Position) -> &mut Piece {
        &mut self.board[pos.letter as usize][pos.number as usize]
    }

    

    fn private_calc_move(&mut self, position: &Position) -> Vec<Position> {
        let player = &self.board[position.letter as usize][position.number as usize];
        if player.name() == "empty".to_string() {
            return vec![];
        }
        if player.color != self.turn{
            return vec![];
        }
        let v:Vec<Position> = vec![Position{
            letter:1,
            number:1
        }]; 
        v
    }
    fn private_is_possible(&mut self, from: &Position, to: &Position) -> bool{
        let possible = self.private_calc_move(&from);
        for i in possible.iter(){
            print!("{},{}", i.print(), to.print());
            if to.same(i){
                return true;
            }
        }
        false
    }

    fn private_move_piece(&mut self, from: Position, to: Position) -> bool{
        if !self.private_is_possible( &from, &to){
            return false;
        }
        let cp = self.private_get_piece(&from).clone();
        self.private_get_piece(&to).mv(cp);
        self.private_get_piece(&from).clear();
        true
    }
}
