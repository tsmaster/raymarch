// geom/triangle_mesh.rs
//
// a triangle mesh


use crate::math::{Vec3f, sign, clamp};
use crate::sdf::SDF;

//#[derive(Debug, Copy, Clone)]
#[derive(Debug)]
pub struct TriangleMesh {
    pub triangles: Vec<[Vec3f; 3]>,
}

impl SDF for TriangleMesh {
    fn dist(&self, point: &Vec3f) -> f32 {
	let mut closest_dist = self.calc_dist_to_tri(0, point);

	for i in 1..self.triangles.len() {
	    let test_dist = self.calc_dist_to_tri(i, point);
	    closest_dist = f32::min(closest_dist, test_dist);
	}

	closest_dist
    }
}

impl TriangleMesh {
    fn calc_dist_to_tri(&self, tri_index: usize, p: &Vec3f) -> f32 {
	let a = self.triangles[tri_index][0];
	let b = self.triangles[tri_index][1];
	let c = self.triangles[tri_index][2];

	let ba = b - a;
	let cb = c - b;
	let ac = a - c;

	let pa = *p - a;
	let pb = *p - b;
	let pc = *p - c;

	let nor = ba.cross(&ac);

	f32::sqrt(
	    if sign(ba.cross(&nor).dot(&pa)) +
		sign(cb.cross(&nor).dot(&pb)) +
		sign(ac.cross(&nor).dot(&pc)) < 2.0 {
		f32::min(f32::min(
		    (ba * clamp(ba.dot(&pa) / ba.dot2(), 0.0, 1.0) - pa).dot2(),
		    (cb * clamp(cb.dot(&pb) / cb.dot2(), 0.0, 1.0) - pb).dot2()),
		    (ac * clamp(ac.dot(&pc) / ac.dot2(), 0.0, 1.0) - pc).dot2())
	    } else {
		nor.dot(&pa) * nor.dot(&pa) / nor.dot2()
	    })
    }
}
