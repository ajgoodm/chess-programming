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

pub struct Move {
    from: Piece,
    to: Piece,
    is_capture: bool,
}

impl Move {
    fn new(from: Piece, destination_square: Square, is_capture: bool) -> Move {
        let to = Piece::new(
            from.role.clone(),
            from.color.clone(),
            destination_square,
            true,
        );

        Move {
            from,
            to,
            is_capture,
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn candidate_moves(&self) -> Vec<Move> {
        match self.role {
            PieceRole::Pawn => self.pawn_candidate_moves(),
            PieceRole::Rook => Vec::new(),
            PieceRole::Knight => Vec::new(),
            PieceRole::Bishop => Vec::new(),
            PieceRole::King => Vec::new(),
            PieceRole::Queen => Vec::new(),
        }
    }

    fn pawn_candidate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move>;
        match self.color {
            PieceColor::White => {
                moves = [
                    (self.square.north_west(1), true),
                    (self.square.north(1), false),
                    (self.square.north_east(1), true),
                ]
                .into_iter()
                .filter(|(square, _)| Option::is_some(square))
                .map(|(square, is_capture)| Move::new(self.clone(), square.unwrap(), is_capture))
                .collect();
                if !self.has_moved {
                    if let Some(double_move_sq) = self.square.north(2) {
                        moves.push(Move::new(self.clone(), double_move_sq, false));
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
                .map(|(square, is_capture)| Move::new(self.clone(), square.unwrap(), is_capture))
                .collect();
                if !self.has_moved {
                    if let Some(double_move_sq) = self.square.south(2) {
                        moves.push(Move::new(self.clone(), double_move_sq, false));
                    }
                }
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
}
