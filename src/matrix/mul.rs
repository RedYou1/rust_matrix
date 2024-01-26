use super::Matrix;
use std::{
    iter::Sum,
    ops::{AddAssign, Mul, MulAssign, Range},
};

impl<T: Default + Copy + Mul<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn mul_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] * *data;
            }
        }
        result
    }
}

impl<T: Copy + MulAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn mul_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] *= *data;
            }
        }
        self
    }
}

impl<T: Default + Copy + Sum<T> + Mul<Output = T>, const COMMUN: usize, const HEIGHT: usize>
    Matrix<T, COMMUN, HEIGHT>
{
    pub fn mul<const OWIDTH: usize>(
        &self,
        other: &Matrix<T, OWIDTH, COMMUN>,
    ) -> Matrix<T, OWIDTH, HEIGHT> {
        let mut result = Matrix::<T, OWIDTH, HEIGHT>::default();
        for ry in 0..HEIGHT {
            for rx in 0..OWIDTH {
                result.0[ry][rx] = Range {
                    start: 0,
                    end: COMMUN,
                }
                .map(|c| self.0[ry][c] * other.0[c][rx])
                .sum();
            }
        }
        result
    }
}

impl<T: Default + Copy + AddAssign + Mul<Output = T>, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn mul_self(&mut self, other: &Matrix<T, SIDE, SIDE>) -> &mut Self {
        for ry in 0..SIDE {
            let mut temp = [T::default(); SIDE];
            for rx in 0..SIDE {
                for c in 0..SIDE {
                    temp[rx] += self.0[ry][c] * other.0[c][rx];
                }
            }
            self.0[ry] = temp;
        }
        self
    }
}
