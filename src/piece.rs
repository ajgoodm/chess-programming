use crate::board::{File, Rank, Square};

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

pub trait Piece {
    fn color(&self) -> &PieceColor;

    fn role(&self) -> &PieceRole;

    fn square(&self) -> &Square;
}

pub struct Pawn {
    color: PieceColor,
    square: Square,
}

impl Pawn {
    pub fn new(color: PieceColor, file: &File, rank: &Rank) -> Pawn {
        Pawn {
            color,
            square: Square::from_file_rank(file, rank),
        }
    }
}

impl Piece for Pawn {
    fn color(&self) -> &PieceColor {
        &self.color
    }
    fn role(&self) -> &PieceRole {
        &PieceRole::Pawn
    }
    fn square(&self) -> &Square {
        &self.square
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_properties() {
        let pawn = Pawn::new(PieceColor::White, &File::A, &Rank::Second);
        assert_eq!(pawn.color(), &PieceColor::White);
        assert_eq!(pawn.role(), &PieceRole::Pawn);
        assert_eq!(pawn.square(), &Square::A2);
    }
}
