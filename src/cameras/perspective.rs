// camera.rs
// will contain at least a perspective camera

use crate::math::{Vec3f, Ray};

use crate::cameras::camera::Camera;


#[derive(Debug, Copy, Clone)]
pub struct PerspectiveCamera {
    pub posn: Vec3f,
    pub look_at: Vec3f,
    pub up: Vec3f,
    pub fov: f32,  // horizontal FOV in degrees(?!)
}


impl PerspectiveCamera {
    pub fn new(posn: &Vec3f,
	       look_at: &Vec3f,
	       up: &Vec3f,
	       fov: f32) -> PerspectiveCamera {
	PerspectiveCamera {
	    posn: *posn,
	    look_at: *look_at,
	    up: *up,
	    fov
	}
    }
}

impl Camera for PerspectiveCamera {
    fn get_rays(self, width: usize, height: usize) -> Vec::<((usize, usize), Ray)> {
	let mut out_vec = Vec::<((usize, usize), Ray)>::new();

	let forward = (self.look_at - self.posn).normalized();
	let right = (forward.cross(&self.up)).normalized();
	let up_s = right.cross(&forward);

	//println!("forward: {:?}", forward);
	//println!("right:   {:?}", right);

	let half_fov_degrees = self.fov / 2.0;
	let half_fov_radians = half_fov_degrees * std::f32::consts::PI / 180.0;

	let frustum_width = 2.0 * f32::sin(half_fov_radians);
	let frustum_height = frustum_width * (height as f32) / (width as f32);

	let step_right = right.scale(2.0 * frustum_width / (width as f32));
	let step_down = up_s.scale(-2.0 * frustum_height / (height as f32));

	let start_x = - (width as i32) / 2;
	let start_y = - (height as i32) / 2;

	for xi in 0 .. width {
	    for yi in 0 .. height {
		let x = start_x + xi as i32;
		let y = start_y + yi as i32;
		
		let d = forward + step_right.scale(x as f32) + step_down.scale(y as f32);
		let r = Ray {
		    origin: self.posn,
		    direction: d
		};
		out_vec.push(((xi as usize,
			       yi as usize),r));
	    }
	}

	out_vec
    }
}	       
