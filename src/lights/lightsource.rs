// lights/lightsource.rs
//
// define a trait for light sources

use crate::bdg_color::ColorRgbF;
use crate::math::Vec3f;
use crate::SDF;
use crate::Shader;

pub trait LightSource {
    fn get_illumination(&self,
			point: &Vec3f,
			normal: &Vec3f,
			objects: &Vec::<(Box<dyn SDF + Sync>,
					 Box<dyn Shader + Sync>)>
    ) -> Option<(f32, ColorRgbF)>;
}
