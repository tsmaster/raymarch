// shaders/distance_fade.rs


use crate::math::{Vec3f, clamped_map};
use crate::bdg_color::ColorRgbF;
use crate::Shader;
use crate::LightSource;
use crate::SDF;

pub struct DistanceFadeShader {
    pub near_dist: f32,
    pub far_dist: f32,
    pub near_shader: Box<dyn Shader + Sync>,
    pub far_shader: Box<dyn Shader + Sync>,
}

impl Shader for DistanceFadeShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>
    ) -> ColorRgbF {

	let dist = point.dist(cam_posn);

	if dist <= self.near_dist {
	    return self.near_shader.get_color(point, normal, cam_posn, lights, objects);
	} else if dist >= self.far_dist {
	    return self.far_shader.get_color(point, normal, cam_posn, lights, objects);
	}

	let near_color = self.near_shader.get_color(point, normal, cam_posn, lights, objects);
	let far_color = self.far_shader.get_color(point, normal, cam_posn, lights, objects);

	clamped_map(dist, self.near_dist, self.far_dist, near_color, far_color)
    }
}
