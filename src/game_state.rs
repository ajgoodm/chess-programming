use std::collections::HashSet;

use crate::board::Square;
use crate::piece::{Piece, PieceColor};

struct GameState {
    white_pieces: HashSet<Piece>,
    black_pieces: HashSet<Piece>,
    to_play: PieceColor,
}

struct Move {
    from: Piece,
    to: Square,
}

impl GameState {
    /// Returns a vector of all legal moves and resulting
    /// game states accessible from self.
    fn moves(&self) -> Vec<(Move, GameState)> {
        match self.to_play {
            PieceColor::White => self
                .white_pieces
                .iter()
                .flat_map(|piece| self.moves_for_piece(piece))
                .collect(),
            PieceColor::Black => self
                .black_pieces
                .iter()
                .flat_map(|piece| self.moves_for_piece(piece))
                .collect(),
        }
    }

    fn moves_for_piece(&self, piece: &Piece) -> Vec<(Move, GameState)> {
        piece
            .candidate_moves()
            .into_iter()
            .filter_map(|square| self.execute_move(square, piece))
            .collect()
    }

    fn execute_move(&self, square: Square, piece: &Piece) -> Option<(Move, GameState)> {
        None
    }
}
