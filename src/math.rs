// math.rs

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub const UP: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 1.0 };
    pub const EAST: Vec3f = Vec3f { x: 1.0, y: 0.0, z: 0.0 };
    pub const NORTH: Vec3f = Vec3f { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vec3f = Vec3f { x: 0.0, y: 0.0, z: -1.0 };
    pub const WEST: Vec3f = Vec3f { x: -1.0, y: 0.0, z: 0.0 };
    pub const SOUTH: Vec3f = Vec3f { x: 0.0, y: -1.0, z: 0.0 };
    
    pub fn dist(self, p:&Vec3f) -> f32 {
	let dx = p.x - self.x;
	let dy = p.y - self.y;
	let dz = p.z - self.z;

	f32::sqrt(dx*dx + dy*dy + dz*dz)
    }

    pub fn cross(self, o:&Vec3f) -> Vec3f {
	let x = self.y * o.z - self.z * o.y;
	let y = self.z * o.x - self.x * o.z;
	let z = self.x * o.y - self.y * o.x;

	Vec3f {x, y, z}
    }

    pub fn dot(self, o:&Vec3f) -> f32 {
	self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn len(self) -> f32 {
	f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalized(self) -> Vec3f {
	let len = self.len();

	Vec3f{
	    x: self.x / len,
	    y: self.y / len,
	    z: self.z / len}
    }

    pub fn scale(self, s:f32) -> Vec3f {
	Vec3f {
	    x: self.x * s,
	    y: self.y * s,
	    z: self.z * s
	}
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
	Self {
	    x: self.x + other.x,
	    y: self.y + other.y,
	    z: self.z + other.z
	}
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
	Self {
	    x: self.x - other.x,
	    y: self.y - other.y,
	    z: self.z - other.z
	}
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f
}

pub fn degrees_to_radians(d: f32) -> f32 {
   d * std::f32::consts::PI / 180.0 
}
