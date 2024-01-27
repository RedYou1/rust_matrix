use super::Matrix;
use std::ops::{Div, DivAssign};

impl<T: Default + Copy + Div<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn div_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] / *data;
            }
        }
        result
    }
}

impl<T: Copy + DivAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn div_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] /= *data;
            }
        }
        self
    }
}

impl<T: Default + Copy + Div<Output = T>, const WIDTH: usize, const HEIGHT: usize> Div<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn div(self, rhs: T) -> Self::Output {
        self.div_scale(&rhs)
    }
}

impl<T: Copy + DivAssign, const WIDTH: usize, const HEIGHT: usize> DivAssign<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn div_assign(&mut self, rhs: T) {
        self.div_scale_self(&rhs);
    }
}
