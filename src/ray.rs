use crate::vec3::{Point, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origem: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origem: Point, direction: Vec3) -> Ray {
        Ray { origem, direction }
    }

    pub fn point_at(&self, t: f32) -> Point {
        self.origem + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3;

    use super::*;

    #[test]
    fn test_basic() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.5, 1.5));
        let destiny = ray.point_at(2.0);
        let expected = Point::new(2.0, 1.0, 3.0);
        vec3::cmp_vec3(destiny, expected);
    }
}
