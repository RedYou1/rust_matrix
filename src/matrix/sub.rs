use super::Matrix;
use std::{
    mem::MaybeUninit,
    ops::{Sub, SubAssign},
};

impl<T: Clone + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() - data.clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + SubAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] -= data.clone();
            }
        }
        self
    }
}

impl<T: Clone + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize> Sub<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn sub(self, rhs: T) -> Self::Output {
        self.sub_scale(&rhs)
    }
}

impl<T: Clone + SubAssign, const WIDTH: usize, const HEIGHT: usize> SubAssign<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn sub_assign(&mut self, rhs: T) {
        self.sub_scale_self(&rhs);
    }
}

impl<T: Clone + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_ref(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() - other.0[y][x].clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + SubAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn sub_ref_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] -= other.0[y][x].clone();
            }
        }
        self
    }
}

impl<T: Clone + Sub<Output = T>, const WIDTH: usize, const HEIGHT: usize> Sub
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_ref(&rhs)
    }
}

impl<T: Clone + SubAssign, const WIDTH: usize, const HEIGHT: usize> SubAssign
    for Matrix<T, WIDTH, HEIGHT>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_ref_self(&rhs);
    }
}
