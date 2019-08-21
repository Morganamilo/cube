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
const C: usize = 13;
const R: usize = 14;
const LD: usize = 15;
const D: usize = 16;
const RD: usize = 17;

const BLU: usize = 18;
const BU: usize = 19;
const BRU: usize = 20;
const BL: usize = 21;
const B: usize = 22;
const BR: usize = 23;
const BLD: usize = 24;
const BD: usize = 25;
const BRD: usize = 26;

static LAYOUT: [usize; 27] = [
    FLU, FU, FRU, FL, F, FR, FLD, FD, FRD, LU, U, RU, L, C, R, LD, D, RD, BLU, BU, BRU, BL, B, BR,
    BLD, BD, BRD,
];

pub static FRONT: Face = Face {
    corners: [FLU, FLD, FRD, FRU],
    edges: [FU, FL, FD, FR],
    center: F,
};

pub static BACK: Face = Face {
    corners: [BLU, BRU, BRD, BLD],
    edges: [BU, BR, BD, BL],
    center: B,
};

pub static UP: Face = Face {
    corners: [BLU, FLU, FRU, BRU],
    edges: [BU, LU, FU, RU],
    center: U,
};

pub static DOWN: Face = Face {
    corners: [BLD, BRD, FRD, FLD],
    edges: [BD, RD, FD, LD],
    center: D,
};

pub static LEFT: Face = Face {
    corners: [BLU, BLD, FLD, FLU],
    edges: [BL, LD, FL, LU],
    center: L,
};

pub static RIGHT: Face = Face {
    corners: [BRU, FRU, FRD, BRD],
    edges: [BR, RU, FR, RD],
    center: R,
};

pub static MIDDLE: Face = Face {
    corners: [FU, BU, BD, FD],
    edges: [F, U, B, D],
    center: C,
};

pub static EQUATOR: Face = Face {
    corners: [FR, FL, BL, BR],
    edges: [F, L, B, R],
    center: C,
};

pub static STANDING: Face = Face {
    corners: [RU, LU, LD, RD],
    edges: [U, L, D, R],
    center: C,
};

#[derive(Copy, Clone)]
pub struct Face {
    corners: [usize; 4],
    edges: [usize; 4],
    center: usize,
}

impl Face {
    fn layer(&self) -> [usize; 9] {
        let c = &self.corners;
        let e = &self.edges;
        [c[0], c[1], c[2], c[3], e[0], e[1], e[2], e[3], self.center]
    }

    pub fn reverse(&self, rev: bool) -> Self {
        if !rev {
            *self
        } else {
            let c = &self.corners;
            let e = &self.edges;

            Face {
                corners: [c[0], c[3], c[2], c[1]],
                edges: [e[0], e[3], e[2], e[1]],
                center: self.center,
            }
        }
    }
}

pub struct Layout {
    layout: [usize; 27],
}

impl Layout {
    pub fn new() -> Layout {
        Layout { layout: LAYOUT }
    }

    pub fn turn(&mut self, face: &Face) {
        let cp = self.layout;
        let l = &mut self.layout;

        let c = &face.corners;
        let e = &face.edges;

        for piece in 0..4 {
            l[c[piece]] = cp[c[(piece + 1) % c.len()]];
            l[e[piece]] = cp[e[(piece + 1) % e.len()]];
        }
    }

    pub fn layer(&self, face: &Face) -> [usize; 9] {
        let f = face.layer();
        let l = &self.layout;

        let mut out = [0; 9];
        for i in 0..out.len() {
            out[i] = l[f[i]];
        }
        out
    }
}
