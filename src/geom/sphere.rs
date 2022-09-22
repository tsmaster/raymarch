// geom/sphere.rs

use crate::math::Vec3f;

pub struct Sphere {
    pub center: Vec3f,
    pub r: f32,
}

impl Sphere {
    pub fn dist(self, point: &Vec3f) -> f32 {
	self.center.dist(point) - self.r
    }
}
