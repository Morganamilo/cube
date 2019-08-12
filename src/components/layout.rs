
const FLU: usize = 0;
const FU: usize = 1;
const FRU: usize = 2;
const FL: usize = 3;
const F: usize = 4;
const FR: usize = 5;
const FLD: usize = 6;
const FD: usize = 7;
const FRD: usize = 8;

const LU: usize = 9;
const U: usize = 10;
const RU: usize = 11;
const L: usize = 12;
const R: usize = 13;
const LD: usize = 14;
const D: usize = 15;
const RD: usize = 16;

const BLU: usize = 17;
const BU: usize = 18;
const BRU: usize = 19;
const BL: usize = 20;
const B: usize = 21;
const BR: usize = 22;
const BLD: usize = 23;
const BD: usize = 24;
const BRD: usize = 25;

const LAYOUT: [usize; 26] = [
    FLU, FU, FRU, FL, F, FR, FLD, FD, FRD, LU, U, RU, L, R, LD, D, RD, BLU, BU, BRU, BL, B, BR,
    BLD, BD, BRD,
];

const FRONT: [usize; 9] = [FLU, FU, FRU, FL, F, FR, FLD, FD, FRD];

#[derive(Debug)]
pub struct Layout {
    layout: [usize; 26],
}

impl Layout {
    pub fn new() -> Layout {
        Layout { layout: LAYOUT }
    }

    pub fn turn_up(&mut self) {
        let c = self.layout.clone();
        let l = &mut self.layout;

        l[BLU] = c[FLU];
        l[BRU] = c[BLU];
        l[FRU] = c[BRU];
        l[FLU] = c[FRU];

        l[BU] = c[LU];
        l[RU] = c[BU];
        l[FU] = c[RU];
        l[LU] = c[FU];

        for piece in &FRONT {}
    }

    pub fn turn_front(&mut self) {
        let c = self.layout.clone();
        let l = &mut self.layout;

        l[FLU] = c[FLD];
        l[FRU] = c[FLU];
        l[FRD] = c[FRU];
        l[FLD] = c[FRD];

        l[FU] = c[FL];
        l[FR] = c[FU];
        l[FD] = c[FR];
        l[FL] = c[FD];
    }

    pub fn front(&self) -> [usize; 9] {
        let l = &self.layout;

        [
            l[FLU], l[FU], l[FRU], l[FL], l[F], l[FR], l[FLD], l[FD], l[FRD],
        ]
    }

    pub fn up(&self) -> [usize; 9] {
        let l = &self.layout;

        [
            l[BLU], l[BU], l[BRU], l[LU], l[U], l[RU], l[FLU], l[FU], l[FRU],
        ]
    }
}
