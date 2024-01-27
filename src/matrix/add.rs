use super::Matrix;
use std::{mem::MaybeUninit, ops::{Add, AddAssign}};

impl<T: Clone + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn add_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() + data.clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + AddAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn add_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] += data.clone();
            }
        }
        self
    }
}

impl<T: Clone + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize> Add<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn add(self, rhs: T) -> Self::Output {
        self.add_scale(&rhs)
    }
}

impl<T: Clone + AddAssign, const WIDTH: usize, const HEIGHT: usize> AddAssign<T>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn add_assign(&mut self, rhs: T) {
        self.add_scale_self(&rhs);
    }
}

impl<T: Clone + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn add_ref(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result: [[T; WIDTH]; HEIGHT] = unsafe { MaybeUninit::uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result[y][x] = self.0[y][x].clone() + other.0[y][x].clone();
            }
        }
        Self(result)
    }
}

impl<T: Clone + AddAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn add_ref_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] += other.0[y][x].clone();
            }
        }
        self
    }
}

impl<T: Clone + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize> Add
    for Matrix<T, WIDTH, HEIGHT>
{
    type Output = Matrix<T, WIDTH, HEIGHT>;

    fn add(self, rhs: Self) -> Self::Output {
        self.add_ref(&rhs)
    }
}

impl<T: Clone + AddAssign, const WIDTH: usize, const HEIGHT: usize> AddAssign
    for Matrix<T, WIDTH, HEIGHT>
{
    fn add_assign(&mut self, rhs: Self) {
        self.add_ref_self(&rhs);
    }
}
