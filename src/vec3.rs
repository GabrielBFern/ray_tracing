use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Sub, SubAssign};
use std::ops::{Index, IndexMut};

pub type Color = Vec3;
pub type Point = Vec3;

pub type Inside = f32;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    data: [Inside; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> Inside {
        self.data[0]
    }
    pub fn y(&self) -> Inside {
        self.data[1]
    }
    pub fn z(&self) -> Inside {
        self.data[2]
    }

    pub fn r(&self) -> Inside {
        self.data[0]
    }
    pub fn g(&self) -> Inside {
        self.data[1]
    }
    pub fn b(&self) -> Inside {
        self.data[2]
    }

    pub fn dot(&self, other: &Self) -> Inside {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]
    }

    pub fn length_squared(&self) -> Inside {
        self.dot(self)
    }
    pub fn length(&self) -> Inside {
        Inside::sqrt(self.length_squared())
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self.data[1] * other.data[2] - self.data[2] * other.data[1],
                self.data[2] * other.data[0] - self.data[0] * other.data[2],
                self.data[0] * other.data[1] - self.data[1] * other.data[0],
            ],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = Inside;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [
                self.data[0] * rhs.data[0],
                self.data[1] * rhs.data[1],
                self.data[2] * rhs.data[2],
            ],
        }
    }
}

impl Mul<Inside> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Inside) -> Self::Output {
        Vec3 {
            data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs],
        }
    }
}

impl Mul<Vec3> for Inside {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            data: [rhs.data[0] * self, rhs.data[1] * self, rhs.data[2] * self],
        }
    }
}

impl Div<Inside> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Inside) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data = [
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        ];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.data = [
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
        ];
    }
}

impl DivAssign<Inside> for Vec3 {
    fn div_assign(&mut self, rhs: Inside) {
        let c = 1.0 / rhs;
        self.data = [self.data[0] * c, self.data[1] * c, self.data[2] * c];
    }
}

impl MulAssign<Inside> for Vec3 {
    fn mul_assign(&mut self, rhs: Inside) {
        self.data = [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.data = [
            self.data[0] * rhs.data[0],
            self.data[1] * rhs.data[1],
            self.data[2] * rhs.data[2],
        ];
    }
}

impl Not for Vec3 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Vec3 {
            data: [-self.data[0], -self.data[1], -self.data[2]],
        }
    }
}
