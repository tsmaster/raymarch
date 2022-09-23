// geom/plane.rs

use crate::math::Vec3f;
use crate::sdf::SDF;

// a plane filling space below a given z level
pub struct ZPlusPlane {
    pub z: f32,
}

impl SDF for ZPlusPlane {
    fn dist(&self, point: &Vec3f) -> f32 {
	point.z - self.z
    }
}
    
