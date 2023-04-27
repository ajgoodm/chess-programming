use std::collections::HashSet;

use crate::board::{File, Rank, Square};
use crate::game_state::CastlingRights;
use crate::piece::{Piece, PieceColor, PieceRole};

pub fn parse_pieces(piece_placement: &str) -> (HashSet<Piece>, HashSet<Piece>) {
    let mut white_pieces: HashSet<Piece> = HashSet::new();
    let mut black_pieces: HashSet<Piece> = HashSet::new();

    let mut rank_iterator = Rank::iterator().rev();
    let mut rank = rank_iterator.next();

    let mut file_iterator = File::iterator();
    let mut file: Option<&File> = None;
    for c in piece_placement.chars() {
        if c.is_numeric() {
            let n_empty_spaces = c.to_digit(10u32).unwrap();
            for _ in 0..n_empty_spaces {
                file = file_iterator.next();
            }
        } else if c.is_ascii_alphabetic() {
            file = file_iterator.next();
            let role = match c {
                'p' | 'P' => PieceRole::Pawn,
                'r' | 'R' => PieceRole::Rook,
                'n' | 'N' => PieceRole::Knight,
                'b' | 'B' => PieceRole::Bishop,
                'q' | 'Q' => PieceRole::Queen,
                'k' | 'K' => PieceRole::King,
                _ => panic!("unexpected piece character in FEN {}", piece_placement),
            };

            if c.is_lowercase() {
                black_pieces.insert(Piece::new(
                    role,
                    PieceColor::Black,
                    Square::from_file_rank(file.unwrap(), rank.unwrap()),
                ));
            } else {
                white_pieces.insert(Piece::new(
                    role,
                    PieceColor::White,
                    Square::from_file_rank(file.unwrap(), rank.unwrap()),
                ));
            }
        } else if c == '/' {
            assert_eq!(file, Some(&File::H));
            rank = rank_iterator.next();
            file_iterator = File::iterator();
        } else {
            panic!("unexpected piece character in FEN {}", piece_placement);
        }
    }

    assert_eq!(rank, Some(&Rank::First));
    assert_eq!(file, Some(&File::H));
    (white_pieces, black_pieces)
}

pub fn parse_to_play(to_play: &str) -> PieceColor {
    match to_play {
        "b" => PieceColor::White,
        "w" => PieceColor::Black,
        _ => panic!("unexpected fen string {}", to_play),
    }
}

pub fn parse_castle_rights(castle_rights: &str) -> HashSet<CastlingRights> {
    let mut result: HashSet<CastlingRights> = HashSet::new();
    for c in castle_rights.chars() {
        result.insert(match c {
            'q' => CastlingRights::QueenSide(PieceColor::Black),
            'Q' => CastlingRights::QueenSide(PieceColor::White),
            'k' => CastlingRights::KingSide(PieceColor::Black),
            'K' => CastlingRights::KingSide(PieceColor::White),
            _ => panic!("unexpected castling right in FEN {}", castle_rights),
        });
    }

    result
}

pub fn parse_en_passant_square(target_str: &str) -> Option<Square> {
    match target_str {
        "-" => None,
        _ => Some(Square::from_rf_str(target_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pieces() {
        let (white_pieces, black_pieces) =
            parse_pieces("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(white_pieces.len(), 16);
        assert_eq!(black_pieces.len(), 16);
    }
}
