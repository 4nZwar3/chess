use crate::piece::*;
use std::io::{self, Write};
pub struct Chess {
    pieces: Vec<Piece>,
    turn_count: usize,
    turn: Team,
    selected: Option<usize>,
    marked: Option<usize>,
}
impl Chess {
    pub fn new() -> Chess {
        Chess {
            pieces: Vec::new(),
            turn_count: 0,
            turn: Team::White,
            selected: None,
            marked: None,
        }
    }
    pub fn init(&mut self) {
        for i in 0..2 {
            let mut ekis = 0;
            let mut ekisde = 1;
            let mut rei = 3;
            let mut quin = 4;
            let mut equip = Team::White;
            if i == 1 {
                ekis = 7;
                ekisde = 6;
                rei = 4;
                quin = 3;
                equip = Team::Black;
            }
            self.pieces
                .push(Piece::new(ekis, 0, Some(Type::Tower), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, 1, Some(Type::Knight), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, 2, Some(Type::Bishop), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, rei, Some(Type::King), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, quin, Some(Type::Queen), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, 5, Some(Type::Bishop), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, 6, Some(Type::Knight), Some(equip)));
            self.pieces
                .push(Piece::new(ekis, 7, Some(Type::Tower), Some(equip)));
            for j in 0..8 {
                self.pieces
                    .push(Piece::new(ekisde, j, Some(Type::Pawn), Some(equip)));
            }
        }
    }
    pub fn play(&mut self) {
        self.init();
        loop {
            self.unsel_unmark();
            self.print();
            self.do_turn();
        }
    }
    fn unsel_unmark(&mut self) {
        self.selected = None;
        self.marked = None;
    }
    pub fn print(&self) {
        let mut table: [[i32; 8]; 8] = [
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7, 7, 7],
        ];
        for piece in self.pieces.clone().into_iter() {
            let mut t = match piece.get_type() {
                Some(n) => n as i32,
                _ => 7,
            };
            if piece.get_team() != Some(self.turn) {
                t *= -1;
            }
            table[piece.get_pos().x][piece.get_pos().y] = t;
        }
        println!("\n   - - - - - - - - - - - - - - - - -");
        for i in 0..8 {
            print!(" {} ", i + 1);
            for j in 0..8 {
                print!("|");
                let mut element = match table[i][j] {
                    1 | -1 => 'I',
                    2 | -2 => 'T',
                    3 | -3 => 'H',
                    4 | -4 => 'B',
                    5 | -5 => 'Q',
                    6 | -6 => 'K',
                    _ => ' ',
                };
                if table[i][j] < 0 {
                    element = element.to_ascii_lowercase();
                }
                if self.selected.is_some()
                    && i == self.pieces[self.selected.unwrap()].get_pos().x
                    && j == self.pieces[self.selected.unwrap()].get_pos().y
                {
                    print!("[{element}]");
                } else if self.marked.is_some()
                    && i == self.pieces[self.marked.unwrap()].get_pos().x
                    && j == self.pieces[self.marked.unwrap()].get_pos().y
                {
                    print!("<{element}>");
                } else {
                    print!(" {element} ");
                }
            }
            println!("|\n   - - - - - - - - - - - - - - - - -");
        }
        println!(
            "Turn number {} - {}",
            self.turn_count,
            match self.turn {
                Team::White => "Whites",
                _ => "Blacks",
            }
        )
    }
    fn do_turn(&mut self) {
        print!("Insert piece to move: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let un = buf.chars().nth(0).unwrap();
        let dos = buf.chars().nth(1).unwrap();
        if self.select(un, dos) == false {
            return;
        }
        Self::print(&self);
        print!("Insert position to move to: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let un = buf.chars().nth(0).unwrap();
        let dos = buf.chars().nth(1).unwrap();
        if self.mark(un, dos) == false {
            return;
        }
        Self::print(&self);
        print!("Are you sure you want to make the move? [Y/n]: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.chars().nth(0).unwrap().to_ascii_lowercase() {
            'n' => return,
            _ => self.make_turn(),
        };
    }
    fn make_turn(&mut self) {
        if self.selected.is_none() {
            println!("There is no piece to move.");
            return;
        }
        if self.pieces[self.selected.unwrap()].get_pos()
            == self.pieces[self.marked.unwrap()].get_pos()
        {
            println!("Both positions are the same.");
            return;
        }
        let is_turn_valid = match self.pieces[self.selected.unwrap()].get_type() {
            Some(Type::Pawn) => self.pawn_move(),
            _ => false,
        };
        if is_turn_valid == false {
            println!("Move is not valid.");
            return;
        }
        self.do_move();
    }
    fn do_move(&mut self) {
        let new_x = self.pieces[self.marked.unwrap()].get_pos().x;
        let new_y = self.pieces[self.marked.unwrap()].get_pos().y;
        self.pieces[self.selected.unwrap()].set_pos(new_x, new_y);
        self.pieces[self.selected.unwrap()].inc_use();
        self.pieces.remove(self.marked.unwrap());
        self.turn_count += 1;
        self.switch_team();
    }
    fn pawn_move(&self) -> bool {
        if ((
                self.pieces[self.selected.unwrap()].get_pos().y == self.pieces[self.marked.unwrap()].get_pos().y    //if the vertical position is the same
                && (self.pieces[self.selected.unwrap()]
                    .get_pos()
                    .x
                    .abs_diff(self.pieces[self.marked.unwrap()].get_pos().x)
                    == 1    //and the difference of the horizontal positions is equal to 1
                    || self.pieces[self.selected.unwrap()]
                        .get_pos()
                        .x
                        .abs_diff(self.pieces[self.marked.unwrap()].get_pos().x)
                        == 2
                    && self.pieces[self.selected.unwrap()].get_uses() == 0  //or the difference of the horizontal positions is equal to 2 and selected has never been used
                )
            )
            && self.pieces[self.marked.unwrap()].get_team() == None //and marked is empty
            || self.pieces[self.selected.unwrap()]
                .get_pos()
                .y
                .abs_diff(self.pieces[self.marked.unwrap()].get_pos().y)
                == 1 //or the difference of the vertical positions is equal to 1
            && self.pieces[self.selected.unwrap()]
                .get_pos()
                .x
                .abs_diff(self.pieces[self.marked.unwrap()].get_pos().x)
                == 1    //and the difference of the horizontal positions is equal to 1
            && self.pieces[self.selected.unwrap()].same_team(self.pieces[self.marked.unwrap()]) == false    //and selected and marked are not from the same team
            && (self.pieces[self.selected.unwrap()].is_white()
                && self.pieces[self.selected.unwrap()].get_pos().x < self.pieces[self.marked.unwrap()].get_pos().x
                || self.pieces[self.selected.unwrap()].is_black()
                && self.pieces[self.selected.unwrap()].get_pos().x > self.pieces[self.marked.unwrap()].get_pos().x //and marked is forward for selected
            ))
            && self.pieces[self.selected.unwrap()].get_team() == Some(self.turn)
        {
            return true;
        }
        false
    }
    fn select(&mut self, une: char, deux: char) -> bool {
        let ex = Self::char_to_pos(une);
        let yi = Self::char_to_pos(deux);
        if ex == 10 || yi == 10 {
            println!("Not a position.");
            return false;
        }
        for (pos, piece) in self.pieces.clone().into_iter().enumerate() {
            if piece.get_pos().x == ex && piece.get_pos().y == yi {
                self.selected = Some(pos);
                return true;
            }
        }
        println!("Piece not found.");
        false
    }
    fn mark(&mut self, une: char, deux: char) -> bool {
        let ex = Self::char_to_pos(une);
        let yi = Self::char_to_pos(deux);
        if ex == 10 || yi == 10 {
            println!("Not a position.");
            return false;
        }
        for (pos, piece) in self.pieces.clone().into_iter().enumerate() {
            if piece.get_pos().x == ex && piece.get_pos().y == yi {
                self.marked = Some(pos);
                return true;
            }
        }
        self.pieces.push(Piece::new(ex, yi, None, None));
        self.marked = Some(self.pieces.len() - 1);
        true
    }
    fn switch_team(&mut self) {
        if self.turn == Team::White {
            self.turn = Team::Black
        } else {
            self.turn = Team::White
        }
    }
    fn char_to_pos(ch: char) -> usize {
        if !ch.is_ascii() {
            return 10;
        }
        let mut n = ch as usize;
        if n >= '0' as usize && n < '9' as usize {
            n -= '1' as usize;
        } else if n >= 'a' as usize && n < 'h' as usize {
            n = n - 'a' as usize;
        } else {
            return 10;
        }
        n
    }
}
