use super::Matrix;
use std::{mem::MaybeUninit, ops::{Div, DivAssign}};

impl<T: Clone + Div<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn div_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() / data.clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + DivAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn div_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] /= data.clone();
            }
        }
        self
    }
}

impl<T: Clone + Div<Output = T>, const WIDTH: usize, const HEIGHT: usize> Div<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn div(self, rhs: T) -> Self::Output {
        self.div_scale(&rhs)
    }
}

impl<T: Clone + DivAssign, const WIDTH: usize, const HEIGHT: usize> DivAssign<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn div_assign(&mut self, rhs: T) {
        self.div_scale_self(&rhs);
    }
}
