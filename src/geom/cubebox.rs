// geom/cubebox.rs
//
// the SDF for a box, centered at the origin, with half-dimensions provided
// confusing name to avoid conflicting with Rust's boxes
//
// I could use a namespace to disambiguate.

use crate::math::Vec3f;
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
pub struct CubeBox {
    pub half_size: Vec3f,
}

impl SDF for CubeBox {
    fn dist(&self, point: &Vec3f) -> f32 {
	let q:Vec3f = point.abs() - self.half_size;
	q.max(&Vec3f::ZERO).len() + f32::min(f32::max(q.x, f32::max(q.y, q.z)), 0.0)
    }
}
