// math.rs

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(unused)]
impl Vec3f {
    pub const UP: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 1.0 };
    pub const EAST: Vec3f = Vec3f { x: 1.0, y: 0.0, z: 0.0 };
    pub const NORTH: Vec3f = Vec3f { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vec3f = Vec3f { x: 0.0, y: 0.0, z: -1.0 };
    pub const WEST: Vec3f = Vec3f { x: -1.0, y: 0.0, z: 0.0 };
    pub const SOUTH: Vec3f = Vec3f { x: 0.0, y: -1.0, z: 0.0 };    
}

impl Vec3f {
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

pub fn map<T>(v: f32,
	      i_min: f32, i_max: f32,
	      o_min: T, o_max: T) -> T
where
    T: Mul<f32, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
    f32: Mul<T, Output = T> + Mul<f32, Output = f32>
{
    let t = (v - i_min) / (i_max - i_min);
    o_min + (o_max-o_min) * t
}


pub fn clamped_map<T>(v: f32,
	      i_min: f32, i_max: f32,
	      o_min: T, o_max: T) -> T
where
    T: Mul<f32, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
    f32: Mul<T, Output = T> + Mul<f32, Output = f32>
{
    if v <= i_min {
	return o_min;
    } else if v >= i_max {
	return o_max;
    }
	    
    let t = (v - i_min) / (i_max - i_min);
    o_min + (o_max-o_min) * t
}



pub fn array_interpolate<T>
    (t: f32, vals: &[(f32, T)]) -> T
where
    T: Mul<f32, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy,
    f32: Mul<T, Output = T> + Mul<f32, Output = f32>
{
    for i in 0 .. vals.len()-1 {
	let j = i + 1;

	let ti = vals[i].0;
	let tj = vals[j].0;

	if (ti <= t) && (tj > t) {
	    return map(t, ti, tj, vals[i].1, vals[j].1);
	}
    }

    panic!("trying to interpolate value {}, but range is {} to {}", t, vals[0].0, vals[vals.len()-1].0);
}
