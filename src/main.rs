mod structs;
fn main(){
    let a = structs::general::Position{letter: 3, number: 6};
    println!("{}", a.print());
}