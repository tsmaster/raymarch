// geom/cone.rs
//
// a "capped cone" primitive


use crate::math::{Vec2f, Vec3f, clamp};
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
pub struct CappedCone {
    pub height: f32,
    pub radius_1: f32,
    pub radius_2: f32,
}

impl SDF for CappedCone {
    fn dist(&self, point: &Vec3f) -> f32 {
	let q:Vec2f = Vec2f{
	    x: Vec2f{
		x: point.x,
		y: point.y
	    }.len(),
	    y: point.z
	};
	
	let k1 = Vec2f {
	    x: self.radius_2,
	    y: self.height};
	
	let k2 = Vec2f {
	    x: self.radius_2 - self.radius_1,
	    y: 2.0 * self.height
	};

	
	let ca = Vec2f {
	    x: q.x-f32::min(q.x, if q.y<0.0 {
		self.radius_1
	    } else {
		self.radius_2
	    }),
	    y: f32::abs(q.y) - self.height
	};

	let cb = q - k1 + k2 * clamp( (k1-q).dot(&k2)/k2.dot2(), 0.0, 1.0 );

	let s = if cb.x<0.0 && ca.y<0.0 {
	    -1.0
	} else {
	    1.0
	};
	
	s*f32::sqrt( f32::min(ca.dot2(), cb.dot2()) )
	
    }
}



#[derive(Debug, Copy, Clone)]
pub struct InfiniteCone {
    pub slope: Vec2f,
}

impl SDF for InfiniteCone {
    fn dist(&self, point: &Vec3f) -> f32 {
	let q = Vec2f {
	    x: Vec2f {
		x: point.x,
		y: point.y
	    }.len(),
	    y: -point.z
	};
	let d = (q-self.slope * f32::max(q.dot(&self.slope), 0.0)).len();
	
	d * (if q.x*self.slope.y - q.y*self.slope.x < 0.0 {
	    -1.0
	} else {
	    1.0
	})
    }
}

