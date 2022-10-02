// lights/cone.rs

use crate::SDF;
use crate::bdg_color::ColorRgbF;
use crate::cast;
use crate::lights::lightsource::LightSource;
use crate::math::{Vec3f, Ray, clamped_map, radians_to_degrees};
use crate::shaders::shader::Shader;

pub struct ConeLight {
    pub posn: Vec3f,
    pub direction: Vec3f, // the direction that the light goes
    pub color: ColorRgbF,
    pub intensity: f32,
    pub intensity_angle_full_degrees: f32,
    pub intensity_angle_zero_degrees: f32,
}

impl LightSource for ConeLight {
    fn get_illumination(&self,
			point: &Vec3f,
			normal: &Vec3f,
			objects: &Vec::<(Box<dyn SDF + Sync>,
					 Box<dyn Shader + Sync>)>
    ) -> Option<(f32, ColorRgbF)> {
	let axial_vec_to_light = (self.direction * -1.0).normalized();
	let dir_to_light = (self.posn - *point).normalized();
	    
	let dot = normal.normalized().dot(&(axial_vec_to_light));
	
	if dot <= 0.0 {
	    // facing away
	    None
	} else {
	    let step_away: Vec3f = *point + dir_to_light * 0.01;
	    
	    let to_light_ray = Ray {
		origin: step_away,
		direction: dir_to_light
	    };
	    
	    let cast = cast::shoot_ray_at_objects(&to_light_ray,
						  objects,
						  &step_away,
						  1000,
						  10000.0);
	    match cast {
		Some(_) => None,
		None => {

		    let cos_intensity = clamped_map(dot, 0.0, 1.0, 0.0, self.intensity);
		    
		    let angle_to_light = axial_vec_to_light.angle(&dir_to_light);
		    let degrees_to_light = radians_to_degrees(angle_to_light);
		    
		    Some((clamped_map(degrees_to_light,
				      self.intensity_angle_full_degrees,
				      self.intensity_angle_zero_degrees,
				      cos_intensity,
				      0.0),
			  self.color))
		}
	    }
	}
    }

    fn get_direction(&self,
		     point: &Vec3f) -> Option<Vec3f> {
	Some(*point - self.posn)
    }    
}
