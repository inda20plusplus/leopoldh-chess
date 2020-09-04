# leopoldh-chess
Rust chess library to build great chess games with.
Following the rules from [Rules of chess](https://en.wikipedia.org/wiki/Rules_of_chess).


## TODO
- [x] implementing game states(encoded in string).
- [ ] creating list of possible moves from gamestate.
- [ ] going to next gamestate(validating against possible moves).
- [ ] check for checkmate.
- [ ] check for check.


## Docs
```
encoded_gamestate = [65 char string <0 or 1 / current turn white or black><then a concatination of the board / _ as empty and char for the players>]

0 = white pawn
1 = white knight
2 = white bishop
3 = white rook
4 = white queen
5 = white king
a = black pawn
b = black knight
c = black bishop
d = black rook
e = black queen
f = black king

start_gamestate =
             white  a b c d e f g h
                  1 3 1 2 4 5 2 1 3
                  2 0 0 0 0 0 0 0 0 
                  3 _ _ _ _ _ _ _ _
                  4 _ _ _ _ _ _ _ _
                  5 _ _ _ _ _ _ _ _
                  6 _ _ _ _ _ _ _ _
                  7 a a a a a a a a
                  8 d b c e f c b d

encoded_start_gamestate = '03124521300000000________________________________aaaaaaaadbcefcbd'
```