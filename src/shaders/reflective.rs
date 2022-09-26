// shaders/reflective.rs

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;

pub struct ReflectiveShader {
    pub color: ColorRgbF,
    pub roughness: f32,
}

impl Shader for ReflectiveShader {
    fn get_base_color(&self,
		      _point: &Vec3f,
		      _normal: &Vec3f,
		      _view: &Vec3f) -> ColorRgbF {
	// TODO cast reflection ray(s)
	self.color
    }
}
