use std::slice::Iter;
use std::{collections::HashMap, hash::Hash};

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
        for s in RANK_1_SQUARES.iter() {
            m.insert(s.clone(), Rank::First);
        }
        for s in RANK_2_SQUARES.iter() {
            m.insert(s.clone(), Rank::Second);
        }
        for s in RANK_3_SQUARES.iter() {
            m.insert(s.clone(), Rank::Third);
        }
        for s in RANK_4_SQUARES.iter() {
            m.insert(s.clone(), Rank::Fourth);
        }
        for s in RANK_5_SQUARES.iter() {
            m.insert(s.clone(), Rank::Fifth);
        }
        for s in RANK_6_SQUARES.iter() {
            m.insert(s.clone(), Rank::Sixth);
        }
        for s in RANK_7_SQUARES.iter() {
            m.insert(s.clone(), Rank::Seventh);
        }
        for s in RANK_8_SQUARES.iter() {
            m.insert(s.clone(), Rank::Eigth);
        }
        m
    };

    static ref SQUARE_TO_FILE: HashMap<Square, File> = {
        let mut m: HashMap<Square, File> = HashMap::new();
        for s in FILE_A_SQUARES.iter() {
            m.insert(s.clone(), File::A);
        }
        for s in FILE_B_SQUARES.iter() {
            m.insert(s.clone(), File::B);
        }
        for s in FILE_C_SQUARES.iter() {
            m.insert(s.clone(), File::C);
        }
        for s in FILE_D_SQUARES.iter() {
            m.insert(s.clone(), File::D);
        }
        for s in FILE_E_SQUARES.iter() {
            m.insert(s.clone(), File::E);
        }
        for s in FILE_F_SQUARES.iter() {
            m.insert(s.clone(), File::F);
        }
        for s in FILE_G_SQUARES.iter() {
            m.insert(s.clone(), File::G);
        }
        for s in FILE_H_SQUARES.iter() {
            m.insert(s.clone(), File::H);
        }
        m
    };

    static ref NE_A8_DIAG_SQUARES: Vec<Square> = vec![Square::A8];
    static ref NE_A7_DIAG_SQUARES: Vec<Square> = vec![
        Square::A7, Square::B8,
    ];
    static ref NE_A6_DIAG_SQUARES: Vec<Square> = vec![
        Square::A6, Square::B7, Square::C8,
    ];
    static ref NE_A5_DIAG_SQUARES: Vec<Square> = vec![
        Square::A5, Square::B6, Square::C7, Square::D8,
    ];
    static ref NE_A4_DIAG_SQUARES: Vec<Square> = vec![
        Square::A4, Square::B5, Square::C6, Square::D7, Square::E8,
    ];
    static ref NE_A3_DIAG_SQUARES: Vec<Square> = vec![
        Square::A3, Square::B4, Square::C5, Square::D6, Square::E7, Square::F8,
    ];
    static ref NE_A2_DIAG_SQUARES: Vec<Square> = vec![
        Square::A2, Square::B3, Square::C4, Square::D5, Square::E6, Square::F7, Square::G8,
    ];
    static ref NE_A1_DIAG_SQUARES: Vec<Square> = vec![
        Square::A1, Square::B2, Square::C3, Square::D4, Square::E5, Square::F6, Square::G7, Square::H8,
    ];
    static ref NE_B1_DIAG_SQUARES: Vec<Square> = vec![
        Square::B1, Square::C2, Square::D3, Square::E4, Square::F5, Square::G6, Square::H7,
    ];
    static ref NE_C1_DIAG_SQUARES: Vec<Square> = vec![
        Square::C1, Square::D2, Square::E3, Square::F4, Square::G5, Square::H6,
    ];
    static ref NE_D1_DIAG_SQUARES: Vec<Square> = vec![
        Square::D1, Square::E2, Square::F3, Square::G4, Square::H5,
    ];
    static ref NE_E1_DIAG_SQUARES: Vec<Square> = vec![
        Square::E1, Square::F2, Square::G3, Square::H4,
    ];
    static ref NE_F1_DIAG_SQUARES: Vec<Square> = vec![
        Square::F1, Square::G2, Square::H3,
    ];
    static ref NE_G1_DIAG_SQUARES: Vec<Square> = vec![
        Square::G1, Square::H2,
    ];
    static ref NE_H1_DIAG_SQUARES: Vec<Square> = vec![
        Square::H1,
    ];


    static ref SQUARE_TO_NE_DIAGONAL: HashMap<Square, NEDiagonal> = {
        let mut m: HashMap<Square, NEDiagonal> = HashMap::new();
        for s in NE_A8_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A8);
        }
        for s in NE_A7_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A7);
        }
        for s in NE_A6_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A6);
        }
        for s in NE_A5_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A5);
        }
        for s in NE_A4_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A4);
        }
        for s in NE_A3_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A3);
        }
        for s in NE_A2_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A2);
        }
        for s in NE_A1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::A1);
        }
        for s in NE_B1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::B1);
        }
        for s in NE_C1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::C1);
        }
        for s in NE_D1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::D1);
        }
        for s in NE_E1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::E1);
        }
        for s in NE_F1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::F1);
        }
        for s in NE_G1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::G1);
        }
        for s in NE_H1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NEDiagonal::H1);
        }
        m
    };

    static ref NW_H8_DIAG_SQUARES: Vec<Square> = vec![Square::H8];
    static ref NW_H7_DIAG_SQUARES: Vec<Square> = vec![
        Square::H7, Square::G8,
    ];
    static ref NW_H6_DIAG_SQUARES: Vec<Square> = vec![
        Square::H6, Square::G7, Square::F8,
    ];
    static ref NW_H5_DIAG_SQUARES: Vec<Square> = vec![
        Square::H5, Square::G6, Square::F7, Square::E8,
    ];
    static ref NW_H4_DIAG_SQUARES: Vec<Square> = vec![
        Square::H4, Square::G5, Square::F6, Square::E7, Square::D8,
    ];
    static ref NW_H3_DIAG_SQUARES: Vec<Square> = vec![
        Square::H3, Square::G4, Square::F5, Square::E6, Square::D7, Square::C8,
    ];
    static ref NW_H2_DIAG_SQUARES: Vec<Square> = vec![
        Square::H2, Square::G3, Square::F4, Square::E5, Square::D6, Square::C7, Square::B8,
    ];
    static ref NW_H1_DIAG_SQUARES: Vec<Square> = vec![
        Square::H1, Square::G2, Square::F3, Square::E4, Square::D5, Square::C6, Square::B7, Square::A8,
    ];
    static ref NW_G1_DIAG_SQUARES: Vec<Square> = vec![
        Square::G1, Square::F2, Square::E3, Square::D4, Square::C5, Square::B6, Square::A7,
    ];
    static ref NW_F1_DIAG_SQUARES: Vec<Square> = vec![
        Square::F1, Square::E2, Square::D3, Square::C4, Square::B5, Square::A6,
    ];
    static ref NW_E1_DIAG_SQUARES: Vec<Square> = vec![
        Square::E1, Square::D2, Square::C3, Square::B4, Square::A5,
    ];
    static ref NW_D1_DIAG_SQUARES: Vec<Square> = vec![
        Square::D1, Square::C2, Square::B3, Square::A4,
    ];
    static ref NW_C1_DIAG_SQUARES: Vec<Square> = vec![
        Square::C1, Square::B2, Square::A3,
    ];
    static ref NW_B1_DIAG_SQUARES: Vec<Square> = vec![
        Square::B1, Square::A2,
    ];
    static ref NW_A1_DIAG_SQUARES: Vec<Square> = vec![
        Square::A1,
    ];

    static ref SQUARE_TO_NW_DIAGONAL: HashMap<Square, NWDiagonal> = {
        let mut m: HashMap<Square, NWDiagonal> = HashMap::new();
        for s in NE_A8_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H8);
        }
        for s in NE_A7_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H7);
        }
        for s in NE_A6_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H6);
        }
        for s in NE_A5_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H5);
        }
        for s in NE_A4_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H4);
        }
        for s in NE_A3_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H3);
        }
        for s in NE_A2_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H2);
        }
        for s in NE_A1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::H1);
        }
        for s in NE_B1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::G1);
        }
        for s in NE_C1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::F1);
        }
        for s in NE_D1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::E1);
        }
        for s in NE_E1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::D1);
        }
        for s in NE_F1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::C1);
        }
        for s in NE_G1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::B1);
        }
        for s in NE_H1_DIAG_SQUARES.iter() {
            m.insert(s.clone(), NWDiagonal::A1);
        }
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

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        static RANKS: [Rank; 8] = [
            Rank::First,
            Rank::Second,
            Rank::Third,
            Rank::Fourth,
            Rank::Fifth,
            Rank::Sixth,
            Rank::Seventh,
            Rank::Eigth,
        ];
        RANKS.iter()
    }
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

impl File {
    pub fn iterator() -> Iter<'static, File> {
        static FILES: [File; 8] = [
            File::A,
            File::B,
            File::C,
            File::D,
            File::E,
            File::F,
            File::G,
            File::H,
        ];
        FILES.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum NEDiagonal {
    A8,
    A7,
    A6,
    A5,
    A4,
    A3,
    A2,
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
}

impl NEDiagonal {
    pub fn iterator() -> Iter<'static, NEDiagonal> {
        static NE_DIAGONALS: [NEDiagonal; 15] = [
            NEDiagonal::A8,
            NEDiagonal::A7,
            NEDiagonal::A6,
            NEDiagonal::A5,
            NEDiagonal::A4,
            NEDiagonal::A3,
            NEDiagonal::A2,
            NEDiagonal::A1,
            NEDiagonal::B1,
            NEDiagonal::C1,
            NEDiagonal::D1,
            NEDiagonal::E1,
            NEDiagonal::F1,
            NEDiagonal::G1,
            NEDiagonal::H1,
        ];
        NE_DIAGONALS.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum NWDiagonal {
    H8,
    H7,
    H6,
    H5,
    H4,
    H3,
    H2,
    H1,
    G1,
    F1,
    E1,
    D1,
    C1,
    B1,
    A1,
}

impl NWDiagonal {
    pub fn iterator() -> Iter<'static, NWDiagonal> {
        static NW_DIAGONALS: [NWDiagonal; 15] = [
            NWDiagonal::H8,
            NWDiagonal::H7,
            NWDiagonal::H6,
            NWDiagonal::H5,
            NWDiagonal::H4,
            NWDiagonal::H3,
            NWDiagonal::H2,
            NWDiagonal::H1,
            NWDiagonal::G1,
            NWDiagonal::F1,
            NWDiagonal::E1,
            NWDiagonal::D1,
            NWDiagonal::C1,
            NWDiagonal::B1,
            NWDiagonal::A1,
        ];
        NW_DIAGONALS.iter()
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

    pub fn from_rf_str(file_rank: &str) -> Square {
        assert_eq!(file_rank.len(), 2);
        let mut chars = file_rank.chars();
        let file = match chars.next().unwrap() {
            'a' | 'A' => File::A,
            'b' | 'B' => File::B,
            'c' | 'C' => File::C,
            'd' | 'D' => File::D,
            'e' | 'E' => File::E,
            'f' | 'F' => File::F,
            'g' | 'G' => File::G,
            'h' | 'H' => File::H,
            _ => panic!("unexpected square str {}", file_rank),
        };

        let rank = match chars.next().unwrap() {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eigth,
            _ => panic!("unexpected square str {}", file_rank),
        };

        Square::from_file_rank(&file, &rank)
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

    pub fn file(&self) -> File {
        *SQUARE_TO_FILE.get(self).unwrap()
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

    pub fn ne_diagonal(&self) -> NEDiagonal {
        *SQUARE_TO_NE_DIAGONAL.get(self).unwrap()
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
    use std::collections::HashSet;

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

    #[test]
    fn test_ne_diagonals() {
        let all_squares: HashSet<Square> = vec![
            NE_A8_DIAG_SQUARES.iter().cloned(),
            NE_A7_DIAG_SQUARES.iter().cloned(),
            NE_A6_DIAG_SQUARES.iter().cloned(),
            NE_A5_DIAG_SQUARES.iter().cloned(),
            NE_A4_DIAG_SQUARES.iter().cloned(),
            NE_A3_DIAG_SQUARES.iter().cloned(),
            NE_A2_DIAG_SQUARES.iter().cloned(),
            NE_A1_DIAG_SQUARES.iter().cloned(),
            NE_B1_DIAG_SQUARES.iter().cloned(),
            NE_C1_DIAG_SQUARES.iter().cloned(),
            NE_D1_DIAG_SQUARES.iter().cloned(),
            NE_E1_DIAG_SQUARES.iter().cloned(),
            NE_F1_DIAG_SQUARES.iter().cloned(),
            NE_G1_DIAG_SQUARES.iter().cloned(),
            NE_H1_DIAG_SQUARES.iter().cloned(),
        ]
        .into_iter()
        .flatten()
        .collect();
        assert_eq!(all_squares.len(), 64);
    }

    #[test]
    fn test_nw_diagonals() {
        let all_squares: HashSet<Square> = vec![
            NW_H8_DIAG_SQUARES.iter().cloned(),
            NW_H7_DIAG_SQUARES.iter().cloned(),
            NW_H6_DIAG_SQUARES.iter().cloned(),
            NW_H5_DIAG_SQUARES.iter().cloned(),
            NW_H4_DIAG_SQUARES.iter().cloned(),
            NW_H3_DIAG_SQUARES.iter().cloned(),
            NW_H2_DIAG_SQUARES.iter().cloned(),
            NW_A1_DIAG_SQUARES.iter().cloned(),
            NW_B1_DIAG_SQUARES.iter().cloned(),
            NW_C1_DIAG_SQUARES.iter().cloned(),
            NW_D1_DIAG_SQUARES.iter().cloned(),
            NW_E1_DIAG_SQUARES.iter().cloned(),
            NW_F1_DIAG_SQUARES.iter().cloned(),
            NW_G1_DIAG_SQUARES.iter().cloned(),
            NW_H1_DIAG_SQUARES.iter().cloned(),
        ]
        .into_iter()
        .flatten()
        .collect();
        assert_eq!(all_squares.len(), 64);
    }
}
