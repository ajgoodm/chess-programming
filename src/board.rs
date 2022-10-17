use std::collections::HashSet;

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
}

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
#[derive(Debug, PartialEq, Eq, Hash)]
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
}
