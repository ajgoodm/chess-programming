use std::collections::HashSet;

use crate::piece::{Move, Piece, PieceColor};

struct GameState {
    white_pieces: HashSet<Piece>,
    black_pieces: HashSet<Piece>,
    to_play: PieceColor,
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

    fn moves_for_piece(&self, from: &Piece) -> Vec<(Move, GameState)> {
        from.candidate_moves()
            .into_iter()
            .filter_map(|move_| self.execute_move(move_))
            .collect()
    }

    /// I take a proposed move (piece and destination square) that is 'allowed'
    /// according to a piece's move set and evaluate it it the context of self
    /// to see if it's actually allowed (is the square occupied? Is a castling square
    /// threatened?).
    fn execute_move(&self, move_: Move) -> Option<(Move, GameState)> {
        None
    }
}
