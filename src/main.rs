mod chess;

fn main(){
    let mut game = chess::Gamestate::new();
    game.start();
    let moves = game.all_possible_moves();
    for i in 0..8{
        for j in 0..8{
            print!("{:?}",game.full_name(i, j));
        } 
        println!("");   
    }
    println!("{:?}", game.move_piece(0, 0, 1, 1));
    for i in 0..8{
        for j in 0..8{
            print!("{:?}",game.full_name(i, j));
        } 
        println!("");   
    }
}