use super::Matrix;
use std::{
    mem::MaybeUninit,
    ops::{Rem, RemAssign},
};

impl<T: Clone + Rem<Output = T>, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn rem_ref(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for (y, row) in result.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item = self.0[y][x].clone() % other.0[y][x].clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + RemAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn rem_ref_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, item) in row.iter_mut().enumerate() {
                *item %= other.0[y][x].clone();
            }
        }
        self
    }
}

impl<T: Clone + Rem<Output = T>, const WIDTH: usize, const HEIGHT: usize> Rem
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.rem_ref(&rhs)
    }
}

impl<T: Clone + RemAssign, const WIDTH: usize, const HEIGHT: usize> RemAssign
    for Matrix<T, WIDTH, HEIGHT>
{
    fn rem_assign(&mut self, rhs: Self) {
        self.rem_ref_self(&rhs);
    }
}
