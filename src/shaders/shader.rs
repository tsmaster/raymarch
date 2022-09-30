// shaders/shader.rs
//
// define a trait for shaders

use crate::bdg_color::ColorRgbF;
use crate::math::Vec3f;
use crate::LightSource;
use crate::SDF;

pub trait Shader {
    fn get_color(&self, point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>		 
		      
    ) -> ColorRgbF;
}
