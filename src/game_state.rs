use std::{collections::HashSet, hash::Hash};

use crate::board::Square;
use crate::piece::{Move, Piece, PieceColor};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CastlingRights {
    QueenSide(PieceColor),
    KingSide(PieceColor),
}

pub struct GameState {
    white_pieces: HashSet<Piece>,
    black_pieces: HashSet<Piece>,
    to_play: PieceColor,
    halfmove_clock: usize,
    fullmove_number: usize,
    castling_rights: HashSet<CastlingRights>,
    en_passant_target: Option<Square>,
}

impl GameState {
    pub fn from_fen(fen: String) -> GameState {
        use crate::fen_serialization;

        let parts: Vec<&str> = fen.split(' ').collect();
        assert_eq!(parts.len(), 6);
        let (white_pieces, black_pieces) = fen_serialization::parse_pieces(parts[0]);
        let to_play = fen_serialization::parse_to_play(parts[1]);
        let castling_rights = fen_serialization::parse_castle_rights(parts[2]);
        let en_passant_target = fen_serialization::parse_en_passant_square(parts[3]);
        let halfmove_clock: usize = parts[4]
            .parse::<usize>()
            .expect("failed to parse halfmove clock");
        let fullmove_number: usize = parts[5]
            .parse::<usize>()
            .expect("failed to parse fullmove number");

        GameState {
            white_pieces,
            black_pieces,
            to_play,
            halfmove_clock,
            fullmove_number,
            castling_rights,
            en_passant_target,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen() {
        GameState::from_fen(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string(),
        );
    }
}
