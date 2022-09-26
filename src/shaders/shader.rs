// shaders/shader.rs
//
// define a trait for shaders

use crate::bdg_color::ColorRgbF;
use crate::math::Vec3f;

pub trait Shader {
    fn get_base_color(&self, point: &Vec3f,
		      normal: &Vec3f,
		      view: &Vec3f) -> ColorRgbF;
}
