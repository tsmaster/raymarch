// cameras/ortho.rs
//
// an orthographic camera
// could also be isometric?

use crate::math::{Vec3f, Ray};

use crate::cameras::camera::Camera;

#[derive(Debug, Copy, Clone)]
pub struct OrthoCamera {
    pub posn: Vec3f,  // center of screen
    pub look_at: Vec3f,
    pub world_up: Vec3f,
    pub world_width: f32,  // horizontal width of screen in the world
}


impl OrthoCamera {
    pub fn new(posn: &Vec3f,
	       look_at: &Vec3f,
	       world_up: &Vec3f,
	       world_width: f32) -> OrthoCamera {
	OrthoCamera {
	    posn: *posn,
	    look_at: *look_at,
	    world_up: *world_up,
	    world_width
	}
    }
}

impl Camera for OrthoCamera {
    fn get_rays(self, width: usize, height: usize) -> Vec::<((usize, usize), Ray)> {
	let mut out_vec = Vec::<((usize, usize), Ray)>::new();

	let forward = (self.look_at - self.posn).normalized();
	let right = (forward.cross(&self.world_up)).normalized();
	let up_s = right.cross(&forward);

	let pixels_per_meter = (width as f32) / self.world_width;
	let world_height = (height as f32) / pixels_per_meter;

	let step_right = right.scale(self.world_width / (width as f32));
	let step_down = up_s.scale(-1.0 * world_height / (height as f32));

	let start_x = - (width as i32) / 2;
	let start_y = - (height as i32) / 2;

	for xi in 0 .. width {
	    for yi in 0 .. height {
		let x = start_x + xi as i32;
		let y = start_y + yi as i32;

		let start_posn = self.posn + step_right.scale(x as f32) + step_down.scale(y as f32);

		//println!("xi {} yi {} start {:?} dir {:?}", xi, yi, start_posn, forward);
		
		let r = Ray {
		    origin: start_posn,
		    direction: forward
		};
		out_vec.push(((xi as usize,
			       yi as usize),r));
	    }
	}

	out_vec
    }
}	       
