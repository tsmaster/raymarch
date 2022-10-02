// shaders/flat.rs
//
// always return a single color, unaffected by lighting

use crate::SDF;
use crate::shaders::shader::Shader;
use crate::bdg_color::ColorRgbF;
use crate::lights::lightsource::LightSource;
use crate::math::Vec3f;

pub struct FlatShader {
    pub color: ColorRgbF,
}

impl Shader for FlatShader {
    fn get_color(&self,
		 _point: &Vec3f,
		 _normal: &Vec3f,
		 _cam_posn: &Vec3f,
		 _lights: &Vec::<Box<dyn LightSource + Sync>>,
		 _objects: &Vec::<(Box<dyn SDF + Sync>,
				   Box<dyn Shader + Sync>)>		 
    ) -> ColorRgbF {
	// doesn't get easier than this
	self.color
    }
}
