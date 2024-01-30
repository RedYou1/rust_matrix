use std::{mem::MaybeUninit, ops::Not};

mod add;
mod div;
mod mul;
mod rem;
mod sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const WIDTH: usize, const HEIGHT: usize>([[T; WIDTH]; HEIGHT]);

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Default
    for Matrix<T, WIDTH, HEIGHT>
{
    fn default() -> Self {
        Self([[T::default(); WIDTH]; HEIGHT])
    }
}

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn default_box() -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for y in &mut result.as_mut().0 {
            for x in y {
                *x = T::default();
            }
        }
        result
    }
}

impl<T: Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub const fn new_unit(data: T) -> Self {
        Self([[data; WIDTH]; HEIGHT])
    }

    pub fn new_box_unit(data: T) -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for y in &mut result.as_mut().0 {
            for x in y {
                *x = data;
            }
        }
        result
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub const fn new(data: [[T; WIDTH]; HEIGHT]) -> Self {
        Self(data)
    }

    pub fn new_fn<Data: Fn(usize, usize) -> T>(data: Data) -> Self {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for (y, row) in result.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = data(x, y);
            }
        }
        Self(result)
    }

    pub fn new_box_fn<Data: Fn(usize, usize) -> T>(data: Data) -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for (y, row) in result.0.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = data(x, y);
            }
        }
        result
    }
}

impl<T: Default + Copy, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn new_scale(data: T) -> Self {
        let mut result = Matrix::<T, SIDE, SIDE>::default();
        for i in 0..SIDE {
            result.0[i][i] = data;
        }
        result
    }

    pub fn new_box_scale(data: T) -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for (y, row) in result.0.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = if x == y { data } else { T::default() };
            }
        }
        result
    }
}

impl<T: Clone, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn transpose(&self) -> Matrix<T, HEIGHT, WIDTH> {
        let mut result: [[T; HEIGHT]; WIDTH] = unsafe { MaybeUninit::uninit().assume_init() };
        for (x, row) in result.iter_mut().enumerate() {
            for (y, item) in row.iter_mut().enumerate() {
                *item = self.0[y][x].clone();
            }
        }
        Matrix(result)
    }
}

impl<T: Clone, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn transpose_self(&mut self) -> &mut Self {
        for d in 0..SIDE - 1 {
            for opp in d + 1..SIDE {
                (self.0[d][opp], self.0[opp][d]) = (self.0[opp][d].clone(), self.0[d][opp].clone());
            }
        }
        self
    }
}

impl<T: Default + PartialEq, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn is_trig_inf(&self) -> bool {
        for y in 0..SIDE {
            for x in y + 1..SIDE {
                if self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_trig_sup(&self) -> bool {
        for y in 1..SIDE {
            for x in 0..y {
                if self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_diagonale(&self) -> bool {
        for (y, row) in self.0.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if y != x && *item != T::default() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_scale(&self, value: &T) -> bool {
        for (y, row) in self.0.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if y == x {
                    if *item != *value {
                        return false;
                    }
                } else if *item != T::default() {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsRef<[[T; WIDTH]; HEIGHT]>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn as_ref(&self) -> &[[T; WIDTH]; HEIGHT] {
        &self.0
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsMut<[[T; WIDTH]; HEIGHT]>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn as_mut(&mut self) -> &mut [[T; WIDTH]; HEIGHT] {
        &mut self.0
    }
}

impl<T: Clone + Not<Output = T>, const WIDTH: usize, const HEIGHT: usize> Not
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn not(self) -> Self::Output {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for (y, row) in result.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = !self.0[y][x].clone();
            }
        }
        Matrix(result)
    }
}

macro_rules! impl_inverse_matrix {
    ($($ty:ty),*) => {
        $(
        impl Matrix<$ty, 2, 2> {
            pub fn inverse(&self) -> Option<Self> {
                let [[a, b], [c, d]] = self.0;
                let mut opp = a * d - b * c;
                if opp == 0.0 {
                    return None;
                }
                opp = 1.0 / opp;
                Some(Self([[opp * d, opp * -b], [opp * -c, opp * a]]))
            }
        }
        )*
    };
}
impl_inverse_matrix!(f32, f64);
