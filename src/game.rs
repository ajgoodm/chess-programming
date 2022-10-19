use crate::board::{File, Rank, Square};
use crate::piece::{PieceColor, PieceRole};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Piece {
    color: PieceColor,
    role: PieceRole,
    square: Square,
}

impl Piece {
    pub fn new(color: PieceColor, role: PieceRole, file: &File, rank: &Rank) -> Piece {
        Piece {
            color,
            role,
            square: Square::from_file_rank(file, rank),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_new() {
        assert_eq!(
            Piece::new(PieceColor::White, PieceRole::Rook, &File::A, &Rank::First),
            Piece {
                color: PieceColor::White,
                role: PieceRole::Rook,
                square: Square::A1
            }
        );
    }
}
