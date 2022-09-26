// geom/torus.rs
//
// the SDF for a torus, centered at the origin
// r1 is the radius from the center across the hole plus r2
// r2 is the radius of the bready cylinder

use crate::math::{Vec3f, Vec2f};
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
pub struct Torus {
    pub r1: f32,
    pub r2: f32
}

impl SDF for Torus {
    fn dist(&self, point: &Vec3f) -> f32 {
	let xz = Vec2f {
	    x: point.x,
	    y: point.z
	};
	
	let q = Vec2f {
	    x: xz.len() - self.r1,
	    y: point.y
	};

	q.len() - self.r2
    }
}
