#![cfg_attr(coverage, feature(no_coverage))]

use ray::Ray;
use vec3::{Color, Point};

mod ray;
mod vec3;

#[cfg_attr(coverage, no_coverage)]
fn main() {
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.5, 1.5));
    let _destiny = ray.point_at(2.0);
    let _color = Color::new(100.0, 200.0, 150.0);
}
