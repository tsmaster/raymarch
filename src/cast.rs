// cast.rs
//
// contains the logic to cast a ray

use crate::math::{Vec3f, Ray};
use super::sdf::SDF;
use crate::shaders::shader::Shader;


pub fn shoot_ray_at_objects(r: &Ray,
			obj_list: &Vec<(Box<dyn SDF + Sync>,
					Box<dyn Shader + Sync>)>,
			start_point: &Vec3f,
			num_steps: usize,
			dist: f32) -> Option<(usize, Vec3f)> {

    let tolerance = 1.0e-4;

    let mut cur_pos = *start_point;
    let r_step = r.direction.normalized();

    for _si in 0 .. num_steps {
	let mut best_dist = dist * 100.0;
	let mut best_obj_idx = 0;

	for obj_idx in 0 .. obj_list.len() {
	    let (obj, _) = &obj_list[obj_idx];

	    let obj_bound = obj.bound(&cur_pos);
	    match (obj_bound) {
		Some(bound_dist) => {
		    if (bound_dist > best_dist) {
			continue;
		    }
		},
		None => {}
	    }
	    
	    let obj_dist = obj.dist(&cur_pos);
	    if obj_dist < best_dist {
		best_obj_idx = obj_idx;
		best_dist = obj_dist;
	    }
	}

	if best_dist < tolerance {
	    return Some((best_obj_idx, cur_pos));
	}

	cur_pos = cur_pos + r_step.scale(best_dist);

	if (cur_pos - *start_point).len() > dist {
	    break;
	}	
    }
    
    return None;
}

