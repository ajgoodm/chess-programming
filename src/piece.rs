use crate::board::Square;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PieceRole {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

pub struct Piece {
    role: PieceRole,
    color: PieceColor,
    square: Square,
}

impl Piece {
    /// I return all squares accesible from my position
    /// in one move. I do not know anything about game state.
    pub fn candidate_moves(&self) -> Vec<Square> {
        Vec::new()
    }
}
