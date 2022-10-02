// geom/cylinder.rs
//
// the SDF for cylinders, finite or infinite.

use crate::math::{Vec2f, Vec3f};
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
pub struct CylinderInfiniteX {
    pub y: f32,
    pub z: f32,
    pub r: f32
}

impl SDF for CylinderInfiniteX {
    fn dist(&self, point: &Vec3f) -> f32 {
	Vec2f {
	    x: self.y - point.y,
	    y: self.z - point.z}.len() - self.r
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CylinderInfiniteY {
    pub x: f32,
    pub z: f32,
    pub r: f32
}

impl SDF for CylinderInfiniteY {
    fn dist(&self, point: &Vec3f) -> f32 {
	Vec2f {
	    x: self.x - point.x,
	    y: self.z - point.z}.len() - self.r
    }
}


#[derive(Debug, Copy, Clone)]
pub struct CylinderInfiniteZ {
    pub x: f32,
    pub y: f32,
    pub r: f32
}

impl SDF for CylinderInfiniteZ {
    fn dist(&self, point: &Vec3f) -> f32 {
	Vec2f {
	    x: self.x - point.x,
	    y: self.y - point.y}.len() - self.r
    }
}



#[derive(Debug, Copy, Clone)]
pub struct CylinderCappedY {
    pub h: f32,
    pub r: f32
}

impl SDF for CylinderCappedY {
    fn dist(&self, point: &Vec3f) -> f32 {

	let hr = Vec2f {
	    x: self.r,
	    y: self.h
	};

	let xz = Vec2f {
	    x: point.x,
	    y: point.z
	};
	
	let d = Vec2f {
	    x: xz.len(),
	    y: point.y}.abs() - hr;

	f32::min(f32::max(d.x, d.y), 0.0) + d.max(0.0).len()
    }
}



#[derive(Debug, Copy, Clone)]
pub struct CylinderCappedZ {
    pub h: f32,
    pub r: f32
}

impl SDF for CylinderCappedZ {
    fn dist(&self, point: &Vec3f) -> f32 {

	let hr = Vec2f {
	    x: self.r,
	    y: self.h
	};

	let xy = Vec2f {
	    x: point.x,
	    y: point.y
	};
	
	let d = Vec2f {
	    x: xy.len(),
	    y: point.z}.abs() - hr;

	f32::min(f32::max(d.x, d.y), 0.0) + d.max(0.0).len()
    }
}


