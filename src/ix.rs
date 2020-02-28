use std::convert::TryInto;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(Copy, Clone)]
pub struct Ix2 {
    pub x: usize,
    pub y: usize,
    w: usize,
    h: usize,
}

impl Ix2 {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Ix2 {
        assert!(x < w);
        assert!(y < h);
        Ix2 { x, y, w, h }
    }

    pub fn try_new(x: isize, y: isize, w: usize, h: usize) -> Option<Ix2> {
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
        VECT.iter().map(move |r| c + r.clone())
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
        VECT.iter().map(move |r| c + r.clone())
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

pub struct Mat<T>(pub Vec<Vec<T>>);

impl<T> Mat<T> {
    pub fn new(mat: Vec<Vec<T>>) -> Mat<T> {
        Mat(mat)
    }

    pub fn ix(&self, x: usize, y: usize) -> Ix2 {
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

impl<T> Index<Ix2> for Mat<T> {
    type Output = T;

    fn index(&self, ix: Ix2) -> &Self::Output {
        &self.0[ix.y][ix.x]
    }
}

impl<T> IndexMut<Ix2> for Mat<T> {
    fn index_mut(&mut self, ix: Ix2) -> &mut Self::Output {
        &mut self.0[ix.y][ix.x]
    }
}
