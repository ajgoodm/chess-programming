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
