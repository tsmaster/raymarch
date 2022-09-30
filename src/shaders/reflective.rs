// shaders/reflective.rs
//
// for metallic objects, I guess TODO

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;
use crate::LightSource;
use crate::SDF;

pub struct ReflectiveShader {
    pub color: ColorRgbF,
    pub roughness: f32,
}

impl Shader for ReflectiveShader {
    fn get_color(&self,
		 _point: &Vec3f,
		 _normal: &Vec3f,
		 _cam_pos: &Vec3f,
		 _lights: &Vec::<Box<dyn LightSource + Sync>>,
		 _objects: &Vec::<(Box<dyn SDF + Sync>,
				   Box<dyn Shader + Sync>)>		 
    ) -> ColorRgbF {
	// TODO cast reflection ray(s)
	self.color
    }
}
