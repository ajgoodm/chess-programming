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
        let mut moves: Vec<Move> = Vec::new();
        moves
    }
}
