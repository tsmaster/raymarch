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
    pub const ZERO: Vec3f = Vec3f { x: 0.0, y: 0.0, z: 0.0 };
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

    pub fn abs(self) -> Vec3f {
	Vec3f {
	    x: f32::abs(self.x),
	    y: f32::abs(self.y),
	    z: f32::abs(self.z)
	}
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

    pub fn dot2(self) -> f32 {
	self.x * self.x + self.y * self.y + self.z * self.z
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

    pub fn angle(self, o:&Vec3f) -> f32 {
	// theta = arccos((a dot b) / (|a| |b|))
	let num = self.dot(o);
	let denom = self.len() * o.len();

	if denom == 0.0 {
	    panic!("0 length vector in angle");
	}

	let ratio = num / denom;
	ratio.acos()
    }

    pub fn min(self, o:&Vec3f) -> Vec3f {
	Vec3f{
	    x: f32::min(self.x, o.x),
	    y: f32::min(self.y, o.y),
	    z: f32::min(self.z, o.z)
	}
    }

    pub fn max(self, o:&Vec3f) -> Vec3f {
	Vec3f{
	    x: f32::max(self.x, o.x),
	    y: f32::max(self.y, o.y),
	    z: f32::max(self.z, o.z)
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

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, s:f32) -> Self {
	Self {
	    x: self.x * s,
	    y: self.y * s,
	    z: self.z * s
	}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn len(self) -> f32 {
	f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn abs(self) -> Vec2f {
	Vec2f {
	    x: f32::abs(self.x),
	    y: f32::abs(self.y)
	}
    }

    pub fn max(self, v:f32) -> Vec2f {
	Vec2f {
	    x: f32::max(self.x, v),
	    y: f32::max(self.y, v)
	}	
    }

    pub fn dot(self, o:&Vec2f) -> f32 {
	self.x * o.x + self.y * o.y
    }

    pub fn dot2(self) -> f32 {
	self.x * self.x + self.y * self.y
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
	Self {
	    x: self.x + other.x,
	    y: self.y + other.y
	}
    }
}

impl Sub for Vec2f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
	Self {
	    x: self.x - other.x,
	    y: self.y - other.y
	}
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, s:f32) -> Self {
	Self {
	    x: self.x * s,
	    y: self.y * s
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

pub fn radians_to_degrees(r: f32) -> f32 {
    r * 180.0 / std::f32::consts::PI
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


pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    f32::min(f32::max(v, min), max)
}

pub fn sign(v: f32) -> f32 {
    if v == 0.0 {
	0.0
    } else if v < 0.0 {
	-1.0
    } else {
	1.0
    }
}
