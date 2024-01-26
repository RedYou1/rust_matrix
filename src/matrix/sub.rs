use super::Matrix;
use std::ops::{Sub, SubAssign};

impl<T: Default + Copy + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn sub_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] - *data;
            }
        }
        result
    }
}

impl<T: Copy + SubAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] -= *data;
            }
        }
        self
    }
}

impl<T: Default + Copy + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn sub(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] - other.0[y][x];
            }
        }
        result
    }
}

impl<T: Copy + SubAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] -= other.0[y][x];
            }
        }
        self
    }
}