use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref FILE_A_SQUARES: HashSet<Square> = [
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
    static ref FILE_B_SQUARES: HashSet<Square> = [
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
    static ref FILE_C_SQUARES: HashSet<Square> = [
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
    static ref FILE_D_SQUARES: HashSet<Square> = [
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
    static ref FILE_E_SQUARES: HashSet<Square> = [
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
    static ref FILE_F_SQUARES: HashSet<Square> = [
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
    static ref FILE_G_SQUARES: HashSet<Square> = [
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
    static ref FILE_H_SQUARES: HashSet<Square> = [
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
    static ref RANK_1_SQUARES: HashSet<Square> = [
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
    static ref RANK_2_SQUARES: HashSet<Square> = [
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
    static ref RANK_3_SQUARES: HashSet<Square> = [
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
    static ref RANK_4_SQUARES: HashSet<Square> = [
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
    static ref RANK_5_SQUARES: HashSet<Square> = [
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
    static ref RANK_6_SQUARES: HashSet<Square> = [
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
    static ref RANK_7_SQUARES: HashSet<Square> = [
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
    static ref RANK_8_SQUARES: HashSet<Square> = [
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
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl Rank {
    pub fn squares(&self) -> &HashSet<Square> {
        match self {
            Rank::First => &*RANK_1_SQUARES,
            Rank::Second => &*RANK_2_SQUARES,
            Rank::Third => &*RANK_3_SQUARES,
            Rank::Fourth => &*RANK_4_SQUARES,
            Rank::Fifth => &*RANK_5_SQUARES,
            Rank::Sixth => &*RANK_6_SQUARES,
            Rank::Seventh => &*RANK_7_SQUARES,
            Rank::Eigth => &*RANK_8_SQUARES,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

impl File {
    pub fn squares(&self) -> &HashSet<Square> {
        match self {
            File::A => &*FILE_A_SQUARES,
            File::B => &*FILE_B_SQUARES,
            File::C => &*FILE_C_SQUARES,
            File::D => &*FILE_D_SQUARES,
            File::E => &*FILE_E_SQUARES,
            File::F => &*FILE_F_SQUARES,
            File::G => &*FILE_G_SQUARES,
            File::H => &*FILE_H_SQUARES,
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_squares() {
        let rank_1: Rank = Rank::First;
        assert_eq!(rank_1.squares(), &*RANK_1_SQUARES);
    }

    #[test]
    fn test_file_squares() {
        let rank_1: Rank = Rank::First;
        assert_eq!(rank_1.squares(), &*RANK_1_SQUARES);
    }

    #[test]
    fn test_square_from_file_rank() {
        assert_eq!(Square::from_file_rank(&File::A, &Rank::First), Square::A1);
        assert_eq!(Square::from_file_rank(&File::G, &Rank::Fifth), Square::G5);
    }
}
