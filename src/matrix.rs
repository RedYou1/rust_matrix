mod add;
mod div;
mod mul;
mod sub;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T, const WIDTH: usize, const HEIGHT: usize>([[T; WIDTH]; HEIGHT]);

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Default
    for Matrix<T, WIDTH, HEIGHT>
{
    fn default() -> Self {
        Self([[T::default(); WIDTH]; HEIGHT])
    }
}

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn default_box() -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = T::default();
            }
        }
        result
    }
}

impl<T: Copy, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn new_unit(data: T) -> Self {
        Self([[data; WIDTH]; HEIGHT])
    }

    pub fn new_box_unit(data: T) -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                result.0[y][x] = data;
            }
        }
        result
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn new(data: [[T; WIDTH]; HEIGHT]) -> Self {
        Self(data)
    }
}

impl<T: Default + Copy, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn new_scale(data: T) -> Self {
        let mut result = Matrix::<T, SIDE, SIDE>::default();
        for i in 0..SIDE {
            result.0[i][i] = data;
        }
        result
    }
    pub fn new_box_scale(data: T) -> Box<Self> {
        let mut result = unsafe { Box::<Self>::new_uninit().assume_init() };
        for y in 0..SIDE {
            for x in 0..SIDE {
                result.0[y][x] = if x == y { data } else { T::default() };
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

impl<T: Default + PartialEq, const SIDE: usize> Matrix<T, SIDE, SIDE> {
    pub fn is_trig_inf(&self) -> bool {
        for y in 0..SIDE {
            for x in y + 1..SIDE {
                if self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_trig_sup(&self) -> bool {
        for y in 1..SIDE {
            for x in 0..y {
                if self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_diagonale(&self) -> bool {
        for y in 0..SIDE {
            for x in 0..SIDE {
                if y != x && self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_scale(&self, value: T) -> bool {
        for y in 0..SIDE {
            for x in 0..SIDE {
                if y == x {
                    if self.0[y][x] != value {
                        return false;
                    }
                } else if self.0[y][x] != T::default() {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsRef<[[T; WIDTH]; HEIGHT]>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn as_ref(&self) -> &[[T; WIDTH]; HEIGHT] {
        &self.0
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> AsMut<[[T; WIDTH]; HEIGHT]>
    for Matrix<T, WIDTH, HEIGHT>
{
    fn as_mut(&mut self) -> &mut [[T; WIDTH]; HEIGHT] {
        &mut self.0
    }
}
