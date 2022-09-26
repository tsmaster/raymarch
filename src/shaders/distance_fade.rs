// shaders/distance_fade.rs


use crate::math::{Vec3f, clamped_map};
use crate::bdg_color::ColorRgbF;
use crate::Shader;

pub struct DistanceFadeShader {
    pub near_dist: f32,
    pub far_dist: f32,
    pub near_shader: Box<dyn Shader + Sync>,
    pub far_shader: Box<dyn Shader + Sync>,
}

impl Shader for DistanceFadeShader {
    fn get_base_color(&self,
		      point: &Vec3f,
		      normal: &Vec3f,
		      cam_posn: &Vec3f) -> ColorRgbF {

	let dist = point.dist(cam_posn);

	if dist <= self.near_dist {
	    return self.near_shader.get_base_color(point, normal, cam_posn);
	} else if dist >= self.far_dist {
	    return self.far_shader.get_base_color(point, normal, cam_posn);
	}

	let near_color = self.near_shader.get_base_color(point, normal, cam_posn);
	let far_color = self.far_shader.get_base_color(point, normal, cam_posn);

	clamped_map(dist, self.near_dist, self.far_dist, near_color, far_color)
    }
}
