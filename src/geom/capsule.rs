// geom/capsule.rs
//
// the SDF for capsules

use crate::math::{Vec3f, clamp};
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
pub struct Capsule {
    pub a: Vec3f,
    pub b: Vec3f,
    pub r: f32
}

impl SDF for Capsule {
    fn dist(&self, point: &Vec3f) -> f32 {
	let pa : Vec3f = *point - self.a;
	let ba : Vec3f = self.b - self.a;

	let h : f32 = clamp(pa.dot(&ba) / ba.dot(&ba), 0.0, 1.0);
	(pa - ba * h).len() - self.r
    }
}
