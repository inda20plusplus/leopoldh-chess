#[derive(Debug)]


pub struct PlayerType<'a> {
    pub name: &'a str,
    pub moves: Vec<(i16,i16)>,
}
pub struct Position{
    pub letter: char,
    pub number: i16,
}
pub struct Player<'a>{
    pub player_type: PlayerType<'a> ,
    pub position: Position,
    pub team: bool,
    pub moved_last: bool,
    pub moved: bool,
    pub movements: Vec<(char,i16)>
}