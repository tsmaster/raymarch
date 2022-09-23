// geom/sphere.rs

use crate::math::Vec3f;
use crate::sdf::SDF;

pub struct Sphere {
    pub center: Vec3f,
    pub r: f32,
}

impl SDF for Sphere {
    fn dist(&self, point: &Vec3f) -> f32 {
	self.center.dist(point) - self.r
    }
}
