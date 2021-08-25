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
}
