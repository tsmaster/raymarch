// math.rs

#[derive(Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn dist(self, p:&Vec3f) -> f32 {
	let dx = p.x - self.x;
	let dy = p.y - self.y;
	let dz = p.z - self.z;

	f32::sqrt(dx*dx + dy*dy + dz*dz)
    }
}
