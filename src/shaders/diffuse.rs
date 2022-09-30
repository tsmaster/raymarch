// shaders/diffuse.rs
//
// Should be a calculation of objects with diffuse shading, currently just a flat shader TODO

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;
use crate::LightSource;
use crate::SDF;

pub struct DiffuseShader {
    pub color: ColorRgbF,
}

impl Shader for DiffuseShader {
    fn get_color(&self,
		 _point: &Vec3f,
		 _normal: &Vec3f,
		 _cam_posn: &Vec3f,
		 _lights: &Vec::<Box<dyn LightSource + Sync>>,
		 _objects: &Vec::<(Box<dyn SDF + Sync>,
				   Box<dyn Shader + Sync>)>
		      
    ) -> ColorRgbF {

	// TODO calc diffuse falloff here
	self.color
    }
}
