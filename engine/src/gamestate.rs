use super::piece::Piece;
use super::position::Position;
use super::Color;
use super::Kind;
#[derive(Clone)]
pub struct Gamestate {
    pub board: Vec<Vec<Piece>>,
    pub turn: Color,
    pub enpassant: (bool, Position),
}
#[allow(dead_code)]
impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            board: vec![vec![Piece::new(Kind::None, Color::None); 8]; 8],
            turn: Color::White,
            enpassant: (false, Position::new((0, 0))),
        }
    }
    pub fn checkmate(&mut self) -> bool {
        let can_move: bool = self.can_move();
        if self.check() && can_move == false {
            return true;
        }
        false
    }
    pub fn stalemate(&mut self) -> bool {
        let can_move: bool = self.can_move();
        if self.check() == false && can_move == false {
            self.next();
            return true;
        }
        self.next();
        false
    }
    pub fn opponent(&self) -> Color {
        match self.turn {
            Color::White => return Color::Black,
            Color::Black => return Color::White,
            _ => panic!("error turn"),
        }
    }
    pub fn next(&mut self) {
        match self.turn {
            Color::White => self.turn = Color::Black,
            Color::Black => self.turn = Color::White,
            _ => panic!("error turn"),
        }
    }
    fn can_move(&mut self) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i.clone() as i32, j.clone() as i32));
                let moves = self.possible_moves(&position);
                if moves.len() > 0 {
                    return true;
                }
            }
        }
        false
    }
    pub fn possible_moves(&mut self, position: &Position) -> Vec<(i32, i32)> {
        position.panic();
        let possible: Vec<Position> = self.calc_move(&position);
        let mut ret: Vec<(i32, i32)> = vec![];
        for i in possible.iter() {
            let mut tmp = self.clone();
            if tmp.move_piece(position.clone(), i.clone(), Kind::Queen) {
                if tmp.check() == false {
                    ret.push(i.val());
                }
            }
        }
        ret
    }
}

impl Gamestate {
    pub fn print(&mut self) -> () {
        let current = self;
        for i in 0..8 {
            for j in 0..8 {
                let piece = current.get_piece(&Position::new((7 - i as i32, j as i32)));
                print!(" {} ", super::icon::icon(piece.color(), piece.kind()));
            }
            println!("");
        }
        println!("+++++++++++++++++++++++++++++")
    }
    pub fn get_piece(&mut self, position: &Position) -> &mut Piece {
        position.panic();
        &mut self.board[position.letter as usize][position.number as usize]
    }
    pub fn get_piece_kind(&mut self, position: &Position) -> Kind {
        position.panic();
        self.board[position.letter as usize][position.number as usize].kind()
    }

    pub fn check(&mut self) -> bool {
        self.next();
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i as i32, j as i32));
                let moves = self.calc_move(&position);
                for i in moves.iter() {
                    if self.get_piece(&i).kind() == Kind::King {
                        self.next();
                        return true;
                    }
                }
            }
        }
        self.next();
        false
    }
    fn move_allowed(&mut self, from: &Position, to: &Position) -> bool {
        let possible = self.calc_move(&from);
        for i in possible {
            if to.same(&i) {
                return true;
            }
        }
        false
    }
    fn was_enpassant(&mut self, from: Position, to: Position) {
        if self.enpassant.0 {
            let mut dir = 1;
            if self.get_piece(&from).color() == Color::Black {
                dir = -1;
            }
            if self.get_piece(&from).kind() == Kind::Pawn && Position::new((to.letter - dir, to.number)).same(&self.enpassant.1) {
                self.get_piece(&Position::new((to.letter - dir, to.number))).clear();
            }
        }
    }
    fn enpassant_next(&mut self, from: Position, to: Position) {
        if self.get_piece(&from).kind() == Kind::Pawn && (from.letter - to.letter).abs() == 2 {
            self.enpassant = (true, to);
        } else {
            self.enpassant.0 = false;
        }
    }
    pub fn move_piece(&mut self, from: Position, to: Position, promote: Kind) -> bool {
        if !self.move_allowed(&from, &to) {
            return false;
        }
        from.panic();
        to.panic();
        self.was_enpassant(from, to);
        self.enpassant_next(from, to);
        let copy = self.get_piece(&from).clone();
        self.get_piece(&to).mv(copy);
        self.get_piece(&from).clear();
        if self.get_piece(&to).kind() == Kind::Pawn {
            if to.letter == 7 || to.letter == 0 {
                match promote {
                    Kind::None => return false,
                    _ => return self.promote(to, promote),
                }
            }
        }
        if self.check() {
            return false;
        }
        true
    }
}
//small castling
#[allow(dead_code)]
impl Gamestate {
    pub fn small_castling_available(&mut self) -> bool {
        if !self.small_castling_legal() {
            return false;
        }
        let mut tmp = self.clone();
        tmp.private_small_castling()
    }
    pub fn small_castling(&mut self) -> bool {
        if !self.small_castling_legal() {
            return false;
        }
        self.private_small_castling()
    }
    fn small_castling_legal(&mut self) -> bool {
        let s = match self.turn {
            Color::Black => 1,
            Color::White => 0,
            _ => panic!("bad color"),
        };
        self.get_piece_kind(&Position { number: 4, letter: 7 * s }) == Kind::King
            && self.get_piece_kind(&Position { number: 5, letter: 7 * s }) == Kind::None
            && self.get_piece_kind(&Position { number: 6, letter: 7 * s }) == Kind::None
            && self.get_piece_kind(&Position { number: 7, letter: 7 * s }) == Kind::Rook
            && self.get_piece(&Position { number: 4, letter: 7 * s }).moved() == false
            && self.get_piece(&Position { number: 7, letter: 7 * s }).moved() == false
            && self.check() == false
    }
    fn private_small_castling(&mut self) -> bool {
        let s = match self.turn {
            Color::Black => 1,
            Color::White => 0,
            _ => panic!("bad color"),
        };
        {
            let from = self.get_piece(&Position { number: 4, letter: 7 * s }).clone();
            self.get_piece(&Position { number: 4, letter: 7 * s }).clear();
            self.get_piece(&Position { number: 6, letter: 7 * s }).mv(from);
        }
        {
            let from = self.get_piece(&Position { number: 7, letter: 7 * s }).clone();
            self.get_piece(&Position { number: 7, letter: 7 * s }).clear();
            self.get_piece(&Position { number: 5, letter: 7 * s }).mv(from);
        }
        self.next();
        if self.check() {
            return false;
        }
        true
    }
}
//large castling
#[allow(dead_code)]
impl Gamestate {
    pub fn large_castling_available(&mut self) -> bool {
        if !self.large_castling_legal() {
            return false;
        }
        let mut tmp = self.clone();
        tmp.private_large_castling()
    }
    pub fn large_castling(&mut self) -> bool {
        if !self.large_castling_legal() {
            return false;
        }
        self.private_large_castling()
    }
    fn large_castling_legal(&mut self) -> bool {
        let s = match self.turn {
            Color::Black => 1,
            Color::White => 0,
            _ => panic!("bad color"),
        };
        self.get_piece_kind(&Position { number: 4, letter: 7 * s }) == Kind::King
            && self.get_piece_kind(&Position { number: 3, letter: 7 * s }) == Kind::None
            && self.get_piece_kind(&Position { number: 2, letter: 7 * s }) == Kind::None
            && self.get_piece_kind(&Position { number: 1, letter: 7 * s }) == Kind::None
            && self.get_piece_kind(&Position { number: 0, letter: 7 * s }) == Kind::Rook
            && self.get_piece(&Position { number: 4, letter: 7 * s }).moved() == false
            && self.get_piece(&Position { number: 0, letter: 7 * s }).moved() == false
            && self.check() == false
    }
    fn private_large_castling(&mut self) -> bool {
        let s = match self.turn {
            Color::Black => 1,
            Color::White => 0,
            _ => panic!("bad color"),
        };
        {
            let from = self.get_piece(&Position { number: 4, letter: 7 * s }).clone();
            self.get_piece(&Position { number: 4, letter: 7 * s }).clear();
            self.get_piece(&Position { number: 2, letter: 7 * s }).mv(from);
        }
        {
            let from = self.get_piece(&Position { number: 0, letter: 7 * s }).clone();
            self.get_piece(&Position { number: 0, letter: 7 * s }).clear();
            self.get_piece(&Position { number: 3, letter: 7 * s }).mv(from);
        }
        if self.check() {
            return false;
        }
        self.next();
        true
    }
}
//moves
#[allow(dead_code)]
impl Gamestate {
    fn promote(&mut self, position: Position, to: Kind) -> bool {
        position.panic();
        if self.check() {
            return false;
        }
        let piece = self.get_piece(&position);
        match to {
            Kind::Queen | Kind::King | Kind::Knight | Kind::Rook => match piece.color() {
                Color::White | Color::Black => {
                    if piece.promote(to) {
                        return true;
                    }
                    return false;
                }
                _ => return false,
            },
            _ => return false,
        }
    }
    fn calc_move(&mut self, position: &Position) -> Vec<Position> {
        let player = &self.get_piece(position);
        if player.color() != self.turn {
            return vec![];
        }
        match &self.get_piece(position).kind() {
            &Kind::King => return self.king_moves(position),
            &Kind::Queen => return self.queen_moves(position),
            &Kind::Rook => return self.rook_moves(position),
            &Kind::Knight => return self.knight_moves(position),
            &Kind::Bishop => return self.bishop_moves(position),
            &Kind::Pawn => return self.pawn_moves(position),
            _ => panic!("bad kind"),
        }
    }
    fn knight_moves(&mut self, position: &Position) -> Vec<Position> {
        let mut ret = vec![];
        for i in [-1, 1].iter() {
            for j in [-1, 1].iter() {
                ret.extend(self.moves(position, (*i * 2, *j), false));
                ret.extend(self.moves(position, (*i, *j * 2), false));
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
            Color::White => return self.pawn(position, 1),
            Color::Black => return self.pawn(position, -1),
            _ => return vec![Position::new((1, 1))],
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
                if self.get_piece(&cur_position).color() == Color::None {
                    ret.push(cur_position);
                    let cur_position1 = Position {
                        number: (position.number),
                        letter: (position.letter + 2 * dir),
                    };
                    if !piece.moved() {
                        if self.get_piece(&cur_position1).color() == Color::None {
                            ret.push(cur_position1);
                        }
                    }
                }
            }
        }
        for i in [-1, 1].iter() {
            let cur_position = Position::new((position.letter + dir, position.number + i));
            if cur_position.inside() {
                let cur_piece = self.get_piece(&cur_position).clone();
                if cur_piece.color() == piece.opponent() {
                    ret.push(cur_position);
                }
                if self.enpassant.0 == true {
                    if self.enpassant.1.same(&Position::new((position.letter, position.number + i))) {
                        ret.push(cur_position);
                    }
                }
            }
        }
        ret
    }
    fn moves(&mut self, position: &Position, dir: (i32, i32), inf: bool) -> Vec<Position> {
        let mut ret = vec![];
        let mut cur_position = position.clone();
        loop {
            cur_position.number += dir.0;
            cur_position.letter += dir.1;
            if cur_position.not_inside() {
                break;
            }
            let cur_piece = &self.get_piece(&cur_position).clone();

            if cur_piece.color() != self.turn {
                ret.push(cur_position.clone());
            }
            if cur_piece.color() != Color::None || !inf {
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
                    ret.extend(self.moves(position, (*i, *j), true).iter().copied());
                }
            }
        }
        ret
    }
}
//start positions
impl Gamestate {
    pub fn populate(&mut self, gamestyle: String) -> () {
        match gamestyle.as_str() {
            "default" => {
                self.board[0][0] = Piece::new(Kind::Rook, Color::White);
                self.board[0][1] = Piece::new(Kind::Knight, Color::White);
                self.board[0][2] = Piece::new(Kind::Bishop, Color::White);
                self.board[0][3] = Piece::new(Kind::Queen, Color::White);
                self.board[0][4] = Piece::new(Kind::King, Color::White);
                self.board[0][5] = Piece::new(Kind::Bishop, Color::White);
                self.board[0][6] = Piece::new(Kind::Knight, Color::White);
                self.board[0][7] = Piece::new(Kind::Rook, Color::White);
                self.board[7][0] = Piece::new(Kind::Rook, Color::Black);
                self.board[7][1] = Piece::new(Kind::Knight, Color::Black);
                self.board[7][2] = Piece::new(Kind::Bishop, Color::Black);
                self.board[7][3] = Piece::new(Kind::Queen, Color::Black);
                self.board[7][4] = Piece::new(Kind::King, Color::Black);
                self.board[7][5] = Piece::new(Kind::Bishop, Color::Black);
                self.board[7][6] = Piece::new(Kind::Knight, Color::Black);
                self.board[7][7] = Piece::new(Kind::Rook, Color::Black);
                self.board[1][0] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][1] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][2] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][3] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][4] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][5] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][6] = Piece::new(Kind::Pawn, Color::White);
                self.board[1][7] = Piece::new(Kind::Pawn, Color::White);
                self.board[6][0] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][1] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][2] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][3] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][4] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][5] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][6] = Piece::new(Kind::Pawn, Color::Black);
                self.board[6][7] = Piece::new(Kind::Pawn, Color::Black);
            }
            _ => panic!("unknown board"),
        }
    }
}
