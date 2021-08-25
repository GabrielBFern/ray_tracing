mod vec3;
use ray::Ray;
use vec3::Vec3;

use crate::vec3::Color;
mod ray;

fn main() {
    let x = Vec3::new(0.1, 0.2, 0.3);
    let _y: Color = x;
    let _ray = Ray::new(x, x);
}
