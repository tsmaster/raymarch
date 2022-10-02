// shaders/graphpaper.rs

use crate::SDF;
use crate::shaders::shader::Shader;
use crate::bdg_color::ColorRgbF;
use crate::lights::lightsource::LightSource;
use crate::math::{Vec3f, clamped_map};

pub struct GraphPaperXYShader {
    pub line_period: f32,  // distance between lines
    pub line_width: f32,
    pub line_fade_width: f32,
    pub paper_shader: Box<dyn Shader + Sync>,
    pub line_shader: Box<dyn Shader + Sync>,
}

impl Shader for GraphPaperXYShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>		      
    ) -> ColorRgbF {

	let paper_color = self.paper_shader.get_color(point, normal, cam_posn, lights, objects);
	let line_color = self.line_shader.get_color(point, normal, cam_posn, lights, objects);

	//println!("graph paper shading {:?}", point);
	let mut vx = point.x % self.line_period;
	let mut vy = point.y % self.line_period;

	//println!("vx {} vy {}", vx, vy);

	if vx < 0.0 {
	    vx += self.line_period;
	}

	if vy < 0.0 {
	    vy += self.line_period;
	}


	//println!("after + vx {} vy {}", vx, vy);
	

	let dist_to_x = f32::min(vx, self.line_period - vx);
	let dist_to_y = f32::min(vy, self.line_period - vy);

	let dist_to_line = f32::min(dist_to_x, dist_to_y);

	//println!("dist to line: {} ", dist_to_line);

	clamped_map(dist_to_line,
		    self.line_width / 2.0 - self.line_fade_width / 2.0,
		    self.line_width / 2.0 + self.line_fade_width / 2.0,
		    line_color,
		    paper_color)
    }
}
