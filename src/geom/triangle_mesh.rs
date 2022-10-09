// geom/triangle_mesh.rs
//
// a triangle mesh
// based on code explained at https://iquilezles.org/articles/triangledistance/

use std::fmt;

use crate::geom::cubebox::CubeBox;
use crate::geom::container::Container;
use crate::geom::translate::OpTranslate;
use crate::math::{Vec3f, sign, clamp};
use crate::sdf::SDF;

#[derive(Debug, Copy, Clone)]
enum Dimension {
    X,
    Y,
    Z
}

fn select_val_by_dimension(x_val: f32,
			   y_val: f32,
			   z_val: f32,
			   dim: Dimension) -> f32 {
    match dim {
	Dimension::X => x_val,
	Dimension::Y => y_val,
	Dimension::Z => z_val
    }
}

fn sort_values_by_dimension(x_val: f32,
			    y_val: f32,
			    z_val: f32) -> (f32, Dimension, // max
					    f32, Dimension) // min
{
    if x_val >= y_val {
	if x_val >= z_val {
	    if y_val >= z_val {
		// x, y, z
		(x_val, Dimension::X, z_val, Dimension::Z)
	    } else {
		// x, z, y
		(x_val, Dimension::X, y_val, Dimension::Y)
	    }
	} else {
	    // z, x, y
	    (z_val, Dimension::Z, y_val, Dimension::Y)
	}
    } else {
	if x_val >= z_val {
	    // y, x, z
	    (y_val, Dimension::Y, z_val, Dimension::Z)
	} else {
	    if z_val >= y_val {
		// z, y, x
		(z_val, Dimension::Z, x_val, Dimension::X)
	    } else {
		// y, z, x
		(y_val, Dimension::Y, x_val, Dimension::X)
	    }
	}
    }
}

pub struct TriangleBucket {
    min_vec : Vec3f,
    max_vec : Vec3f,
    triangles: Vec<[Vec3f; 3]>,
    sub_buckets: Vec<TriangleBucket>,
}

impl TriangleBucket {
    pub fn new() -> TriangleBucket {
	TriangleBucket {
	    min_vec: Vec3f::ZERO,
	    max_vec: Vec3f::ZERO,
	    triangles: vec!(),
	    sub_buckets: vec!()
	}
    }

    pub fn add_tri(&mut self,
		   tri:&[Vec3f; 3]) {

	let min_x = f32::min(f32::min(tri[0].x,
				      tri[1].x),
			     tri[2].x);

	let min_y = f32::min(f32::min(tri[0].y,
				      tri[1].y),
			     tri[2].y);

	let min_z = f32::min(f32::min(tri[0].z,
				      tri[1].z),
			     tri[2].z);

	let max_x = f32::max(f32::max(tri[0].x,
				      tri[1].x),
			     tri[2].x);

	let max_y = f32::max(f32::max(tri[0].y,
				      tri[1].y),
			     tri[2].y);

	let max_z = f32::max(f32::max(tri[0].z,
				      tri[1].z),
			     tri[2].z);

	if self.triangles.len() == 0 {
	    self.min_vec = Vec3f {
		x: min_x,
		y: min_y,
		z: min_z
	    };

	    self.max_vec = Vec3f {
		x: max_x,
		y: max_y,
		z: max_z
	    };
	} else {
	    self.min_vec = Vec3f {
		x: f32::min(self.min_vec.x,
			    min_x),
		y: f32::min(self.min_vec.y,
			    min_y),
		z: f32::min(self.min_vec.z,
			    min_z)
	    };

	    self.max_vec = Vec3f {
		x: f32::max(self.max_vec.x,
			    max_x),
		y: f32::max(self.max_vec.y,
			    max_y),
		z: f32::max(self.max_vec.z,
			    max_z)
	    };
	}

	self.triangles.push(*tri);
    }

    pub fn subdivide_box(&mut self,
			 min_tris:usize,
			 min_vol: f32,
			 max_depth: usize,
    ) {
	let x_extent = self.max_vec.x - self.min_vec.x;
	let y_extent = self.max_vec.y - self.min_vec.y;
	let z_extent = self.max_vec.z - self.min_vec.z;

	println!("bounding box x ext {} min {} max {}", x_extent, self.min_vec.x, self.max_vec.x);
	println!("bounding box y ext {} min {} max {}", y_extent, self.min_vec.y, self.max_vec.y);
	println!("bounding box z ext {} min {} max {}", z_extent, self.min_vec.z, self.max_vec.z);

	let vol = x_extent *
	    y_extent *
	    z_extent;

	println!("bounding volume {}", vol);
	println!("triangle count {}", self.triangles.len());

	if (self.triangles.len() < min_tris) ||
	    vol < min_vol ||
	    max_depth == 0 {
		println!("base case, no subdivide");
		return;
	    }

	let (max_ext, max_dim, min_ext, min_dim) = sort_values_by_dimension(
	    x_extent,
	    y_extent,
	    z_extent);

	println!("max_ext {} max dim {:?} min ext {} min dim {:?}",
		 max_ext, max_dim,
		 min_ext, min_dim);

	let ext_ratio = max_ext / min_ext;

	println!("extent ratio: {}", ext_ratio);

	let splits = if ext_ratio > 2.5 {
	    (ext_ratio + 0.5) as u32
	} else {
	    2
	};

	self.sort_tris_box(max_dim, splits, min_tris, min_vol, max_depth);
    }

    fn sort_tris_box(&mut self, split_dim: Dimension, num_children: u32,
		     min_tris: usize, min_vol: f32,
		     max_depth: usize) {
	println!("sorting tris by {:?} into {} child buckets", split_dim, num_children);

	let mut sub_buckets = vec!();

	for _i in 0..num_children {
	    sub_buckets.push(TriangleBucket::new());
	}

	let x_extent = self.max_vec.x - self.min_vec.x;
	let y_extent = self.max_vec.y - self.min_vec.y;
	let z_extent = self.max_vec.z - self.min_vec.z;

	println!("x ext {} y ext {} z ext {}", x_extent, y_extent, z_extent);

	let split_extent = select_val_by_dimension(x_extent, y_extent, z_extent, split_dim);

	let split_min_val = select_val_by_dimension(self.min_vec.x,
						    self.min_vec.y,
						    self.min_vec.z,
						    split_dim);

	for t in &self.triangles {
	    let cx = (t[0].x + t[1].x + t[2].x) / 3.0;
	    let cy = (t[0].y + t[1].y + t[2].y) / 3.0;
	    let cz = (t[0].z + t[1].z + t[2].z) / 3.0;

	    let tri_val = select_val_by_dimension(cx, cy, cz, split_dim);
	    //println!("cx {} cy {} cz {}", cx, cy, cz);
	    //println!("tri_val: {}", tri_val);

	    let tri_off = tri_val - split_min_val;

	    let tri_frac = tri_off / split_extent;
	    
	    
	    let tri_idx = if tri_frac >= 1.0 {
		(num_children - 1) as usize
	    } else {
		((num_children as f32) * tri_frac) as usize
	    };
	    
	    //println!("tri_idx: {}", tri_idx);

	    assert!(tri_idx <= num_children as usize);

	    sub_buckets[tri_idx].add_tri(&t);
	}

	sub_buckets.retain(|b| b.triangles.len() > 0);

	if sub_buckets.len() > 1 {
	    self.triangles.clear();
	    for sbi in 0..sub_buckets.len() {
		println!("subdividing sub_bucket {}", sbi);
		let sb = &mut sub_buckets[sbi];
		sb.subdivide_box(min_tris, min_vol, max_depth - 1);
	    }
	    self.sub_buckets = sub_buckets;
	} else {
	    println!("did not put into sub_bucket");
	}
    }
}



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
	if self.precomp_tris.len() > 0 {
	    let mut closest_dist = self.precomp_calc_squared_dist_to_tri(0, point);

	    for i in 1..self.precomp_tris.len() {
		let test_dist = self.precomp_calc_squared_dist_to_tri(i, point);
		closest_dist = f32::min(closest_dist, test_dist);
	    }

	    f32::sqrt(closest_dist)
	} else {
	    let mut closest_dist = self.calc_squared_dist_to_tri(0, point);

	    for i in 1..self.triangles.len() {
		let test_dist = self.calc_squared_dist_to_tri(i, point);
		closest_dist = f32::min(closest_dist, test_dist);
	    }

	    f32::sqrt(closest_dist)
	}
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

    pub fn make_tm_tree_from_triangle_bucket(tri_bucket: TriangleBucket) -> Box<dyn SDF + Sync> {
	//println!("making tree from bucket");
	if tri_bucket.sub_buckets.len() == 0 {
	    // base case

	    let center_vec = (tri_bucket.max_vec + tri_bucket.min_vec) * 0.5;
	    let half_size = tri_bucket.max_vec - center_vec;

	    //println!("cv {:?} hs {:?}", center_vec, half_size);

	    let bounds = OpTranslate {
		v: center_vec,
		primitive: Box::new(CubeBox {
		    half_size: half_size
		})
	    };

	    let mut tm = TriangleMesh::new();
	    tm.triangles = tri_bucket.triangles;
	    tm.bound_vol = Some(Box::new(bounds));
	    tm.bake();
	    Box::new(tm)
	} else {
	    let mut container = Container::new();

	    let mut min_vec = tri_bucket.sub_buckets[0].min_vec;
	    let mut max_vec = tri_bucket.sub_buckets[0].max_vec;

	    for sb in tri_bucket.sub_buckets {
		min_vec = min_vec.min(&sb.min_vec);
		max_vec = max_vec.max(&sb.max_vec);
		let child_tree = TriangleMesh::make_tm_tree_from_triangle_bucket(sb);
		container.add_child(child_tree);
	    }

	    let center_vec = (min_vec + max_vec) * 0.5;
	    let half_size = max_vec - center_vec;
	    
	    let mut container = Box::new(container);

	    let bound_prim = OpTranslate {
		v: center_vec,
		primitive: Box::new(CubeBox {
		    half_size: half_size
		})
	    };
	    
	    container.bound_vol = Some(Box::new(bound_prim));

	    container
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

    fn precomp_calc_squared_dist_to_tri(&self, tri_index: usize, p: &Vec3f) -> f32 {
	let pt = &self.precomp_tris[tri_index];

	let pa = *p - pt.a;
	let pb = *p - pt.b;
	let pc = *p - pt.c;

	if sign(pt.c_ba_nor.dot(&pa)) +
	    sign(pt.c_cb_nor.dot(&pb)) +
	    sign(pt.c_ac_nor.dot(&pc)) < 2.0 {
		f32::min(f32::min(
		    (pt.ba * clamp(pt.ba.dot(&pa) * pt.ood2_ba, 0.0, 1.0) - pa).dot2(),
		    (pt.cb * clamp(pt.cb.dot(&pb) * pt.ood2_cb, 0.0, 1.0) - pb).dot2()),
			 (pt.ac * clamp(pt.ac.dot(&pc) * pt.ood2_ac, 0.0, 1.0) - pc).dot2())
	    } else {
		pt.nor.dot(&pa) * pt.nor.dot(&pa) / pt.nor.dot2()
	    }
    }

    pub fn bake(&mut self) {
	// precompute triangle data

	self.precomp_tris = Vec::with_capacity(self.triangles.len());

	for t in &self.triangles {
	    let a = t[0];
	    let b = t[1];
	    let c = t[2];

	    let ba = b - a;
	    let cb = c - b;
	    let ac = a - c;

	    let nor = ba.cross(&ac);

	    let c_ba_nor = ba.cross(&nor);
	    let c_cb_nor = cb.cross(&nor);
	    let c_ac_nor = ac.cross(&nor);

	    let ood2_ba = 1.0 / (ba.dot2());
	    let ood2_cb = 1.0 / (cb.dot2());
	    let ood2_ac = 1.0 / (ac.dot2());
	    let ood2_nor = 1.0 / (nor.dot2());

	    let precomp_tri = PrecompTri {
		a: a,
		b: b,
		c: c,

		ba: ba,
		cb: cb,
		ac: ac,

		nor: nor,

		c_ba_nor: c_ba_nor,
		c_cb_nor: c_cb_nor,
		c_ac_nor: c_ac_nor,

		ood2_ba: ood2_ba,
		ood2_cb: ood2_cb,
		ood2_ac: ood2_ac,
		ood2_nor: ood2_nor,
	    };

	    self.precomp_tris.push(precomp_tri);
	}
    }
}
