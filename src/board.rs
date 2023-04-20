use std::{
    collections::HashMap,
    hash::Hash,
};

lazy_static! {
    static ref FILE_A_SQUARES: Vec<Square> = [
        Square::A1,
        Square::A2,
        Square::A3,
        Square::A4,
        Square::A5,
        Square::A6,
        Square::A7,
        Square::A8
    ]
    .into_iter()
    .collect();
    static ref FILE_B_SQUARES: Vec<Square> = [
        Square::B1,
        Square::B2,
        Square::B3,
        Square::B4,
        Square::B5,
        Square::B6,
        Square::B7,
        Square::B8
    ]
    .into_iter()
    .collect();
    static ref FILE_C_SQUARES: Vec<Square> = [
        Square::C1,
        Square::C2,
        Square::C3,
        Square::C4,
        Square::C5,
        Square::C6,
        Square::C7,
        Square::C8
    ]
    .into_iter()
    .collect();
    static ref FILE_D_SQUARES: Vec<Square> = [
        Square::D1,
        Square::D2,
        Square::D3,
        Square::D4,
        Square::D5,
        Square::D6,
        Square::D7,
        Square::D8
    ]
    .into_iter()
    .collect();
    static ref FILE_E_SQUARES: Vec<Square> = [
        Square::E1,
        Square::E2,
        Square::E3,
        Square::E4,
        Square::E5,
        Square::E6,
        Square::E7,
        Square::E8
    ]
    .into_iter()
    .collect();
    static ref FILE_F_SQUARES: Vec<Square> = [
        Square::F1,
        Square::F2,
        Square::F3,
        Square::F4,
        Square::F5,
        Square::F6,
        Square::F7,
        Square::F8
    ]
    .into_iter()
    .collect();
    static ref FILE_G_SQUARES: Vec<Square> = [
        Square::G1,
        Square::G2,
        Square::G3,
        Square::G4,
        Square::G5,
        Square::G6,
        Square::G7,
        Square::G8
    ]
    .into_iter()
    .collect();
    static ref FILE_H_SQUARES: Vec<Square> = [
        Square::H1,
        Square::H2,
        Square::H3,
        Square::H4,
        Square::H5,
        Square::H6,
        Square::H7,
        Square::H8
    ]
    .into_iter()
    .collect();
    static ref RANK_1_SQUARES: Vec<Square> = [
        Square::A1,
        Square::B1,
        Square::C1,
        Square::D1,
        Square::E1,
        Square::F1,
        Square::G1,
        Square::H1,
    ]
    .into_iter()
    .collect();
    static ref RANK_2_SQUARES: Vec<Square> = [
        Square::A2,
        Square::B2,
        Square::C2,
        Square::D2,
        Square::E2,
        Square::F2,
        Square::G2,
        Square::H2,
    ]
    .into_iter()
    .collect();
    static ref RANK_3_SQUARES: Vec<Square> = [
        Square::A3,
        Square::B3,
        Square::C3,
        Square::D3,
        Square::E3,
        Square::F3,
        Square::G3,
        Square::H3,
    ]
    .into_iter()
    .collect();
    static ref RANK_4_SQUARES: Vec<Square> = [
        Square::A4,
        Square::B4,
        Square::C4,
        Square::D4,
        Square::E4,
        Square::F4,
        Square::G4,
        Square::H4,
    ]
    .into_iter()
    .collect();
    static ref RANK_5_SQUARES: Vec<Square> = [
        Square::A5,
        Square::B5,
        Square::C5,
        Square::D5,
        Square::E5,
        Square::F5,
        Square::G5,
        Square::H5,
    ]
    .into_iter()
    .collect();
    static ref RANK_6_SQUARES: Vec<Square> = [
        Square::A6,
        Square::B6,
        Square::C6,
        Square::D6,
        Square::E6,
        Square::F6,
        Square::G6,
        Square::H6,
    ]
    .into_iter()
    .collect();
    static ref RANK_7_SQUARES: Vec<Square> = [
        Square::A7,
        Square::B7,
        Square::C7,
        Square::D7,
        Square::E7,
        Square::F7,
        Square::G7,
        Square::H7,
    ]
    .into_iter()
    .collect();
    static ref RANK_8_SQUARES: Vec<Square> = [
        Square::A8,
        Square::B8,
        Square::C8,
        Square::D8,
        Square::E8,
        Square::F8,
        Square::G8,
        Square::H8,
    ]
    .into_iter()
    .collect();
    static ref FILE_RANK_TO_SQUARE: HashMap<(&'static File, &'static Rank), Square> = {
        let mut m: HashMap<(&File, &Rank), Square> = HashMap::new();
        // File A
        m.insert((&File::A, &Rank::First), Square::A1);
        m.insert((&File::A, &Rank::Second), Square::A2);
        m.insert((&File::A, &Rank::Third), Square::A3);
        m.insert((&File::A, &Rank::Fourth), Square::A4);
        m.insert((&File::A, &Rank::Fifth), Square::A5);
        m.insert((&File::A, &Rank::Sixth), Square::A6);
        m.insert((&File::A, &Rank::Seventh), Square::A7);
        m.insert((&File::A, &Rank::Eigth), Square::A8);

        // File B
        m.insert((&File::B, &Rank::First), Square::B1);
        m.insert((&File::B, &Rank::Second), Square::B2);
        m.insert((&File::B, &Rank::Third), Square::B3);
        m.insert((&File::B, &Rank::Fourth), Square::B4);
        m.insert((&File::B, &Rank::Fifth), Square::B5);
        m.insert((&File::B, &Rank::Sixth), Square::B6);
        m.insert((&File::B, &Rank::Seventh), Square::B7);
        m.insert((&File::B, &Rank::Eigth), Square::B8);

        // File C
        m.insert((&File::C, &Rank::First), Square::C1);
        m.insert((&File::C, &Rank::Second), Square::C2);
        m.insert((&File::C, &Rank::Third), Square::C3);
        m.insert((&File::C, &Rank::Fourth), Square::C4);
        m.insert((&File::C, &Rank::Fifth), Square::C5);
        m.insert((&File::C, &Rank::Sixth), Square::C6);
        m.insert((&File::C, &Rank::Seventh), Square::C7);
        m.insert((&File::C, &Rank::Eigth), Square::C8);

        // File D
        m.insert((&File::D, &Rank::First), Square::D1);
        m.insert((&File::D, &Rank::Second), Square::D2);
        m.insert((&File::D, &Rank::Third), Square::D3);
        m.insert((&File::D, &Rank::Fourth), Square::D4);
        m.insert((&File::D, &Rank::Fifth), Square::D5);
        m.insert((&File::D, &Rank::Sixth), Square::D6);
        m.insert((&File::D, &Rank::Seventh), Square::D7);
        m.insert((&File::D, &Rank::Eigth), Square::D8);

        // File E
        m.insert((&File::E, &Rank::First), Square::E1);
        m.insert((&File::E, &Rank::Second), Square::E2);
        m.insert((&File::E, &Rank::Third), Square::E3);
        m.insert((&File::E, &Rank::Fourth), Square::E4);
        m.insert((&File::E, &Rank::Fifth), Square::E5);
        m.insert((&File::E, &Rank::Sixth), Square::E6);
        m.insert((&File::E, &Rank::Seventh), Square::E7);
        m.insert((&File::E, &Rank::Eigth), Square::E8);

        // File F
        m.insert((&File::F, &Rank::First), Square::F1);
        m.insert((&File::F, &Rank::Second), Square::F2);
        m.insert((&File::F, &Rank::Third), Square::F3);
        m.insert((&File::F, &Rank::Fourth), Square::F4);
        m.insert((&File::F, &Rank::Fifth), Square::F5);
        m.insert((&File::F, &Rank::Sixth), Square::F6);
        m.insert((&File::F, &Rank::Seventh), Square::F7);
        m.insert((&File::F, &Rank::Eigth), Square::F8);

        // File G
        m.insert((&File::G, &Rank::First), Square::G1);
        m.insert((&File::G, &Rank::Second), Square::G2);
        m.insert((&File::G, &Rank::Third), Square::G3);
        m.insert((&File::G, &Rank::Fourth), Square::G4);
        m.insert((&File::G, &Rank::Fifth), Square::G5);
        m.insert((&File::G, &Rank::Sixth), Square::G6);
        m.insert((&File::G, &Rank::Seventh), Square::G7);
        m.insert((&File::G, &Rank::Eigth), Square::G8);

        // File H
        m.insert((&File::H, &Rank::First), Square::H1);
        m.insert((&File::H, &Rank::Second), Square::H2);
        m.insert((&File::H, &Rank::Third), Square::H3);
        m.insert((&File::H, &Rank::Fourth), Square::H4);
        m.insert((&File::H, &Rank::Fifth), Square::H5);
        m.insert((&File::H, &Rank::Sixth), Square::H6);
        m.insert((&File::H, &Rank::Seventh), Square::H7);
        m.insert((&File::H, &Rank::Eigth), Square::H8);
        m
    };

    // used to look up square by index
    static ref SQUARE_ARRAY: [Square; 64] = [
        Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
        Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
        Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
        Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
        Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
        Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
        Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
        Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    ];

    // used to look up index by square
    static ref SQUARE_TO_INDEX: HashMap<Square, usize> = {
        let mut m: HashMap<Square, usize> = HashMap::new();
        for (idx, square) in SQUARE_ARRAY.iter().enumerate() {
            m.insert(square.clone(), idx);
        }
        m
    };

    static ref SQUARE_TO_RANK: HashMap<Square, Rank> = {
        let mut m: HashMap<Square, Rank> = HashMap::new();
        m.insert(Square::A1, Rank::First);
        m.insert(Square::B1, Rank::First);
        m.insert(Square::C1, Rank::First);
        m.insert(Square::D1, Rank::First);
        m.insert(Square::E1, Rank::First);
        m.insert(Square::F1, Rank::First);
        m.insert(Square::G1, Rank::First);
        m.insert(Square::H1, Rank::First);

        m.insert(Square::A2, Rank::Second);
        m.insert(Square::B2, Rank::Second);
        m.insert(Square::C2, Rank::Second);
        m.insert(Square::D2, Rank::Second);
        m.insert(Square::E2, Rank::Second);
        m.insert(Square::F2, Rank::Second);
        m.insert(Square::G2, Rank::Second);
        m.insert(Square::H2, Rank::Second);

        m.insert(Square::A3, Rank::Third);
        m.insert(Square::B3, Rank::Third);
        m.insert(Square::C3, Rank::Third);
        m.insert(Square::D3, Rank::Third);
        m.insert(Square::E3, Rank::Third);
        m.insert(Square::F3, Rank::Third);
        m.insert(Square::G3, Rank::Third);
        m.insert(Square::H3, Rank::Third);

        m.insert(Square::A4, Rank::Fourth);
        m.insert(Square::B4, Rank::Fourth);
        m.insert(Square::C4, Rank::Fourth);
        m.insert(Square::D4, Rank::Fourth);
        m.insert(Square::E4, Rank::Fourth);
        m.insert(Square::F4, Rank::Fourth);
        m.insert(Square::G4, Rank::Fourth);
        m.insert(Square::H4, Rank::Fourth);

        m.insert(Square::A5, Rank::Fifth);
        m.insert(Square::B5, Rank::Fifth);
        m.insert(Square::C5, Rank::Fifth);
        m.insert(Square::D5, Rank::Fifth);
        m.insert(Square::E5, Rank::Fifth);
        m.insert(Square::F5, Rank::Fifth);
        m.insert(Square::G5, Rank::Fifth);
        m.insert(Square::H5, Rank::Fifth);

        m.insert(Square::A6, Rank::Sixth);
        m.insert(Square::B6, Rank::Sixth);
        m.insert(Square::C6, Rank::Sixth);
        m.insert(Square::D6, Rank::Sixth);
        m.insert(Square::E6, Rank::Sixth);
        m.insert(Square::F6, Rank::Sixth);
        m.insert(Square::G6, Rank::Sixth);
        m.insert(Square::H6, Rank::Sixth);

        m.insert(Square::A7, Rank::Seventh);
        m.insert(Square::B7, Rank::Seventh);
        m.insert(Square::C7, Rank::Seventh);
        m.insert(Square::D7, Rank::Seventh);
        m.insert(Square::E7, Rank::Seventh);
        m.insert(Square::F7, Rank::Seventh);
        m.insert(Square::G7, Rank::Seventh);
        m.insert(Square::H7, Rank::Seventh);

        m.insert(Square::A8, Rank::Eigth);
        m.insert(Square::B8, Rank::Eigth);
        m.insert(Square::C8, Rank::Eigth);
        m.insert(Square::D8, Rank::Eigth);
        m.insert(Square::E8, Rank::Eigth);
        m.insert(Square::F8, Rank::Eigth);
        m.insert(Square::G8, Rank::Eigth);
        m.insert(Square::H8, Rank::Eigth);
        m
    };

    static ref SQUARE_TO_FILE: HashMap<Square, File> = {
        let mut m: HashMap<Square, File> = HashMap::new();
        m.insert(Square::A1, File::A);
        m.insert(Square::A2, File::A);
        m.insert(Square::A3, File::A);
        m.insert(Square::A4, File::A);
        m.insert(Square::A5, File::A);
        m.insert(Square::A6, File::A);
        m.insert(Square::A7, File::A);
        m.insert(Square::A8, File::A);

        m.insert(Square::B1, File::B);
        m.insert(Square::B2, File::B);
        m.insert(Square::B3, File::B);
        m.insert(Square::B4, File::B);
        m.insert(Square::B5, File::B);
        m.insert(Square::B6, File::B);
        m.insert(Square::B7, File::B);
        m.insert(Square::B8, File::B);

        m.insert(Square::C1, File::C);
        m.insert(Square::C2, File::C);
        m.insert(Square::C3, File::C);
        m.insert(Square::C4, File::C);
        m.insert(Square::C5, File::C);
        m.insert(Square::C6, File::C);
        m.insert(Square::C7, File::C);
        m.insert(Square::C8, File::C);

        m.insert(Square::D1, File::D);
        m.insert(Square::D2, File::D);
        m.insert(Square::D3, File::D);
        m.insert(Square::D4, File::D);
        m.insert(Square::D5, File::D);
        m.insert(Square::D6, File::D);
        m.insert(Square::D7, File::D);
        m.insert(Square::D8, File::D);

        m.insert(Square::E1, File::E);
        m.insert(Square::E2, File::E);
        m.insert(Square::E3, File::E);
        m.insert(Square::E4, File::E);
        m.insert(Square::E5, File::E);
        m.insert(Square::E6, File::E);
        m.insert(Square::E7, File::E);
        m.insert(Square::E8, File::E);

        m.insert(Square::F1, File::F);
        m.insert(Square::F2, File::F);
        m.insert(Square::F3, File::F);
        m.insert(Square::F4, File::F);
        m.insert(Square::F5, File::F);
        m.insert(Square::F6, File::F);
        m.insert(Square::F7, File::F);
        m.insert(Square::F8, File::F);

        m.insert(Square::G1, File::G);
        m.insert(Square::G2, File::G);
        m.insert(Square::G3, File::G);
        m.insert(Square::G4, File::G);
        m.insert(Square::G5, File::G);
        m.insert(Square::G6, File::G);
        m.insert(Square::G7, File::G);
        m.insert(Square::G8, File::G);

        m.insert(Square::H1, File::H);
        m.insert(Square::H2, File::H);
        m.insert(Square::H3, File::H);
        m.insert(Square::H4, File::H);
        m.insert(Square::H5, File::H);
        m.insert(Square::H6, File::H);
        m.insert(Square::H7, File::H);
        m.insert(Square::H8, File::H);
        m
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eigth,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

impl Square {
    pub fn from_file_rank(file: &File, rank: &Rank) -> Square {
        FILE_RANK_TO_SQUARE.get(&(file, rank)).unwrap().clone()
    }

    fn index(&self) -> usize {
        *SQUARE_TO_INDEX.get(self).unwrap()
    }

    pub fn rank(&self) -> Rank {
        *SQUARE_TO_RANK.get(self).unwrap()
    }

    pub fn rank_squares(&self) -> Vec<Square> {
        match self.rank() {
            Rank::First => RANK_1_SQUARES.iter().cloned().collect(),
            Rank::Second => RANK_2_SQUARES.iter().cloned().collect(),
            Rank::Third => RANK_3_SQUARES.iter().cloned().collect(),
            Rank::Fourth => RANK_4_SQUARES.iter().cloned().collect(),
            Rank::Fifth => RANK_5_SQUARES.iter().cloned().collect(),
            Rank::Sixth => RANK_6_SQUARES.iter().cloned().collect(),
            Rank::Seventh => RANK_7_SQUARES.iter().cloned().collect(),
            Rank::Eigth => RANK_8_SQUARES.iter().cloned().collect(),
        }
    }

    pub fn file_squares(&self) -> Vec<Square> {
        match self.file() {
            File::A => FILE_A_SQUARES.iter().cloned().collect(),
            File::B => FILE_B_SQUARES.iter().cloned().collect(),
            File::C => FILE_C_SQUARES.iter().cloned().collect(),
            File::D => FILE_D_SQUARES.iter().cloned().collect(),
            File::E => FILE_E_SQUARES.iter().cloned().collect(),
            File::F => FILE_F_SQUARES.iter().cloned().collect(),
            File::G => FILE_G_SQUARES.iter().cloned().collect(),
            File::H => FILE_H_SQUARES.iter().cloned().collect(),
        }
    }

    pub fn file(&self) -> File {
        *SQUARE_TO_FILE.get(self).unwrap()
    }

    /// I return the square that is n squares north of self.
    /// If there is no such square, I return None
    pub fn north(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let dest_idx = self.index() + (n * 8);
        if dest_idx >= 64 {
            None
        } else {
            Some(SQUARE_ARRAY[dest_idx].clone())
        }
    }

    pub fn north_east(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let dest_idx = self.index() + (n * 9);
        if dest_idx >= 64 {
            return None;
        }

        let dest_square = SQUARE_ARRAY[dest_idx].clone();
        if dest_square.file() <= self.file() {
            None
        } else {
            Some(dest_square)
        }
    }

    pub fn east(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let dest_idx = self.index() + n;
        if dest_idx > 64 {
            return None;
        }

        let dest_square = SQUARE_ARRAY[dest_idx].clone();
        if dest_square.rank() != self.rank() {
            None
        } else {
            Some(dest_square)
        }
    }

    pub fn south_east(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let subtrahend = n * 7;
        let self_idx = self.index();

        if subtrahend > self.index() {
            return None;
        }
        let dest_square = SQUARE_ARRAY[self_idx - subtrahend].clone();
        if dest_square.file() <= self.file() {
            None
        } else {
            Some(dest_square)
        }
    }

    pub fn south(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let subtrahend = n * 8;
        let self_idx = self.index();

        if subtrahend > self.index() {
            None
        } else {
            let dest_square = SQUARE_ARRAY[self_idx - subtrahend].clone();
            Some(dest_square)
        }
    }

    pub fn south_west(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let subtrahend = n * 9;
        let self_idx = self.index();

        if subtrahend > self.index() {
            return None;
        }
        let dest_square = SQUARE_ARRAY[self_idx - subtrahend].clone();
        if dest_square.file() >= self.file() {
            None
        } else {
            Some(dest_square)
        }
    }

    pub fn west(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let self_idx = self.index();

        if n > self.index() {
            return None;
        }
        let dest_square = SQUARE_ARRAY[self_idx - n].clone();
        if dest_square.rank() != self.rank() {
            None
        } else {
            Some(dest_square)
        }
    }

    pub fn north_west(&self, n: usize) -> Option<Square> {
        debug_assert!(n > 0);
        let dest_idx = self.index() + (n * 7);
        if dest_idx >= 64 {
            return None;
        }

        let dest_square = SQUARE_ARRAY[dest_idx].clone();
        if dest_square.file() >= self.file() {
            None
        } else {
            Some(dest_square)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_file_rank() {
        assert_eq!(Square::from_file_rank(&File::A, &Rank::First), Square::A1);
        assert_eq!(Square::from_file_rank(&File::G, &Rank::Fifth), Square::G5);
    }

    #[test]
    fn test_north() {
        assert_eq!(Square::A1.north(1), Some(Square::A2));
        assert_eq!(Square::A1.north(2), Some(Square::A3));
        assert_eq!(Square::A8.north(1), None);
    }

    #[test]
    fn test_north_east() {
        assert_eq!(Square::A1.north_east(1), Some(Square::B2));
        assert_eq!(Square::A1.north_east(2), Some(Square::C3));
        assert_eq!(Square::H1.north_east(1), None);
        assert_eq!(Square::A8.north_east(1), None);
    }

    #[test]
    fn test_east() {
        assert_eq!(Square::A1.east(1), Some(Square::B1));
        assert_eq!(Square::A1.east(4), Some(Square::E1));
        assert_eq!(Square::H1.north_east(1), None);
        assert_eq!(Square::H8.north_east(1), None);
    }

    #[test]
    fn test_south_east() {
        assert_eq!(Square::A1.south_east(1), None);
        assert_eq!(Square::H2.south_east(1), None);
        assert_eq!(Square::B2.south_east(1), Some(Square::C1));
    }

    #[test]
    fn test_south() {
        assert_eq!(Square::A1.south(1), None);
        assert_eq!(Square::A2.south(1), Some(Square::A1));
        assert_eq!(Square::A3.south(2), Some(Square::A1));
    }

    #[test]
    fn test_south_west() {
        assert_eq!(Square::A1.south_west(1), None);
        assert_eq!(Square::B1.south_west(1), None);
        assert_eq!(Square::H8.south_west(3), Some(Square::E5));
    }

    #[test]
    fn test_west() {
        assert_eq!(Square::A1.west(1), None);
        assert_eq!(Square::B2.west(1), Some(Square::A2));
    }

    #[test]
    fn test_north_west() {
        assert_eq!(Square::A1.north_west(1), None);
        assert_eq!(Square::H8.north_west(2), None);
        assert_eq!(Square::H1.north_west(3), Some(Square::E4));
    }

    #[test]
    fn test_rank_ordered() {
        let mut jumbled = vec![
            Rank::Third,
            Rank::Seventh,
            Rank::Fourth,
            Rank::First,
            Rank::Fifth,
            Rank::Sixth,
            Rank::Second,
            Rank::Eigth,
        ];
        jumbled.sort();

        assert_eq!(
            vec![
                Rank::First,
                Rank::Second,
                Rank::Third,
                Rank::Fourth,
                Rank::Fifth,
                Rank::Sixth,
                Rank::Seventh,
                Rank::Eigth,
            ],
            jumbled
        )
    }

    #[test]
    fn test_file_ordered() {
        let mut jumbled = vec![
            File::C,
            File::B,
            File::H,
            File::E,
            File::A,
            File::G,
            File::F,
            File::D,
        ];
        jumbled.sort();

        assert_eq!(
            vec![
                File::A,
                File::B,
                File::C,
                File::D,
                File::E,
                File::F,
                File::G,
                File::H,
            ],
            jumbled
        )
    }
}
