// geom/container.rs
//
// a node which contains other geom primitives
// will provide the SDF for the closest primitive
//
// TODO this is basically a CSG union node. Maybe rename it to fit
// with OpDifference, etc.

use std::fmt;
    
use crate::math::Vec3f;
use crate::sdf::SDF;

pub struct Container {
    pub items:Vec<Box<dyn SDF + Sync>>,

    pub bound_vol:Option<Box<dyn SDF + Sync>>
}

impl SDF for Container {
    fn dist(&self, point: &Vec3f) -> f32 {
	assert!(self.items.len() > 0);
	
	let mut best_dist = self.items[0].dist(point);
	for i in 1..self.items.len() {
	    best_dist = f32::min(best_dist, self.items[i].dist(point));
	}
	best_dist
    }

    fn bound(&self, point: &Vec3f) -> Option<f32> {
	assert!(self.items.len() > 0);

	match &self.bound_vol {
	    None => {
		let mut best_dist = match self.items[0].bound(point) {
		    Some(val) => val,
		    None => {return None;}
		};		
		
		for child in &self.items {
		    match child.bound(point) {
			Some(val) => { best_dist = f32::min(best_dist, val); }
			None => { return None; }
		    }
		}

		Some(best_dist)
	    },
	    Some(vol) => {
		Some(vol.dist(point))
	    }
	}
    }
}

impl fmt::Debug for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "Container")
    }
}

impl Container {
    pub fn new() -> Self {
	Container {
	    items: vec!(),
	    bound_vol: None
	}
    }

    pub fn add_child(&mut self, child_obj: Box<dyn SDF + Sync>) {
	self.items.push(child_obj);
    }
}
