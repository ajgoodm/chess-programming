use std::collections::HashSet;

use crate::board::{Rank, Square};

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
        let to = Piece::new(from.role.clone(), from.color.clone(), destination_square);

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
}

impl Piece {
    pub fn new(role: PieceRole, color: PieceColor, square: Square) -> Piece {
        Piece {
            role,
            color,
            square,
        }
    }

    /// I return all squares accesible from my position
    /// in one move. I do not know anything about game state.
    pub fn candidate_moves(&self) -> HashSet<Move> {
        match self.role {
            PieceRole::Pawn => self.pawn_candidate_moves(),
            PieceRole::Rook => self.rook_candidate_moves(),
            PieceRole::Knight => self.knight_candidate_moves(),
            PieceRole::Bishop => self.bishop_candidate_moves(),
            PieceRole::King => self.king_candidate_moves(),
            PieceRole::Queen => self.queen_candidate_moves(),
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
                if self.square.rank() == Rank::Second {
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
                if self.square.rank() == Rank::Seventh {
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

    fn bishop_candidate_moves(&self) -> HashSet<Move> {
        let mut squares = self
            .square
            .ne_diagonal_squares()
            .into_iter()
            .collect::<HashSet<Square>>();
        squares.extend(self.square.nw_diagonal_squares());
        squares.remove(&self.square);
        squares
            .into_iter()
            .map(|square| Move::new(self.clone(), square, false))
            .collect()
    }

    fn queen_candidate_moves(&self) -> HashSet<Move> {
        let mut squares = self
            .square
            .file_squares()
            .into_iter()
            .collect::<HashSet<Square>>();
        squares.extend(self.square.rank_squares());
        squares.extend(self.square.ne_diagonal_squares());
        squares.extend(self.square.nw_diagonal_squares());
        squares.remove(&self.square);
        squares
            .into_iter()
            .map(|square| Move::new(self.clone(), square, false))
            .collect()
    }

    /// King candidate moves consider castling rights
    /// or threatened squares. These must be considered
    /// in the context of a GameState
    fn king_candidate_moves(&self) -> HashSet<Move> {
        [
            self.square.north(1),
            self.square.north_east(1),
            self.square.east(1),
            self.square.south_east(1),
            self.square.south(1),
            self.square.south_west(1),
            self.square.west(1),
            self.square.north_west(1),
        ]
        .into_iter()
        .filter(|x| Option::is_some(x))
        .map(|square| Move::new(self.clone(), square.unwrap(), false))
        .collect()
    }

    fn knight_candidate_moves(&self) -> HashSet<Move> {
        let mut squares: HashSet<Option<Square>> = HashSet::new();
        if let Some(n) = self.square.north(2) {
            squares.insert(n.east(1));
            squares.insert(n.west(1));
        }
        if let Some(e) = self.square.east(2) {
            squares.insert(e.north(1));
            squares.insert(e.south(1));
        }
        if let Some(s) = self.square.south(2) {
            squares.insert(s.east(1));
            squares.insert(s.west(1));
        }
        if let Some(w) = self.square.west(2) {
            squares.insert(w.north(1));
            squares.insert(w.south(1));
        }
        squares
            .into_iter()
            .filter(|x| Option::is_some(x))
            .map(|square| Move::new(self.clone(), square.unwrap(), false))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::board::{File, Rank};

    #[test]
    fn test_pawn_moves_white() {
        let b2_pawn = Piece::new(PieceRole::Pawn, PieceColor::White, Square::B2);
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
    }

    #[test]
    fn test_pawn_white_has_moved() {
        let a3_pawn = Piece::new(PieceRole::Pawn, PieceColor::White, Square::A3);
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
    }

    #[test]
    fn test_pawn_moves_black() {
        let b7_pawn = Piece::new(PieceRole::Pawn, PieceColor::Black, Square::B7);
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
    }

    #[test]
    fn test_rook_moves() {
        let a1_rook = Piece::new(PieceRole::Rook, PieceColor::White, Square::A1);
        let moves = a1_rook.candidate_moves();
        assert_eq!(moves.len(), 14); // 1st rank and A file but not A1
        assert!(moves.iter().all(|move_| {
            move_.to.square.rank() == Rank::First || move_.to.square.file() == File::A
        }));
    }

    #[test]
    fn test_bishop_moves() {
        let a1_bishop = Piece::new(PieceRole::Bishop, PieceColor::White, Square::A1);
        assert_eq!(a1_bishop.candidate_moves().len(), 7);
        let b2_bishop = Piece::new(PieceRole::Bishop, PieceColor::White, Square::B2);
        assert_eq!(b2_bishop.candidate_moves().len(), 9);
    }

    #[test]
    fn test_queen_candidate_moves() {
        let a1_queen = Piece::new(PieceRole::Queen, PieceColor::White, Square::A1);
        assert_eq!(a1_queen.candidate_moves().len(), 21);
    }

    #[test]
    fn test_king_candidate_moves() {
        let b1_king = Piece::new(PieceRole::King, PieceColor::White, Square::B1);
        assert_eq!(b1_king.candidate_moves().len(), 5);
    }

    #[test]
    fn test_knight_moves() {
        let d4_knight = Piece::new(PieceRole::Knight, PieceColor::White, Square::D4);
        assert_eq!(d4_knight.candidate_moves().len(), 8);

        let a1_knight = Piece::new(PieceRole::Knight, PieceColor::White, Square::A1);
        assert_eq!(a1_knight.candidate_moves().len(), 2);
    }
}
