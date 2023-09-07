#[allow(dead_code)]
mod piece;
mod chess;
use crate::chess::*;
fn main() {
    let mut game = Chess::new();
    game.play();
}
