use std::fmt;
use std::mem::swap;
use std::ops::Neg;

static SLICE_UP: [usize; 3] = [1, 8, 5];
static SLICE_RIGHT: [usize; 3] = [7, 8, 3];
static SLICE_DOWN: [usize; 3] = [5, 8, 1];
static SLICE_LEFT: [usize; 3] = [3, 8, 7];

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up = 0,
    Right = 2,
    Down = 4,
    Left = 6,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Sticker {
    Red,
    Orange,
    Blue,
    Green,
    White,
    Yellow,
}

impl fmt::Debug for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let letter = match self {
            Self::Red => "R",
            Self::Orange => "O",
            Self::Blue => "B",
            Self::Green => "G",
            Self::White => "W",
            Self::Yellow => "Y",
        };
        f.write_str(letter)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Face {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    fn intersect(self, face: Face) -> Direction {
        match self {
            Self::Up => Direction::Up,
            Self::Down => Direction::Down,
            Self::Front => match face {
                Self::Up => Direction::Down,
                Self::Right => Direction::Left,
                Self::Down => Direction::Up,
                Self::Left => Direction::Right,
                _ => unreachable!(),
            },
            Self::Back => -Self::intersect(Self::Front, face),
            Self::Left => match face {
                Self::Up => Direction::Left,
                Self::Front => Direction::Left,
                Self::Down => Direction::Left,
                Self::Back => Direction::Right,
                _ => unreachable!(),
            },
            Self::Right => -Self::intersect(Self::Left, face),
        }
    }

    fn surrounding(self) -> [Face; 4] {
        match self {
            Self::Up => [Self::Front, Self::Right, Self::Back, Self::Left],
            Self::Left => [Self::Up, Self::Back, Self::Down, Self::Front],
            Self::Front => [Self::Up, Self::Left, Self::Down, Self::Right],
            Self::Down => [Self::Front, Self::Left, Self::Back, Self::Right],
            Self::Right => [Self::Up, Self::Front, Self::Down, Self::Back],
            Self::Back => [Self::Up, Self::Right, Self::Down, Self::Left],
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Slice {
    Middle,
    Equator,
    Standing,
}

impl Slice {
    fn intersect(self, face: Face) -> [usize; 3] {
        match self {
            Self::Equator => SLICE_RIGHT,
            Self::Middle => match face {
                Face::Front => SLICE_DOWN,
                Face::Up => SLICE_DOWN,
                Face::Back => SLICE_UP,
                Face::Down => SLICE_UP,
                _ => unreachable!(),
            },
            Self::Standing => match face {
                Face::Up => SLICE_RIGHT,
                Face::Left => SLICE_DOWN,
                Face::Down => SLICE_LEFT,
                Face::Right => SLICE_UP,
                _ => unreachable!(),
            },
        }
    }

    fn surrounding(self) -> [Face; 4] {
        match self {
            Self::Middle => Face::Left.surrounding(),
            Self::Equator => Face::Down.surrounding(),
            Self::Standing => Face::Front.surrounding(),
        }
    }
}

pub enum Rotate {
    X,
    Y,
    Z,
}

pub struct Layout {
    layout: [[Sticker; 9]; 6],
}

impl Layout {
    pub const fn new() -> Self {
        Layout {
            layout: [
                [Sticker::Yellow; 9],
                [Sticker::White; 9],
                [Sticker::Orange; 9],
                [Sticker::Red; 9],
                [Sticker::Blue; 9],
                [Sticker::Green; 9],
            ],
        }
    }

    pub fn solved(&self) -> bool {
        for face in &self.layout {
            if !face.iter().skip(1).all(|&x| x == face[0]) {
                return false;
            }
        }
        true
    }

    pub fn face(&mut self, face: Face, rev: bool) {
        self.rotate_face(face, rev);
        let surrounding = face.surrounding();
        if rev {
            for n in (0..3).rev() {
                self.swap_sides(face, surrounding[n], surrounding[n + 1]);
            }
        } else {
            for n in 0..3 {
                self.swap_sides(face, surrounding[n], surrounding[n + 1]);
            }
        }
    }

    pub fn slice(&mut self, slice: Slice, rev: bool) {
        let surrounding = slice.surrounding();
        if rev {
            for n in (0..3).rev() {
                self.swap_slice(slice, surrounding[n], surrounding[n + 1]);
            }
        } else {
            for n in 0..3 {
                self.swap_slice(slice, surrounding[n], surrounding[n + 1]);
            }
        }
    }

    pub fn rotate(&mut self, rot: Rotate, rev: bool) {
        match rot {
            Rotate::X => self.x(rev),
            Rotate::Y => self.y(rev),
            Rotate::Z => self.z(rev),
        }
    }

    fn x(&mut self, rev: bool) {
        self.slice(Slice::Middle, !rev);
        self.face(Face::Right, rev);
        self.face(Face::Left, !rev);
    }

    fn y(&mut self, rev: bool) {
        self.slice(Slice::Equator, !rev);
        self.face(Face::Up, rev,);
        self.face(Face::Down, !rev);
    }

    fn z(&mut self, rev: bool) {
        self.slice(Slice::Standing, rev);
        self.face(Face::Front, rev);
        self.face(Face::Back, !rev);
    }

    fn swap_pieces<'a>(
        &mut self,
        mut f1: Face,
        mut i1: &'a [usize],
        mut f2: Face,
        mut i2: &'a [usize],
    ) {
        if f1 as usize > f2 as usize {
            swap(&mut f1, &mut f2);
            swap(&mut i1, &mut i2);
        }

        let (l, r) = self.layout.split_at_mut(f2 as usize);
        let (l, r) = (&mut l[f1 as usize], &mut r[0]);

        for (&i, &j) in i1.iter().zip(i2) {
            swap(&mut l[i % 8], &mut r[j % 8]);
        }
    }

    fn swap_slice(&mut self, slice: Slice, f1: Face, f2: Face) {
        let i1 = slice.intersect(f1);
        let i2 = slice.intersect(f2);
        self.swap_pieces(f1, &i1, f2, &i2);
    }

    fn swap_faces(&mut self, f1: Face, f2: Face) {
        self.layout.swap(f1 as usize, f2 as usize);
    }

    fn swap_sides(&mut self, fface: Face, f1: Face, f2: Face) {
        let i1 = fface.intersect(f1) as usize;
        let i2 = fface.intersect(f2) as usize;
        self.swap_pieces(f1, &[i1, i1 + 1, i1 + 2], f2, &[i2, i2 + 1, i2 + 2]);
    }

    fn rotate_face(&mut self, face: Face, rev: bool) {
        let face = &mut self.layout[face as usize];
        if rev {
            for n in 0..7 {
                face.swap(n, n + 2);
            }
        } else {
            for n in (0..7).rev() {
                face.swap(n, n + 2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UP: usize = 0;
    const DOWN: usize = 1;
    const LEFT: usize = 2;
    const RIGHT: usize = 3;
    const FRONT: usize = 4;
    const BACK: usize = 5;

    const P0: usize = 0;
    const P1: usize = 1;
    const P2: usize = 2;
    const P3: usize = 7;
    const P4: usize = 8;
    const P5: usize = 3;
    const P6: usize = 6;
    const P7: usize = 5;
    const P8: usize = 4;
    static PINDEX: [usize; 9] = [P0, P1, P2, P3, P4, P5, P6, P7, P8];

    #[test]
    fn print() {
        let mut l = Layout::new();

        /*l.rotate(Face::Up, false);
        l.rotate(Face::Right, false);
        l.rotate(Face::Up, true);
        l.rotate(Face::Left, true);
        l.rotate(Face::Up, false);
        l.rotate(Face::Right, true);
        l.rotate(Face::Up, true);
        l.rotate(Face::Left, false);*/

        //l.rotate(Face::Up);
        //l.rotate(Face::Down);
        //l.rotate(Face::Down);
        //l.rotate(Face::Left);
        //l.rotate(Face::Left);
        //l.rotate(Face::Right);
        //l.rotate(Face::Right);
        //l.rotate(Face::Front);
        //l.rotate(Face::Front);
        //l.rotate(Face::Back);
        //l.rotate(Face::Back);*/
        //l.s();
        //l.s();
        //l.m();
        //l.m();
        //l.e();
        //l.e();
        l.slice(Slice::Middle, false);
        l.slice(Slice::Middle, false);
        l.slice(Slice::Equator, false);
        l.slice(Slice::Equator, false);
        l.slice(Slice::Standing, false);
        l.slice(Slice::Standing, false);
        println!("{}", l.solved());

        let l = &l.layout;

        for y in 0..3 {
            print!("        ");
            for x in 0..3 {
                print!("{:?} ", l[UP][PINDEX[x + (3 * y)]]);
            }
            println!();
        }
        println!();

        for y in 0..3 {
            for x in 0..3 {
                print!("{:?} ", l[LEFT][PINDEX[x + (3 * y)]]);
            }
            print!("  ");
            for x in 0..3 {
                print!("{:?} ", l[FRONT][PINDEX[x + (3 * y)]]);
            }
            print!("  ");
            for x in 0..3 {
                print!("{:?} ", l[RIGHT][PINDEX[x + (3 * y)]]);
            }
            print!("  ");
            for x in 0..3 {
                print!("{:?} ", l[BACK][PINDEX[x + (3 * y)]]);
            }
            println!();
        }

        println!();

        for y in 0..3 {
            print!("        ");
            for x in 0..3 {
                print!("{:?} ", l[DOWN][PINDEX[x + (3 * y)]]);
            }
            println!();
        }
    }
}
