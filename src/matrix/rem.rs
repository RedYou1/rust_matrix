use super::Matrix;
use std::{
    mem::MaybeUninit,
    ops::{Rem, RemAssign},
};

impl<T: Clone + Rem<Output = T>, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn rem_ref(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() % other.0[y][x].clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + RemAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn rem_ref_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] %= other.0[y][x].clone();
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
