mod chess;

fn main(){
    let mut game = chess::Game::new();
    let mut a = game.print();
    for i in a.iter(){
        for j in i.iter(){
            print!(" {} ",&j);
        }
        println!("");
    }
    game.do_move((0,1), (2,0));
    a = game.print();
    for i in a.iter(){
        for j in i.iter(){
            print!(" {} ",&j);
        }
        println!("");
    }
    game.undo();
    a = game.print();
    for i in a.iter(){
        for j in i.iter(){
            print!(" {} ",&j);
        }
        println!("");
    }
}