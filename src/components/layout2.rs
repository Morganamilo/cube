use std::fmt;
use std::mem::swap;
use std::ops::Neg;

static SOLVED: Layout = Layout::new();

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

#[derive(PartialEq, Eq)]
pub struct Layout {
    layout: [[Sticker; 9]; 6],
}

impl Layout {
    pub const fn new() -> Self {
        Layout {
            layout: [
                [Sticker::White; 9],
                [Sticker::Yellow; 9],
                [Sticker::Green; 9],
                [Sticker::Blue; 9],
                [Sticker::Red; 9],
                [Sticker::Orange; 9],
            ],
        }
    }

    pub fn solved(&self) -> bool {
        self == &SOLVED
    }

    pub fn rotate(&mut self, face: Face) {
        self.rotate_face(face);
        let surrounding = face.surrounding();
        for n in 0..3 {
            self.swap_sides(face, surrounding[n], surrounding[n + 1]);
        }
    }

    pub fn m(&mut self) {
        self.swap_parts(Face::Front, &[1, 8, 5], Face::Up, &[1, 8, 5]);
        self.swap_parts(Face::Up, &[1, 8, 5], Face::Back, &[5, 8, 1]);
        self.swap_parts(Face::Back, &[5, 8, 1], Face::Down, &[1, 8, 5]);
    }

    pub fn e(&mut self) {
        self.swap_parts(Face::Front, &[7, 8, 3], Face::Left, &[7, 8, 3]);
        self.swap_parts(Face::Left, &[7, 8, 3], Face::Back, &[7, 8, 3]);
        self.swap_parts(Face::Back, &[7, 8, 3], Face::Right, &[7, 8, 3]);
    }

    pub fn s(&mut self) {
        self.swap_parts(Face::Up, &[7, 8, 3], Face::Left, &[1, 8, 5]);
        self.swap_parts(Face::Left, &[1, 8, 5], Face::Down, &[3, 8, 7]);
        self.swap_parts(Face::Down, &[3, 8, 7], Face::Right, &[5, 8, 1]);
    }


    fn swap_parts(&mut self, mut f1: Face, i1: &[usize], mut f2: Face, i2: &[usize]) {
        let (i1, i2) = if f1 as usize > f2 as usize {
            swap(&mut f1, &mut f2);
            (i2, i1)
        } else {
            (i1, i2)
        };

        let (l, r) = self.layout.split_at_mut(f2 as usize);
        let (l, r) = (&mut l[f1 as usize], &mut r[0]);

        for (&i, &j) in i1.iter().zip(i2) {
            swap(&mut l[i], &mut r[j]);
        }
    }

    pub fn x(&mut self) {
        self.m();
        self.m();
        self.m();
        self.rotate(Face::Right);
        self.rotate(Face::Left);
        self.rotate(Face::Left);
        self.rotate(Face::Left);
    }

    fn y(&mut self) {
        let faces = [Face::Front, Face::Left, Face::Back, Face::Right];

        for n in (0..3).map(|n| n as usize) {
            self.swap_faces(faces[n], faces[n+1]);
        }

        self.rotate_face(Face::Up);

        self.rotate_face(Face::Down);
        self.rotate_face(Face::Down);
        self.rotate_face(Face::Down);
    }

    fn z(&mut self) {
        let faces = [Face::Up, Face::Left, Face::Down, Face::Right];

        for n in (0..3).map(|n| n as usize) {
            self.swap_faces(faces[n], faces[n+1]);
        }


        self.rotate_face(Face::Up);

        self.rotate_face(Face::Left);
        self.rotate_face(Face::Left);
        self.rotate_face(Face::Left);

        self.rotate_face(Face::Front);

        self.rotate_face(Face::Back);
        self.rotate_face(Face::Back);
        self.rotate_face(Face::Back);
    }

    fn swap_faces(&mut self, mut f1: Face, mut f2: Face) {
        let l = &mut self.layout;
        if f1 as usize > f2 as usize {
            swap(&mut f1, &mut f2);
        }
        let (l, r) = l.split_at_mut(f2 as usize);
        let l = &mut l[f1 as usize];
        let r = &mut r[0];
        swap(l, r);
    }

    fn swap_sides(&mut self, fface: Face, mut lface: Face, mut rface: Face) {
        if lface as usize > rface as usize {
            swap(&mut lface, &mut rface);
        }

        let li = fface.intersect(lface) as usize;
        let ri = fface.intersect(rface) as usize;
        let (l, r) = self.layout.split_at_mut(rface as usize);
        let (l, r) = (&mut l[lface as usize], &mut r[0]);

        for n in 0..3 {
            swap(&mut l[(li + n) % 8], &mut r[(ri + n) % 8]);
        }
    }

    fn rotate_face(&mut self, face: Face) {
        let face = &mut self.layout[face as usize];

        for n in (0..=4).rev().step_by(2) {
            face.swap(n, n + 2);
            face.swap(n + 1, n + 3);
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

        //l.rotate(Face::Up);
        /*l.rotate(Face::Up);
        l.rotate(Face::Down);
        l.rotate(Face::Down);
        l.rotate(Face::Left);
        l.rotate(Face::Left);
        l.rotate(Face::Right);
        l.rotate(Face::Right);
        l.rotate(Face::Front);
        l.rotate(Face::Front);
        l.rotate(Face::Back);
        l.rotate(Face::Back);*/
        l.s();
        l.s();
        l.m();
        l.m();
        l.e();
        l.e();

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
