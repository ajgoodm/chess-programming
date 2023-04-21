use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    from: Piece,
    to: Piece,
    requires_capture: bool,
}

impl Move {
    fn new(from: Piece, destination_square: Square, requires_capture: bool) -> Move {
        let to = Piece::new(
            from.role.clone(),
            from.color.clone(),
            destination_square,
            true,
        );

        Move {
            from,
            to,
            requires_capture,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    role: PieceRole,
    color: PieceColor,
    square: Square,
    has_moved: bool,
}

impl Piece {
    pub fn new(role: PieceRole, color: PieceColor, square: Square, has_moved: bool) -> Piece {
        Piece {
            role,
            color,
            square,
            has_moved,
        }
    }

    /// I return all squares accesible from my position
    /// in one move. I do not know anything about game state.
    pub fn candidate_moves(&self) -> HashSet<Move> {
        match self.role {
            PieceRole::Pawn => self.pawn_candidate_moves(),
            PieceRole::Rook => self.rook_candidate_moves(),
            PieceRole::Knight => HashSet::new(),
            PieceRole::Bishop => HashSet::new(),
            PieceRole::King => HashSet::new(),
            PieceRole::Queen => HashSet::new(),
        }
    }

    fn pawn_candidate_moves(&self) -> HashSet<Move> {
        let mut moves: HashSet<Move>;
        match self.color {
            PieceColor::White => {
                moves = [
                    (self.square.north_west(1), true),
                    (self.square.north(1), false),
                    (self.square.north_east(1), true),
                ]
                .into_iter()
                .filter(|(square, _)| Option::is_some(square))
                .map(|(square, requires_capture)| {
                    Move::new(self.clone(), square.unwrap(), requires_capture)
                })
                .collect();
                if !self.has_moved {
                    if let Some(double_move_sq) = self.square.north(2) {
                        moves.insert(Move::new(self.clone(), double_move_sq, false));
                    }
                }
            }
            PieceColor::Black => {
                moves = [
                    (self.square.south_east(1), true),
                    (self.square.south(1), false),
                    (self.square.south_west(1), true),
                ]
                .into_iter()
                .filter(|(square, _)| Option::is_some(square))
                .map(|(square, requires_capture)| {
                    Move::new(self.clone(), square.unwrap(), requires_capture)
                })
                .collect();
                if !self.has_moved {
                    if let Some(double_move_sq) = self.square.south(2) {
                        moves.insert(Move::new(self.clone(), double_move_sq, false));
                    }
                }
            }
        }
        moves
    }

    fn rook_candidate_moves(&self) -> HashSet<Move> {
        let mut squares = self
            .square
            .rank_squares()
            .into_iter()
            .collect::<HashSet<Square>>();
        squares.extend(self.square.file_squares());
        squares.remove(&self.square);
        squares
            .into_iter()
            .map(|square| Move::new(self.clone(), square, false))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::{File, Rank};
    use super::*;

    #[test]
    fn test_pawn_moves_white() {
        let b2_pawn = Piece::new(PieceRole::Pawn, PieceColor::White, Square::B2, false);
        let moves = b2_pawn.candidate_moves();
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves
                .iter()
                .map(|m| m.to.square.clone())
                .collect::<HashSet<Square>>(),
            vec![Square::A3, Square::B3, Square::C3, Square::B4,]
                .into_iter()
                .collect::<HashSet<Square>>()
        );
        assert!(moves.iter().all(|m| m.to.has_moved));
    }

    #[test]
    fn test_pawn_white_has_moved() {
        let a3_pawn = Piece::new(PieceRole::Pawn, PieceColor::White, Square::A3, true);
        let moves = a3_pawn.candidate_moves();
        assert_eq!(moves.len(), 2);
        assert_eq!(
            moves
                .iter()
                .map(|m| m.to.square.clone())
                .collect::<HashSet<Square>>(),
            vec![Square::A4, Square::B4,]
                .into_iter()
                .collect::<HashSet<Square>>()
        );
        assert!(moves.iter().all(|m| m.to.has_moved));
    }

    #[test]
    fn test_pawn_moves_black() {
        let b7_pawn = Piece::new(PieceRole::Pawn, PieceColor::Black, Square::B7, false);
        let moves = b7_pawn.candidate_moves();
        assert_eq!(moves.len(), 4);
        assert_eq!(
            moves
                .iter()
                .map(|m| m.to.square.clone())
                .collect::<HashSet<Square>>(),
            vec![Square::A6, Square::B6, Square::C6, Square::B5,]
                .into_iter()
                .collect::<HashSet<Square>>()
        );
        assert!(moves.iter().all(|m| m.to.has_moved));
    }

    #[test]
    fn test_rook_moves() {
        let a1_rook = Piece::new(PieceRole::Rook, PieceColor::White, Square::A1, false);
        let moves = a1_rook.candidate_moves();
        assert_eq!(moves.len(), 14); // 1st rank and A file but not A1
        assert!(moves.iter().all(|move_| {
            move_.to.square.rank() == Rank::First || move_.to.square.file() == File::A
        }));
    }
}
