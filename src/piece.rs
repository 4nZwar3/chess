#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Pawn = 1,
    Tower,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Team {
    White,
    Black,
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Piece {
    pos: Position,
    ty: Option<Type>,
    eq: Option<Team>,
    uses: usize,
}
impl Piece {
    pub fn new(x: usize, y: usize, t: Option<Type>, e: Option<Team>) -> Piece {
        Piece {
            pos: Position::new(x, y),
            ty: t,
            eq: e,
            uses: 0,
        }
    }
    pub fn get_pos(&self) -> Position {
        self.pos
    }
    pub fn get_pos_ref(&self) -> &Position {
        &self.pos
    }
    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = Position::new(x, y)
    }
    pub fn get_type(&self) -> Option<Type> {
        self.ty
    }
    pub fn set_type(&mut self, t: Type) {
        self.ty = Some(t)
    }
    pub fn get_team(&self) -> Option<Team> {
        self.eq
    }
    pub fn set_team(&mut self, e: Team) {
        self.eq = Some(e)
    }
    pub fn same_team(&self, other: Piece) -> bool {
        if self.get_team() == other.get_team() {
            return true;
        }
        false
    }
    pub fn is_white(&self) -> bool {
        if self.get_team() == Some(Team::White) {
            return true;
        }
        false
    }
    pub fn is_black(&self) -> bool {
        if self.get_team() == Some(Team::Black) {
            return true;
        }
        false
    }
    pub fn get_uses(&self) -> usize {
        self.uses.clone()
    }
    pub fn set_uses(&mut self, u: usize) {
        self.uses = u
    }
    pub fn inc_use(&mut self) {
        self.uses += 1
    }
}
