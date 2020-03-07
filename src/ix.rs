use std::convert::TryInto;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Ix2 {
    pub x: usize,
    pub y: usize,
    w: usize,
    h: usize,
}

impl Ix2 {
    pub fn new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Ix2 {
        let x = x.try_into().ok().unwrap();
        let y = y.try_into().ok().unwrap();

        assert!(x >= 0);
        assert!(x < w as isize);
        assert!(y >= 0);
        assert!(y < h as isize);

        Ix2 {
            x: x as usize,
            y: y as usize,
            w,
            h,
        }
    }

    pub fn try_new<T: TryInto<isize>>(x: T, y: T, w: usize, h: usize) -> Option<Ix2> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;

        if x >= 0 && x < w as isize && y >= 0 && y < h as isize {
            Some(Ix2 {
                x: x as usize,
                y: y as usize,
                w,
                h,
            })
        } else {
            None
        }
    }

    pub fn try_add<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Ix2> {
        let dx = rhs.0.try_into().ok()?;
        let dy = rhs.1.try_into().ok()?;
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;
        Self::try_new(x, y, self.w, self.h)
    }

    pub fn try_sub<T: TryInto<isize>>(&self, rhs: (T, T)) -> Option<Ix2> {
        let dx = rhs.0.try_into().ok()?;
        let dy = rhs.1.try_into().ok()?;
        let x = self.x as isize - dx;
        let y = self.y as isize - dy;
        Self::try_new(x, y, self.w, self.h)
    }

    pub fn neighbor4(&self) -> impl Iterator<Item = Ix2> {
        const VECT: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];
        let c = self.clone();
        VECT.iter().filter_map(move |r| c.try_add(r.clone()))
    }

    pub fn neighbor8(&self) -> impl Iterator<Item = Ix2> {
        const VECT: &[(isize, isize)] = &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let c = self.clone();
        VECT.iter().filter_map(move |r| c.try_add(r.clone()))
    }
}

impl<T: TryInto<isize>> Add<(T, T)> for Ix2 {
    type Output = Ix2;

    fn add(self, rhs: (T, T)) -> Self::Output {
        self.try_add(rhs).unwrap()
    }
}

impl<T: TryInto<isize>> AddAssign<(T, T)> for Ix2 {
    fn add_assign(&mut self, rhs: (T, T)) {
        *self = self.clone() + rhs;
    }
}

impl<T: TryInto<isize>> Sub<(T, T)> for Ix2 {
    type Output = Ix2;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        self.try_sub(rhs).unwrap()
    }
}

impl<T: TryInto<isize>> SubAssign<(T, T)> for Ix2 {
    fn sub_assign(&mut self, rhs: (T, T)) {
        *self = self.clone() - rhs;
    }
}

#[derive(Debug, Clone)]
pub struct Board<T>(pub Vec<Vec<T>>);

impl<T> Board<T> {
    pub fn new(mat: Vec<Vec<T>>) -> Self {
        Self(mat)
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn ix<Ix: TryInto<isize>>(&self, x: Ix, y: Ix) -> Ix2 {
        let w = self.0[0].len();
        let h = self.0.len();
        Ix2::new(x, y, w, h)
    }

    pub fn get<Ix: TryInto<isize>>(&self, x: Ix, y: Ix) -> Option<&T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.0[0].len() as isize;
        let h = self.0.len() as isize;
        if x >= 0 && x < w && y >= 0 && y < h {
            Some(&self.0[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_mut<Ix: TryInto<isize>>(&mut self, x: Ix, y: Ix) -> Option<&mut T> {
        let x = x.try_into().ok()?;
        let y = y.try_into().ok()?;
        let w = self.0[0].len() as isize;
        let h = self.0.len() as isize;
        if x >= 0 && x < w && y >= 0 && y < h {
            Some(&mut self.0[y as usize][x as usize])
        } else {
            None
        }
    }
}

impl<T: PartialEq> Board<T> {
    pub fn find(&self, v: T) -> Option<Ix2> {
        let w = self.0[0].len();
        let h = self.0.len();
        for y in 0..h {
            for x in 0..w {
                if self.0[y][x] == v {
                    return Some(self.ix(x, y));
                }
            }
        }
        None
    }
}

impl<T> Index<Ix2> for Board<T> {
    type Output = T;

    fn index(&self, ix: Ix2) -> &Self::Output {
        &self.0[ix.y][ix.x]
    }
}

impl<T> IndexMut<Ix2> for Board<T> {
    fn index_mut(&mut self, ix: Ix2) -> &mut Self::Output {
        &mut self.0[ix.y][ix.x]
    }
}
