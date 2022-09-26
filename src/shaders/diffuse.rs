// shaders/diffuse.rs

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;

pub struct DiffuseShader {
    pub color: ColorRgbF,
}

impl Shader for DiffuseShader {
    fn get_base_color(&self,
		      _point: &Vec3f,
		      _normal: &Vec3f,
		      _view: &Vec3f) -> ColorRgbF {
	self.color
    }
}
