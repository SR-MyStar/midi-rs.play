use crate::scale::Scale::*;
use Note::*;

#[derive(Copy, Clone)]
pub enum Note {
    N = 0,

    L1 = G3 as isize,
    L2 = A3 as isize,
    L3 = B3 as isize,
    L5 = D3 as isize,
    L6 = E3 as isize,
    L7 = F3 as isize,

    M1 = G4 as isize,
    M2 = A4 as isize,
    M3 = B4 as isize,
    M5 = D4 as isize,
    M6 = E4 as isize,

    H1 = G5 as isize,
    H2 = A5 as isize,
    H3 = B5 as isize,
    H5 = D5 as isize,
    H5s = D5s as isize,
    H6 = E5 as isize,

    Rest = 0xFF,
}

pub const STEP5_NOTES1: &[[[Note; 4]; 4]] = &[
    [
        [Rest, N, N, N],
        [Rest, N, N, N],
        [Rest, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H6, N, N, N],
        [H5, N, H3, H2],
        [H2, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H3, N],
        [M6, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H3, N, N, N],
        [H2, N, H3, H5],
        [H5, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H3, N],
        [H3, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H6, N, N, N],
        [H5, N, H3, H2],
        [H2, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H5, N],
        [H5, N, H3, N],
        [H2, N, H1, N],
    ],
    [
        [M6, N, M6, N],
        [M5, N, M6, H3],
        [H3, N, M6, N],
        [M6, N, M5, N],
    ],
    [
        [M5, N, N, M6],
        [M6, N, M6, N],
        [Rest, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H6, N, N, N],
        [H5, N, H3, H2],
        [H2, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H3, N],
        [M6, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H3, N, N, N],
        [H2, N, H3, H5],
        [H5, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H3, N],
        [H3, N, N, N],
        [M3, N, M5, N],
    ],
    [
        [M6, N, N, N],
        [H6, N, N, N],
        [H5, N, H3, H2],
        [H2, N, H1, N],
    ],
    [
        [H2, N, H1, H2],
        [H2, N, H3, N],
        [H3, N, H2, N],
        [H1, N, H2, N],
    ],
    [
        [M6, N, M6, N],
        [M5, N, M6, H3],
        [H3, N, M6, N],
        [M6, N, M5, N],
    ],
    [
        [M5, N, N, M6],
        [M6, N, M6, N],
        [Rest, N, N, N],
        [M2, N, M1, N],
    ],
    [
        [M6, N, M6, N],
        [M5, N, M6, H3],
        [H3, N, H6, N],
        [H5, N, H3, N],
    ],
    [
        [H5, N, N, H5s],
        [H5, N, H6, N],
        [H6, N, N, N],
        [Rest, N, N, N],
    ],
];

pub const STEP5_NOTES2: &[[[Note; 4]; 4]] = &[
    [
        [Rest, N, N, N],
        [Rest, N, N, N],
        [Rest, N, N, N],
        [L1, N, L2, N],
    ],
    [
        [L3, N, N, N],
        [M3, N, N, N],
        [M2, N, M1, L7],
        [L7, N, L6, N],
    ],
    [
        [L7, N, L6, L7],
        [L7, N, M1, N],
        [L6, N, N, N],
        [L1, N, L2, N],
    ],
    [
        [L3, N, N, N],
        [M1, N, N, N],
        [L1, N, M1, M2],
        [M2, N, L5, N],
    ],
    [
        [L7, N, L6, L7],
        [L7, N, M1, N],
        [M1, N, N, N],
        [L1, N, L2, N],
    ],
    [
        [L3, N, N, N],
        [M3, N, N, N],
        [M2, N, M1, L7],
        [L7, N, L5, N],
    ],
    [
        [L7, N, L5, L7],
        [L7, N, M2, N],
        [M2, N, M1, N],
        [L7, N, L5, N],
    ],
    [
        [M1, N, M1, N],
        [L7, M1, N, M1],
        [M1, N, M1, N],
        [M1, N, L7, N],
    ],
    [
        [L7, N, N, M1],
        [M1, N, M1, N],
        [Rest, N, N, N],
        [L1, N, L2, N],
    ],
    [
        [L3, N, N, N],
        [M3, N, N, N],
        [M2, N, M1, L7],
        [L7, N, L5, N],
    ],
    [
        [L7, N, L5, L7],
        [L7, N, M1, N],
        [M1, N, L6, N],
        [L5, N, L6, N],
    ],
    [
        [L3, N, L3, N],
        [L2, N, L3, M1],
        [M1, N, M3, N],
        [M2, N, M1, N],
    ],
    [
        [M2, N, N, M3],
        [M3, N, M3, N],
        [M3, N, N, N],
        [Rest, N, N, N],
    ],
    [
        [L3, N, L3, N],
        [L2, N, L3, M1],
        [M1, N, M3, N],
        [M2, N, M1, N],
    ],
    [
        [M2, N, N, M3],
        [M3, N, M3, N],
        [M3, N, N, N],
        [Rest, N, N, N],
    ],
];
