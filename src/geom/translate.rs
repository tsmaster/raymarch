// geom/translate.rs
//
// the SDF for a translation operator, where the primitive is moved a
// distance v
// 

use crate::math::{Vec3f};
use crate::sdf::SDF;

pub struct OpTranslate {
    pub v: Vec3f,
    pub primitive: Box<dyn SDF + Sync>
	
}

impl SDF for OpTranslate {
    fn dist(&self, point: &Vec3f) -> f32 {
	self.primitive.dist(&(*point - self.v))
    }
}
