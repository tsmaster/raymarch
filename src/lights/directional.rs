// lights/directional.rs

use crate::math::{Vec3f, Ray, clamped_map};
use crate::bdg_color::ColorRgbF;
use crate::LightSource;
use crate::SDF;
use crate::Shader;
use crate::cast;

pub struct DirectionalLight {
    pub direction:Vec3f, // direction the light goes
    pub color: ColorRgbF,
    pub intensity: f32,
}

impl LightSource for DirectionalLight {
    fn get_illumination(&self,
			point: &Vec3f,
			normal: &Vec3f,
			objects: &Vec::<(Box<dyn SDF + Sync>,
					 Box<dyn Shader + Sync>)>
    ) -> Option<(f32, ColorRgbF)> {
	let dot = normal.normalized().dot(&(self.direction.normalized() * -1.0));
	if dot <= 0.0 {
	    None
	} else {
	    let step_away: Vec3f = *point + (self.direction.normalized() * -1.0) * 0.01;
	    //println!("starting point: {:?} step: {:?}", point, step_away);
	    
	    let to_light_ray = Ray {
		origin: step_away,
		direction: self.direction * -1.0
	    };
	    
	    let cast = cast::shoot_ray_at_objects(&to_light_ray,
						  objects,
						  &step_away,
						  1000,
						  10000.0);
	    match cast {
		Some(_) => None,
		None => Some((clamped_map(dot, 0.0, 1.0, 0.0, self.intensity),
			      self.color))
	    }
	}
    }

    fn get_direction(&self,
		     _point: &Vec3f) -> Option<Vec3f> {
	Some(self.direction)
    }
}
