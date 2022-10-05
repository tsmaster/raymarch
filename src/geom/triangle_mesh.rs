// geom/triangle_mesh.rs
//
// a triangle mesh
// based on code explained at https://iquilezles.org/articles/triangledistance/

use std::fmt;

use crate::geom::cubebox::CubeBox;
use crate::geom::translate::OpTranslate;
use crate::math::{Vec3f, sign, clamp};
use crate::sdf::SDF;


//#[derive(Debug, Copy, Clone)]
pub struct TriangleMesh {
    pub triangles: Vec<[Vec3f; 3]>,

    // precomputed data
    precomp_tris: Vec<PrecompTri>,

    // bounding volume
    bound_vol: Option<Box<dyn SDF + Sync>>,
}

struct PrecompTri {
    a: Vec3f,
    b: Vec3f,
    c: Vec3f,
    
    ba: Vec3f,
    cb: Vec3f,
    ac: Vec3f,

    nor: Vec3f,
    c_ba_nor: Vec3f,
    c_cb_nor: Vec3f,
    c_ac_nor: Vec3f,
    
    ood2_ba: f32,
    ood2_cb: f32,
    ood2_ac: f32,
    ood2_nor: f32
}

impl SDF for TriangleMesh {
    fn dist(&self, point: &Vec3f) -> f32 {
	let mut closest_dist = self.calc_squared_dist_to_tri(0, point);

	for i in 1..self.triangles.len() {
	    let test_dist = self.calc_squared_dist_to_tri(i, point);
	    closest_dist = f32::min(closest_dist, test_dist);
	}

	f32::sqrt(closest_dist)
    }

    fn bound(&self, point: &Vec3f) -> Option<f32> {
	match &self.bound_vol {
	    Some(vol) => Some(vol.dist(point)),
	    None => None
	}
    }
}

impl fmt::Debug for TriangleMesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "Tri Mesh")
    }
}

impl TriangleMesh {
    pub fn new() -> Self {
	TriangleMesh {
	    triangles: vec!(),
	    precomp_tris: vec!(),
	    bound_vol: None
	}
    }
    
    fn calc_squared_dist_to_tri(&self, tri_index: usize, p: &Vec3f) -> f32 {
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

	if sign(ba.cross(&nor).dot(&pa)) +
	    sign(cb.cross(&nor).dot(&pb)) +
	    sign(ac.cross(&nor).dot(&pc)) < 2.0 {
		f32::min(f32::min(
		    (ba * clamp(ba.dot(&pa) / ba.dot2(), 0.0, 1.0) - pa).dot2(),
		    (cb * clamp(cb.dot(&pb) / cb.dot2(), 0.0, 1.0) - pb).dot2()),
			 (ac * clamp(ac.dot(&pc) / ac.dot2(), 0.0, 1.0) - pc).dot2())
	    } else {
		nor.dot(&pa) * nor.dot(&pa) / nor.dot2()
	    }
    }

    pub fn bake(&mut self) {
	// make bounding box
	let bbox = self.make_bounding_box();

	// TODO make other options, choose the tightest fit

	self.bound_vol = Some(bbox);

    }

    fn make_bounding_box(&self) -> Box<dyn SDF + Sync> {
	let mut min_x = self.triangles[0][0].x;
	let mut min_y = self.triangles[0][0].y;
	let mut min_z = self.triangles[0][0].z;

	let mut max_x = min_x;
	let mut max_y = min_y;
	let mut max_z = min_z;

	for tri_idx in 0..self.triangles.len() {
	    for vert_idx in 0..3 {
		let vx = self.triangles[tri_idx][vert_idx].x;
		let vy = self.triangles[tri_idx][vert_idx].y;
		let vz = self.triangles[tri_idx][vert_idx].z;

		min_x = f32::min(min_x, vx);
		min_y = f32::min(min_y, vy);
		min_z = f32::min(min_z, vz);

		max_x = f32::max(max_x, vx);
		max_y = f32::max(max_y, vy);
		max_z = f32::max(max_z, vz);
	    }
	}

	let min_vec = Vec3f {
	    x: min_x,
	    y: min_y,
	    z: min_z
	};

	let max_vec = Vec3f {
	    x: max_x,
	    y: max_y,
	    z: max_z
	};

	let center_vec = (max_vec + min_vec) * 0.5;
	let half_size = max_vec - center_vec;

	println!("bounding box min {:?}", min_vec);
	println!("bounding box max {:?}", max_vec);
	println!("bounding box ctr {:?}", center_vec);
	println!("bounding box hsz {:?}", half_size);

	Box::new(OpTranslate {
	    v: center_vec,
	    primitive: Box::new(CubeBox {
		half_size: half_size
	    })
	})
    }
}
