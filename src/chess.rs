mod general;
use general::Position;
use general::Piece;
pub struct Game{
    game: Vec<Gamestate>,
    pub current: Gamestate,
    turn: i32
}
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
    pub fn end(self){}
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
        self.current = self.game[self.game.len()-1];
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
}
#[derive(Clone)]
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
    pub fn opponent(&self) -> i32{
        if self.turn == 0 {
            return 1;
        }else{
            return 0;
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
    fn private_get_piece(&mut self, pos: &Position) -> &mut Piece {
        &mut self.board[pos.letter as usize][pos.number as usize]
    }
    fn private_reach_king(&self, position: &Position) -> bool{
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
            return self.pawn_moves(position);
        }
        if self.private_get_piece(position).name == "tower".to_string() {
            return self.tower_moves(position);
        }
        if self.private_get_piece(position).name == "knight".to_string() {
            return self.knight_moves(position);
        }
        if self.private_get_piece(position).name == "horse".to_string() {
            return self.horse_moves(position);
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
        let mut cp = self.private_get_piece(&from).clone();
        self.private_get_piece(&to).mv(cp);
        self.private_get_piece(&from).clear();
        true
    }
}

#[allow(dead_code)]
impl Gamestate{
    fn diagonal_moves(&mut self, pos: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        for d_number in [-1, 1].iter(){
            for d_letter in [-1,1].iter(){
                let mut moving = true;
                let mut cur_position = pos.clone();
                while moving {
                    cur_position.number+=d_number;
                    cur_position.letter+=d_letter;
                    
                    if cur_position.not_inside() {
                        break;
                    }
                    let cur_piece = &self.private_get_piece(&cur_position).clone();
                    if cur_piece.color != 2 {
                        moving = false;
                    }
                    if cur_piece.color != piece.color{
                        ret.push(cur_position.clone());
                    }
                }
            }
        }
        ret
    }
    fn horse_moves(&mut self, pos: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        for delta_number in [-1, 1].iter() {
            for delta_letter in [-1, 1].iter() {
                {
                    let cur_position = general::Position {
                        number: (2*delta_number + pos.number),
                        letter: (delta_letter + pos.letter),
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
                        number: (delta_number + pos.number),
                        letter: (2*delta_letter + pos.letter),
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
    fn king_moves(&mut self, pos: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        for delta_number in [-1,0, 1].iter() {
            for delta_letter in [-1,0, 1].iter() {
                let cur_position = general::Position {
                    number: (delta_number + pos.number),
                    letter: (delta_letter + pos.letter),
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
    fn pawn_moves(&mut self, pos: &Position) -> Vec<Position>{
        if self.private_get_piece(&pos).color == 0{
            return self.white_pawn(pos);
        }else{
            return self.black_pawn(pos);
        }
        
    }
    fn white_pawn(&mut self, pos: &Position)-> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        {
            let cur_position = general::Position {
                number: (pos.number),
                letter: (pos.letter+1),
            };
            if cur_position.inside() {
                
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color == 2 {
                    ret.push(cur_position);
                }
            }    
        }
        if piece.moved == false{
            let cur_position = general::Position {
                number: (pos.number),
                letter: (pos.letter+2),
            };
            if cur_position.inside() {
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color == 2 {
                    ret.push(cur_position);
                }
            }    
        }
        {
           let cur_position = general::Position {
               number: (pos.number+1),
               letter: (pos.letter+1),
           };
           if cur_position.inside() {
               let cur_piece = &self.private_get_piece(&cur_position).clone();
               if cur_piece.color != 2 && cur_piece.color != piece.color{
                   ret.push(cur_position);
               }
           }    
        }
        {
           let cur_position = general::Position {
               number: (pos.number-1),
               letter: (pos.letter+1),
           };
           if cur_position.inside() {
               let cur_piece = &self.private_get_piece(&cur_position).clone();
               if cur_piece.color != 2 && cur_piece.color != piece.color{
                   ret.push(cur_position);
               }
           }    
        }
        ret
    }
    fn black_pawn(&mut self, pos: &Position)-> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        {
            let cur_position = general::Position {
                number: (pos.number),
                letter: (pos.letter-1),
            };
            if cur_position.inside() {
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color == 2 {
                    ret.push(cur_position);
                }
            }    
        }
        if !piece.moved{
            let cur_position = general::Position {
                number: (pos.number),
                letter: (pos.letter-2),
            };
            if cur_position.inside() {
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color == 2 {
                    ret.push(cur_position);
                }
            }    
        }
        {
            let cur_position = general::Position {
                number: (pos.number+1),
                letter: (pos.letter-1),
            };
            if cur_position.inside() {
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color != 2 && cur_piece.color != piece.color{
                    ret.push(cur_position);
                }
            }    
        }
        {
            let cur_position = general::Position {
                number: (pos.number-1),
                letter: (pos.letter-1),
            };
            if cur_position.inside() {
                let cur_piece = &self.private_get_piece(&cur_position).clone();
                if cur_piece.color != 2  && cur_piece.color != piece.color{
                    ret.push(cur_position);
                }
            }    
        }
        ret
    }
    fn straight_moves(&mut self, pos: &Position) -> Vec<Position>{
        let piece = &self.private_get_piece(&pos).clone();
        let mut ret = vec![];
        for d_number in [-1, 1,0].iter(){
            for d_letter in [-1,1,0].iter(){
                if d_number*d_letter != 0{
                    continue;
                }
                let mut moving = true;
                let mut cur_position = pos.clone();
                while moving {
                    cur_position.number+=d_number;
                    cur_position.letter+=d_letter;
                    
                    if cur_position.not_inside() {
                        break;
                    }
                    let cur_piece = &self.private_get_piece(&cur_position).clone();
                    if cur_piece.color != 2 {
                        moving = false;
                    }
                    if cur_piece.color != piece.color{
                        ret.push(cur_position.clone());
                    }
                }
            }
        }
        ret
    }
    fn queen_moves(&mut self, pos: &Position) -> Vec<Position>{
        let mut ret = vec![];
        ret.extend(self.diagonal_moves(pos).iter().copied());
        ret.extend(self.straight_moves(pos).iter().copied());
        ret
    }
    fn knight_moves(&mut self, pos: &Position) -> Vec<Position>{
        let mut ret = vec![];
        ret.extend(self.diagonal_moves(pos).iter().copied());
        ret
    }
    fn tower_moves(&mut self, pos: &Position) -> Vec<Position>{
        let mut ret = vec![];
        ret.extend(self.straight_moves(pos).iter().copied());
        ret
    }
}

