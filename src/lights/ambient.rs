// lights/ambient.rs

use crate::bdg_color::ColorRgbF;
use crate::math::Vec3f;
use crate::lights::lightsource::LightSource;
use crate::SDF;
use crate::shaders::shader::Shader;

pub struct AmbientLight {
    pub color: ColorRgbF,
    pub intensity: f32,
}

impl LightSource for AmbientLight {
    fn get_illumination(&self,
			_point: &Vec3f,
			_normal: &Vec3f,
			_objects: &Vec::<(Box<dyn SDF + Sync>,
					  Box<dyn Shader + Sync>)>
    ) -> Option<(f32, ColorRgbF)> {
	Some((self.intensity, self.color))
    }


    fn get_direction(&self,
		     _point: &Vec3f) -> Option<Vec3f> {
	None
    }    
}
