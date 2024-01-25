use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const WIDTH: usize, const HEIGHT: usize>([[T; WIDTH]; HEIGHT]);

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Default
    for Matrix<T, WIDTH, HEIGHT>
{
    fn default() -> Self {
        Self([[T::default(); WIDTH]; HEIGHT])
    }
}

impl<T: Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn new_unit(data: T) -> Self {
        Self([[data; WIDTH]; HEIGHT])
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn new(data: [[T; WIDTH]; HEIGHT]) -> Self {
        Self(data)
    }
}

impl<T: Default + Copy + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn add_scale(&self, data: &T) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] + *data;
            }
        }
        result
    }
}

impl<T: Copy + AddAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn add_scale_self(&mut self, data: &T) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] += *data;
            }
        }
        self
    }
}

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

impl<T: Default + Copy + Add<Output = T>, const WIDTH: usize, const HEIGHT: usize>
    Matrix<T, WIDTH, HEIGHT>
{
    pub fn add(&self, other: &Matrix<T, WIDTH, HEIGHT>) -> Matrix<T, WIDTH, HEIGHT> {
        let mut result = Matrix::<T, WIDTH, HEIGHT>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[y][x] + other.0[y][x];
            }
        }
        result
    }
}

impl<T: Copy + AddAssign, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn add_self(&mut self, other: &Matrix<T, WIDTH, HEIGHT>) -> &mut Self {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.0[y][x] += other.0[y][x];
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

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn transpose(&self) -> Matrix<T, HEIGHT, WIDTH> {
        let mut result = Matrix::<T, HEIGHT, WIDTH>::default();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = self.0[x][y];
            }
        }
        result
    }
}

impl<T: Copy, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn transpose_self(&mut self) -> &mut Self {
        for d in 0..SIDE - 1 {
            for opp in d + 1..SIDE {
                (self.0[d][opp], self.0[opp][d]) = (self.0[opp][d], self.0[d][opp]);
            }
        }
        self
    }
}
