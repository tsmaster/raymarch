// shaders/specular.rs


use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;

pub struct SpecularShader {
    pub color: ColorRgbF,
    pub specular_power: f32,
}

impl Shader for SpecularShader {
    fn get_base_color(&self,
		      _point: &Vec3f,
		      _normal: &Vec3f,
		      _view: &Vec3f) -> ColorRgbF {
	//TODO calculate specular
	self.color
    }    
}
