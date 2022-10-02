// operators/boolean.rs
//
// boolean operators on scene primitives
// e.g.
// - union
// - difference
// - intersection
// - smooth addition
// - round corners

use crate::math::Vec3f;
use crate::sdf::SDF;

pub struct OpUnion {
    pub primitive1:Box<dyn SDF + Sync>,
    pub primitive2:Box<dyn SDF + Sync>
}

impl SDF for OpUnion {
    fn dist(&self, point: &Vec3f) -> f32 {
	f32::min(self.primitive1.dist(&(*point)),
		 self.primitive2.dist(&(*point)))
    }
}

pub struct OpSubtraction {
    pub primitive1:Box<dyn SDF + Sync>,
    pub primitive2:Box<dyn SDF + Sync>
}

impl SDF for OpSubtraction {
    fn dist(&self, point: &Vec3f) -> f32 {
	f32::max(self.primitive1.dist(&(*point)),
		 -self.primitive2.dist(&(*point)))
    }
}


pub struct OpIntersection {
    pub primitive1:Box<dyn SDF + Sync>,
    pub primitive2:Box<dyn SDF + Sync>
}

impl SDF for OpIntersection {
    fn dist(&self, point: &Vec3f) -> f32 {
	f32::max(self.primitive1.dist(&(*point)),
		 self.primitive2.dist(&(*point)))
    }
}
