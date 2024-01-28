use super::Matrix;
use std::{
    iter::Sum,
    mem::MaybeUninit,
    ops::{AddAssign, Mul, MulAssign, Neg, Range},
};

impl<T: Clone + Mul<Output = T>, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn mul_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for (y, row) in result.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = self.0[y][x].clone() * data.clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + MulAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn mul_scale_self(&mut self, data: &T) -> &mut Self {
        for row in &mut self.0 {
            for item in row {
                *item *= data.clone();
            }
        }
        self
    }
}

impl<T: Clone + Mul<Output = T>, const WIDTH: usize, const HEIGHT: usize> Mul<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_scale(&rhs)
    }
}

impl<T: Clone + MulAssign, const WIDTH: usize, const HEIGHT: usize> MulAssign<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn mul_assign(&mut self, rhs: T) {
        self.mul_scale_self(&rhs);
    }
}

impl<T: Clone + Sum<T> + Mul<Output = T>, const COMMUN: usize, const HEIGHT: usize>
    Matrix<T, COMMUN, HEIGHT>
{
    pub fn mul_ref<const OWIDTH: usize>(
        &self,
        other: &Matrix<T, OWIDTH, COMMUN>,
    ) -> Matrix<T, OWIDTH, HEIGHT> {
        let mut result: [[T; OWIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for (ry, row) in result.iter_mut().enumerate() {
            for (rx, item) in row.iter_mut().enumerate() {
                *item = Range {
                    start: 0,
                    end: COMMUN,
                }
                .map(|c| self.0[ry][c].clone() * other.0[c][rx].clone())
                .sum();
            }
        }
        Matrix(result)
    }
}

impl<T: Default + Copy + AddAssign + Mul<Output = T>, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn mul_ref_self(&mut self, other: &Matrix<T, SIDE, SIDE>) -> &mut Self {
        for ry in 0..SIDE {
            let mut temp = [T::default(); SIDE];
            for (rx, item) in temp.iter_mut().enumerate() {
                for c in 0..SIDE {
                    *item += self.0[ry][c] * other.0[c][rx];
                }
            }
            self.0[ry] = temp;
        }
        self
    }
}

impl<
        T: Copy + Sum<T> + Mul<Output = T>,
        const COMMUN: usize,
        const HEIGHT: usize,
        const OWIDTH: usize,
    > Mul<Matrix<T, OWIDTH, COMMUN>> for Matrix<T, COMMUN, HEIGHT>
{
    type Output = Matrix<T, OWIDTH, HEIGHT>;

    fn mul(self, rhs: Matrix<T, OWIDTH, COMMUN>) -> Self::Output {
        self.mul_ref(&rhs)
    }
}

impl<T: Default + Copy + AddAssign + Mul<Output = T>, const SIDE: usize> MulAssign
    for Matrix<T, SIDE, SIDE>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_ref_self(&rhs);
    }
}

macro_rules! impl_neg_matrix {
    ($($ty:ty),*) => {
        $(
        impl<const WIDTH: usize, const HEIGHT: usize> Neg for Matrix<$ty, WIDTH, HEIGHT> {
            type Output = Matrix<$ty, WIDTH, HEIGHT>;

            fn neg(self) -> Self::Output {
                self * (-1 as $ty)
            }
        }
        )*
    };
}

impl_neg_matrix!(isize, i8, i16, i32, i64, i128, f32, f64);
