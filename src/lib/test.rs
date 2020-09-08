#[cfg(test)]
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;
#[test]
fn test_position() {
    let mut a = structs::general::Position {
        letter: 3,
        number: 6,
    };
    assert!(a.print() == "D7");
    a = structs::general::Position {
        letter: -1,
        number: 7,
    };
    assert!(a.not_inside());
    a = structs::general::Position {
        letter: 8,
        number: 7,
    };
    assert!(a.not_inside());
    a = structs::general::Position {
        letter: 0,
        number: 7,
    };
    assert!( !a.not_inside()); 
}
