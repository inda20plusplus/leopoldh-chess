mod general;
use general::Position;
use general::Piece;
pub struct Game{
    game: Vec<Gamestate>,
    current: Gamestate,
    turn: i32
}
#[allow(unused)]
impl Game {
    pub fn new() -> Game{   
        let mut ret = Game{
            game: vec![Gamestate::new()],
            current: Gamestate::new(),
            turn: 1,
        };
        ret.game[0].start();
        ret.current = ret.game[0].clone();
        ret
    }
    pub fn print(&mut self) -> Vec<Vec<String>>{
        let mut ret: Vec<Vec<String>> = vec![vec!["_".to_owned(); 8];8];
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i as i32, j as i32));
                let piece = self.current.private_get_piece(&position);
                if piece.color == 0{
                    if piece.name == "king".to_string() {
                        ret[i][j] = "♔".to_owned();
                    }
                    if piece.name == "queen".to_string() {
                        ret[i][j] = "♕".to_owned();
                    }
                    if piece.name == "pawn".to_string() {
                        ret[i][j] = "♙".to_owned();
                    }
                    if piece.name == "rook".to_string() {
                        ret[i][j] = "♖".to_owned();
                    }
                    if piece.name == "bishop".to_string() {
                        ret[i][j] = "♗".to_owned();
                    }
                    if piece.name == "knight".to_string() {
                        ret[i][j] = "♘".to_owned();
                    }
                }
                if piece.color == 1{
                    if piece.name == "king".to_string() {
                        ret[i][j] = "♚".to_owned();
                    }
                    if piece.name == "queen".to_string() {
                        ret[i][j] = "♛".to_owned();
                    }
                    if piece.name == "pawn".to_string() {
                        ret[i][j] = "♟".to_owned();
                    }
                    if piece.name == "rook".to_string() {
                        ret[i][j] = "♜".to_owned();
                    }
                    if piece.name == "bishop".to_string() {
                        ret[i][j] = "♝".to_owned();
                    }
                    if piece.name == "knight".to_string() {
                        ret[i][j] = "♞".to_owned();
                    }
                }
            }
        }
        ret
    }
    pub fn possible_moves(&mut self, position: (i32,i32)) -> Vec<(i32, i32)>{
        let ret = self.current.possible_moves(position);
        ret
    }
    pub fn undo(&mut self) -> bool{
        if self.turn < 2 {
            return false;
        }
        self.game.pop();
        self.turn= self.game.len() as i32;
        self.current = self.game[self.game.len()-1].clone();
        true
    }
    pub fn do_move(&mut self, from: (i32, i32), to: (i32, i32)) -> bool{
        if self.current.move_piece(from, to){
            self.game.push(self.current.clone());
            if self.current.check() == true {
                self.undo();
                return false;
            }
            return true;
        }
        false
    }
    pub fn checkmate(&mut self) -> bool{
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i as i32, j as i32));
                let moves = self.possible_moves(position.val());
                for a in moves.iter(){
                    if self.do_move(position.val(), a.clone()){
                        return false
                    }
                }
            }
        }
        true
    }
}
#[derive(Clone)]
pub struct Gamestate {
    pub board: Vec<Vec<Piece>>,
    pub turn: i32,
    pub enpassant: (bool, Position),
    pub check: bool
}
#[allow(dead_code)]
impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            board: vec![vec![Piece::new("empty".to_string(), 2); 8]; 8],
            turn: 0,
            enpassant: (false,Position::new((0,0))),
            check: false
        }
    }
    pub fn opponent(&self) -> i32{
        if self.turn == 0 {
            return 1;
        }else{
            return 0;
        }
    }
    pub fn start(&mut self) -> () {
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
    }

    pub fn color(&mut self, letter: i32, number: i32) -> String {
        let pos = Position { letter, number };
        if pos.not_inside() {
            panic!("bad position");
        }
        Gamestate::private_get_piece(self, &pos).color()
    }
    pub fn moved(&mut self, letter: i32, number: i32) -> bool {
        let pos = Position { letter, number };
        if pos.not_inside() {
            panic!("bad position");
        }
        Gamestate::private_get_piece(self, &pos).moved
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

    pub fn possible_moves(&mut self, position: (i32, i32)) -> Vec<(i32, i32)> {
        let position : Position = Position::new(position);
        if position.not_inside(){
            panic!("bad position");
        }
        let possible : Vec<Position>= self.private_calc_move(&position);
        let mut ret:Vec<(i32, i32)> = vec![];
        for p in possible{
            ret.push(p.val());
        }
        ret
    }
    pub fn check(&mut self) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                let position = Position::new((i as i32, j as i32));
                if self.private_reach_king(&position) {
                    return true;
                };
            }
        }
        false
    }
    //pub fn promote(&mut self, position: (i32, i32), transform: String)-> bool{
    //    if self.can_promote(position) && self.valid_type(&transform){
    //        let mut piece = self.private_get_piece(&Position::new(position));
    //        piece.name = transform;
    //        return true;
    //    }
    //    false
    //}

    //pub fn can_promote(&mut self, position: (i32, i32))-> bool{
    //    let mut is_pawn = false;
    //    {
    //        let piece = self.private_get_piece(&Position::new(position));
    //        is_pawn = piece.name== "pawn".to_string();
    //    }
    //    if self.turn == 0{
    //        if position.0 == 7 && is_pawn{
    //            return true;
    //        }
    //    }else{
    //        if position.0 == 0 && is_pawn{
    //            return true;
    //        }
    //    }
    //    return false;
    //}
    pub fn move_piece(&mut self, from: (i32, i32), to: (i32,i32))->bool {
        let from = Position::new(from);
        let to = Position::new(to);
        if from.not_inside() || to.not_inside(){
            panic!("bad position")
        }
        if !self.private_move_piece(from, to){
            return false;
        }
        self.turn = self.opponent();
        true
    }
}

impl Gamestate {
    //fn valid_type(&mut self, test: &String) -> bool{
    //    let valid = vec!["king".to_string(),"queen".to_string(),"pawn".to_string(),"rook".to_string(),"bishop".to_string(),"knight".to_string()];
    //    for i in valid.iter(){
    //        if i == test{
    //            return true;
    //        }
    //    }
    //    false
    //}
    fn private_get_piece(&mut self, position: &Position) -> &mut Piece {
        &mut self.board[position.letter as usize][position.number as usize]
    }
    fn private_reach_king(&mut self, position: &Position) -> bool{
        let moves = self.private_calc_move(&position);
        for i in moves.iter(){
            if self.private_get_piece(&i).name() == "king".to_owned() {
                return true;
            }
        }
        false
    }
    
    fn private_calc_move(&mut self, position: &Position) -> Vec<Position> {
        let player = &self.board[position.letter as usize][position.number as usize];
        if player.color != self.turn{
            return vec![];
        }
        if self.private_get_piece(position).name == "king".to_string() {
            return self.king_moves(position);
        }
        if self.private_get_piece(position).name == "queen".to_string() {
            return self.queen_moves(position);
        }
        if self.private_get_piece(position).name == "pawn".to_string() {
            let ret = self.pawn_moves(position);
            //if self.can_promote(position.val()){
            //    ret.push(position.clone());
            //}
            return ret;
        }
        if self.private_get_piece(position).name == "rook".to_string() {
            return self.rook_moves(position);
        }
        if self.private_get_piece(position).name == "bishop".to_string() {
            return self.bishop_moves(position);
        }
        if self.private_get_piece(position).name == "knight".to_string() {
            return self.knight_moves(position);
        }
        panic!("bad piece");
    }
    fn private_is_possible(&mut self, from: &Position, to: &Position) -> bool{
        let possible = self.private_calc_move(&from);
        for i in possible.iter(){
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
        if cp.name == "pawn".to_string() && (from.letter-to.letter).abs() == 2{
            self.enpassant = (true, to);
        }else{
            self.enpassant.0 = false;
        }
        self.private_get_piece(&to).mv(cp);
        self.private_get_piece(&from).clear();
        true
    }
}

#[allow(dead_code)]
impl Gamestate{
    
    fn knight_moves(&mut self, position: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&position).clone();
        let mut ret = vec![];
        for delta_number in [-1, 1].iter() {
            for delta_letter in [-1, 1].iter() {
                {
                    let cur_position = general::Position {
                        number: (2*delta_number + position.number),
                        letter: (delta_letter + position.letter),
                    };
                    if cur_position.inside() {
                        let cur_piece = &self.private_get_piece(&cur_position).clone();
                        if cur_piece.color != piece.color {
                            ret.push(cur_position);
                        }
                    }
                    
                }
                {
                    let cur_position = general::Position {
                        number: (delta_number + position.number),
                        letter: (2*delta_letter + position.letter),
                    };
                    if cur_position.inside() {
                        let cur_piece = &self.private_get_piece(&cur_position).clone();
                        if cur_piece.color != piece.color {
                            ret.push(cur_position);
                        }
                    }
                }
            }
        }
        ret
    }
    fn king_moves(&mut self, position: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&position).clone();
        let mut ret = vec![];
        for delta_number in [-1,0, 1].iter() {
            for delta_letter in [-1,0, 1].iter() {
                let cur_position = general::Position {
                    number: (delta_number + position.number),
                    letter: (delta_letter + position.letter),
                };
                if cur_position.inside() {
                    let cur_piece = &self.private_get_piece(&cur_position).clone();
                    if cur_piece.color != piece.color {
                        ret.push(cur_position);
                    }
                }         
            }
        }
        ret
    }
    fn pawn_moves(&mut self, position: &Position) -> Vec<Position>{
        if self.private_get_piece(&position).color == 0{
            return self.pawn(position, 1);
        }else{
            return self.pawn(position, -1);
        }
        
    }
    fn pawn(&mut self, position: &Position, dir: i32)-> Vec<Position>{
        let piece = self.private_get_piece(position).clone();
        let mut ret = vec![];
        for i in 1..3{
            let cur_position = Position {
                number: (position.number),
                letter: (position.letter+i*dir),
            };
            if cur_position.inside() {
                if self.private_get_piece(&cur_position).color == 2 {
                    ret.push(cur_position);
                }
            }    
        }
        for i in [-1, 1].iter(){
            let cur_position = Position::new((position.number+i,position.letter+dir));
            if cur_position.inside() {
                let cur_piece = self.private_get_piece(&cur_position).clone();
                if cur_piece.color == piece.opponent(){
                    ret.push(cur_position);
                }
                if self.enpassant.0 == true{
                    if self.enpassant.1.same(&Position::new((position.number+i,position.letter))){
                        ret.push(cur_position);
                    }
                }
            }    
        }
        ret
    }
    
    fn moves(&mut self, position: &Position, dir: (i32, i32), moving: bool) -> Vec<Position>{
        let piece = &self.private_get_piece(&position).clone();
        let mut ret = vec![];
        let mut cur_position = position.clone();
        while moving {
            cur_position.number+=dir.0;
            cur_position.letter+=dir.1;
            if cur_position.not_inside() {
                break;
            }
            let cur_piece = &self.private_get_piece(&cur_position).clone();
            if cur_piece.color != 2 {
                moving = false;
            }
            if cur_piece.color == piece.opponent(){
                ret.push(cur_position.clone());
            }
        }
        ret
    }
    fn queen_moves(&mut self, position: &Position) -> Vec<Position>{
        let mut ret = vec![];
        for i in [-1, 0, 1].iter(){
            for j in [-1, 0, 1].iter(){
                ret.extend(self.moves(position, (*i, *j), true).iter().copied());
            }
        }
        ret
    }
    fn bishop_moves(&mut self, position: &Position) -> Vec<Position>{
        let mut ret = vec![];
        for i in [-1, 1].iter(){
            for j in [-1, 1].iter(){
                ret.extend(self.moves(position, (*i, *j), true).iter().copied());
            }
        }
        ret
    }
    fn rook_moves(&mut self, position: &Position) -> Vec<Position>{
        let mut ret = vec![];
        for i in [-1, 0, 1].iter(){
            for j in [-1, 0, 1].iter(){
                if i*j == 0{
                    continue;
                }
                ret.extend(self.moves(position, (*i, *j)).iter().copied());
            }
        }
        ret
    }
}

