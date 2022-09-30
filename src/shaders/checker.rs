// shaders/checker.rs

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;
use crate::LightSource;
use crate::SDF;

pub struct CheckerShader {
    pub x_width: f32,
    pub y_width: f32,
    pub odd_shader: Box<dyn Shader + Sync>,
    pub even_shader: Box<dyn Shader + Sync>,
}

impl Shader for CheckerShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>		      
    ) -> ColorRgbF {

	let mut vx = point.x % (self.x_width * 2.0);
	let mut vy = point.y % (self.y_width * 2.0);

	if vx < 0.0 {
	    vx += self.x_width * 2.0;
	}
	
	if vy < 0.0 {
	    vy += self.y_width * 2.0;
	}
	
	if (vx > self.x_width && vy > self.y_width) ||
	    (vx <= self.x_width && vy <= self.y_width) {
		self.odd_shader.get_color(point, normal, cam_posn, lights, objects)
	    } else {
		self.even_shader.get_color(point, normal, cam_posn, lights, objects)
	    }
    }
}
