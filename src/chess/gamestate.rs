use super::piece::Piece;
use super::position::Position;
#[derive(Clone)]
pub struct Gamestate {
    pub board: Vec<Vec<Piece>>,
    pub turn: i32,
    pub enpassant: (bool, Position),
}
#[allow(dead_code)]
impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            board: vec![vec![Piece::new("empty".to_string(), 2); 8]; 8],
            turn: 0,
            enpassant: (false, Position::new((0, 0)))
        }
    }
    pub fn opponent(&self) -> i32 {
        match self.turn {
            0 => return 1,
            1 => return 0,
            _ => panic!("error turn")
        }
    }
    pub fn next(&mut self){
        match self.turn {
            0 => self.turn = 1,
            1 => self.turn = 0,
            _ => panic!("error turn")
        }
    }
    pub fn stalemate(&mut self)->bool{
        let can_move:bool = self.can_move();
        if !self.check() && can_move == false{
            self.next();
            return true;
        }
        self.next();
        false
    }
    pub fn checkmate(&mut self)->bool{
        let can_move:bool = self.can_move();
        self.next();
        if self.check() && can_move == false{
            self.next();
            return true;
        }
        self.next();
        false
    }


    pub fn possible_moves(&mut self, position: &Position) -> Vec<(i32, i32)> {
        position.panic();
        let possible: Vec<Position> = self.calc_move(&position);
        let mut ret: Vec<(i32, i32)> = vec![];
        for i in possible.iter(){
            let mut tmp = self.clone();
            if tmp.move_piece(position.clone(), i.clone()){
                tmp.turn = tmp.opponent();
                if tmp.check() == false{
                    ret.push(i.val());
                }
            }
        }
        ret
    }
    pub fn check(&mut self) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i as i32, j as i32));
                if self.reach_king(&position) {
                    return true;
                };
            }
        }
        false
    }
}

impl Gamestate {
    pub fn get_piece(&mut self, position: &Position) -> &mut Piece {
        &mut self.board[position.letter as usize][position.number as usize]
    }

    fn reach_king(&mut self, position: &Position) -> bool {
        let moves = self.calc_move(&position);
        for i in moves.iter() {
            if self.get_piece(&i).name() == "king".to_owned() {
                return true;
            }
        }
        false
    }
    fn move_allowed(&mut self, from: &Position, to: &Position) -> bool {
        let possible = self.possible_moves(&from);
        for i in possible{
            if to.same(&Position::new(i)) {
                return true;
            }
        }
        false
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> bool {
        if !self.move_allowed(&from, &to) {
            return false;
        }
        let cp = self.get_piece(&from).clone();
        
        if self.enpassant.0{
            let mut dir = 1;
            if cp.color() == 1{
                dir = -1;
            }
            if cp.name() == "pawn".to_string()
            &&
            Position::new((to.letter-dir, to.number)).same(&self.enpassant.1) 
            {
                self.get_piece(&self.enpassant.1).clear();
            }
        }

        if cp.name() == "pawn".to_string() && (from.letter - to.letter).abs() == 2 {
            self.enpassant = (true, to);
        } else {
            self.enpassant.0 = false;
        }
        self.get_piece(&to).mv(&cp);
        self.check = self.check();
        self.turn = self.opponent();
        true
    }
}

//moves 
#[allow(dead_code)]
impl Gamestate {
    fn calc_move(&mut self, position: &Position) -> Vec<Position> {
        let player = &self.get_piece(position);
        if player.color() != self.turn {
            return vec![];
        }
        match &self.get_piece(position).name().as_str() {
            &"king" => return self.king_moves(position),
            &"queen" => return self.queen_moves(position),
            &"rook" => return self.rook_moves(position),
            &"knight" => return self.knight_moves(position),
            &"bishop" => return self.bishop_moves(position),
            &"pawn" => return self.pawn_moves(position),
            _ => panic!("bad name"),
        }
    }
    fn knight_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 1].iter() {
            for j in [-1, 1].iter() {
                ret.extend(self.moves(position, (*i * 2, *j), false).iter().copied());
                ret.extend(self.moves(position, (*i, *j * 2), false).iter().copied());
            }
        }
        ret
    }
    fn king_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 0, 1].iter() {
            for j in [-1, 0, 1].iter() {
                ret.extend(self.moves(position, (*i, *j), false).iter().copied());
            }
        }
        ret
    }
    fn pawn_moves(&mut self, position: &Position) -> Vec<Position> {
        match self.get_piece(&position).color() {
            0 => return self.pawn(position, 1),
            1 => return self.pawn(position, -1),
            _ => return vec![],
        }
    }
    fn pawn(&mut self, position: &Position, dir: i32) -> Vec<Position> {
        let piece = self.get_piece(position).clone();
        let mut ret = vec![];
        {
            let cur_position = Position {
                number: (position.number),
                letter: (position.letter + dir),
            };
            if cur_position.inside() {
                if self.get_piece(&cur_position).color() == 2 {
                    ret.push(cur_position);
                    let cur_position1 = Position {
                        number: (position.number),
                        letter: (position.letter + 2 * dir),
                    };
                    if !piece.moved() {
                        if self.get_piece(&cur_position1).color() == 2 {
                            ret.push(cur_position1);
                        }
                    }
                }
            }
        }
        {
            let cur_position1 = Position {
                number: (position.number),
                letter: (position.letter + 2 * dir),
            };
            if cur_position1.inside() {
                if self.get_piece(&cur_position1).color() == 2 {
                    ret.push(cur_position1);
                }
            }
        }
        {
            let cur_position = Position {
                number: (position.number),
                letter: (position.letter + 2 * dir),
            };
            if cur_position.inside() {
                if self.get_piece(&cur_position).color() == 2 {
                    ret.push(cur_position);
                }
            }
        }
        for i in [-1, 1].iter() {
            let cur_position = Position::new((position.number + i, position.letter + dir));
            if cur_position.inside() {
                let cur_piece = self.get_piece(&cur_position).clone();
                if cur_piece.color() == piece.opponent() {
                    ret.push(cur_position);
                }
                if self.enpassant.0 == true {
                    if self
                        .enpassant
                        .1
                        .same(&Position::new((position.number + i, position.letter)))
                    {
                        ret.push(cur_position);
                    }
                }
            }
        }
        ret
    }
    fn moves(&mut self, position: &Position, dir: (i32, i32), inf: bool) -> Vec<Position> {
        let opponent = self.get_piece(&position).opponent();
        let mut ret = vec![];
        let mut cur_position = position.clone();
        loop {
            cur_position.number += dir.0;
            cur_position.letter += dir.1;
            if cur_position.not_inside() {
                break;
            }
            let cur_piece = &self.get_piece(&cur_position).clone();

            if cur_piece.color() == opponent {
                ret.push(cur_position.clone());
            }
            if cur_piece.color() != 2 || !inf {
                break;
            }
        }
        ret
    }
    fn queen_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 0, 1].iter() {
            for j in [-1, 0, 1].iter() {
                ret.extend(self.moves(position, (*i, *j), true).iter().copied());
            }
        }
        ret
    }
    fn bishop_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 1].iter() {
            for j in [-1, 1].iter() {
                ret.extend(self.moves(position, (*i, *j), true).iter().copied());
            }
        }
        ret
    }
    fn rook_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 0, 1].iter() {
            for j in [-1, 0, 1].iter() {
                if i * j == 0 {
                    continue;
                }
                ret.extend(self.moves(position, (*i, *j), true).iter().copied());
            }
        }
        ret
    }
}
//start positions
impl Gamestate{
    pub fn populate(&mut self, gamestyle: String) -> () {
        match gamestyle.as_str() {
            default => {
                self.board[0][0] = Piece::new("rook".to_string(), 0);
                self.board[0][1] = Piece::new("knight".to_string(), 0);
                self.board[0][2] = Piece::new("bishop".to_string(), 0);
                self.board[0][3] = Piece::new("queen".to_string(), 0);
                self.board[0][4] = Piece::new("king".to_string(), 0);
                self.board[0][5] = Piece::new("bishop".to_string(), 0);
                self.board[0][6] = Piece::new("knight".to_string(), 0);
                self.board[0][7] = Piece::new("rook".to_string(), 0);
                self.board[7][0] = Piece::new("rook".to_string(), 1);
                self.board[7][1] = Piece::new("knight".to_string(), 1);
                self.board[7][2] = Piece::new("bishop".to_string(), 1);
                self.board[7][3] = Piece::new("queen".to_string(), 1);
                self.board[7][4] = Piece::new("king".to_string(), 1);
                self.board[7][5] = Piece::new("bishop".to_string(), 1);
                self.board[7][6] = Piece::new("knight".to_string(), 1);
                self.board[7][7] = Piece::new("rook".to_string(), 1);
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
            },
            _ => println!("unknown board")
        }
        
    }
}