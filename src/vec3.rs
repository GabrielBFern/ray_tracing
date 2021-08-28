use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Not, Sub, SubAssign};
use std::ops::{Index, IndexMut};

pub type Color = Vec3;
pub type Point = Vec3;

pub type Inside = f32;

#[derive(Debug, Copy, Clone)]
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

    pub fn cross(&self, other: &Vec3) -> Vec3 {
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

#[cfg_attr(coverage, no_coverage)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color() {
        let rgb = Color::new(100.0, 200.0, 255.0);
        cmp_float(rgb.r(), 100.0);
        cmp_float(rgb.g(), 200.0);
        cmp_float(rgb.b(), 255.0);
    }

    #[test]
    fn test_coordinates() {
        let pos = Point::new(1.0, 2.0, 3.0);
        cmp_float(pos.x(), 1.0);
        cmp_float(pos.y(), 2.0);
        cmp_float(pos.z(), 3.0);
    }

    #[test]
    fn test_index() {
        let mut rgb = Color::new(100.0, 200.0, 255.0);
        rgb[0] = 10.0;
        cmp_float(rgb[0], 10.0);
    }

    #[test]
    fn test_derives() {
        let pos = Point::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{:?}", pos), "Vec3 { data: [1.0, 2.0, 3.0] }");
    }

    #[test]
    fn test_translate() {
        let pos1 = Point::new(1.0, 2.0, 3.0);
        let pos2 = Point::new(1.0, 1.0, 1.0);
        let expected = Point::new(2.0, 3.0, 4.0);

        cmp_vec3(pos1 + pos2, expected);

        let expected = Point::new(0.0, 1.0, 2.0);
        cmp_vec3(pos1 - pos2, expected);
    }

    #[test]
    fn test_mul() {
        let pos1 = Point::new(3.0, 2.0, 3.0);
        let pos2 = Point::new(0.5, 3.0, 1.0);
        let expected = Point::new(1.5, 6.0, 3.0);

        cmp_vec3(pos1 * pos2, expected);
    }

    #[test]
    fn test_div() {
        let pos1 = Point::new(4.0, 2.0, 12.0);
        let expected = Point::new(1.0, 0.5, 3.0);

        cmp_vec3(pos1 / 4.0, expected);
    }

    #[test]
    fn test_mul_fat() {
        let pos1 = Point::new(3.0, 1.5, 3.0);
        let expected = Point::new(9.0, 4.5, 9.0);

        cmp_vec3(pos1 * 3.0, expected);
    }

    #[test]
    fn test_not() {
        let pos1 = Point::new(3.0, -1.5, 3.0);
        let expected = Point::new(-3.0, 1.5, -3.0);

        cmp_vec3(!pos1, expected);
    }

    #[test]
    fn test_dot() {
        let pos1 = Point::new(3.0, -1.5, 3.0);
        let pos2 = Point::new(0.5, 3.0, 1.0);
        let expected = 1.5 - 4.5 + 3.0;
        cmp_float(pos1.dot(&pos2), expected);
    }

    #[test]
    fn test_length_squared() {
        let pos1 = Point::new(3.0, -1.5, 3.0);
        let expected = 9.0 + 2.25 + 9.0;
        cmp_float(pos1.length_squared(), expected);
    }

    #[test]
    fn test_length() {
        let pos1 = Point::new(3.0, -1.5, 3.0);
        let expected = 4.5;
        cmp_float(pos1.length(), expected);
    }

    #[test]
    fn test_length_cross() {
        let pos1 = Point::new(3.0, 4.0, 2.0);
        let pos2 = Point::new(1.0, 5.0, 6.0);
        let expected = Point::new(14.0, -16.0, 11.0);
        cmp_vec3(pos1.cross(&pos2), expected);
    }

    #[test]
    fn test_unit() {
        let pos1 = Point::new(3.0, 4.0, 2.0);
        let expected = Point::new(0.557086, 0.74278134, 0.3713906);
        cmp_vec3(pos1.unit_vector(), expected);
    }

    #[test]
    fn test_assigns() {
        let mut pos1 = Point::new(3.0, 2.0, 4.0);
        pos1 += Point::new(3.0, 2.0, 4.0);
        let expected = Point::new(6.0, 4.0, 8.0);
        cmp_vec3(pos1, expected);

        let mut pos1 = Point::new(3.0, 2.0, 4.0);
        pos1 -= Point::new(3.0, 2.0, 5.0);
        let expected = Point::new(0.0, 0.0, -1.0);
        cmp_vec3(pos1, expected);

        let mut pos1 = Point::new(3.0, 2.0, 4.0);
        pos1 *= Point::new(3.0, 2.0, 4.0);
        let expected = Point::new(9.0, 4.0, 16.0);
        cmp_vec3(pos1, expected);

        let mut pos1 = Point::new(3.0, 2.0, 4.0);
        pos1 *= 3.0;
        let expected = Point::new(9.0, 6.0, 12.0);
        cmp_vec3(pos1, expected);

        let mut pos1 = Point::new(3.0, 2.0, 4.0);
        pos1 /= 2.0;
        let expected = Point::new(1.5, 1.0, 2.0);
        cmp_vec3(pos1, expected);
    }
}

#[cfg(test)]
#[cfg_attr(coverage, no_coverage)]
pub fn cmp_float(left: f32, right: f32) {
    if cmp_float_inside(left, right) {
        panic!(
            r#"assertion failed: `(left approx_eq right)`
left: `{:?}`,
right: `{:?}`"#,
            left, right,
        );
    }
}

#[cfg(test)]
#[cfg_attr(coverage, no_coverage)]
pub fn cmp_vec3(left: Vec3, right: Vec3) {
    if cmp_float_inside(left.x(), right.x())
        || cmp_float_inside(left.y(), right.y())
        || cmp_float_inside(left.z(), right.z())
    {
        panic!(
            r#"assertion failed: `(left approx_eq right)`
left: `{:?}`,
right: `{:?}`"#,
            left, right,
        );
    }
}

#[cfg(test)]
fn cmp_float_inside(left: f32, right: f32) -> bool {
    (left - right).abs() > f32::EPSILON
}
