// shaders/diffuse.rs
//
// Should be a calculation of objects with diffuse shading, currently just a flat shader TODO

use crate::sdf::SDF;
use crate::shaders::shader::Shader;
use crate::bdg_color::ColorRgbF;
use crate::lights::lightsource::LightSource;
use crate::math::Vec3f;

pub struct DiffuseShader {
    pub color: ColorRgbF,
}

impl Shader for DiffuseShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>
		      
    ) -> ColorRgbF {
	self.get_ambient_color(lights) + self.get_directional_color(point,
								    normal,
								    cam_posn,
								    lights,
								    objects
								    )
    }
}



impl DiffuseShader {
    fn get_ambient_color(&self,
			 lights: &Vec::<Box<dyn LightSource + Sync>>
    ) -> ColorRgbF {
	let mut accum_color = ColorRgbF::BLACK;

	let empty_objs = vec!();

	for l in lights {
	    match l.get_direction(&Vec3f::ZERO) {
		None => {
		    match l.get_illumination(&Vec3f::ZERO,
					     &Vec3f::ZERO,
					     &empty_objs) {
			Some((i, color)) => {			    
			    accum_color = accum_color + (color * i).modulate(
				&self.color);
			},
			None => {}
		    }
		},
		Some(_) => {}
	    }
	}

	accum_color
    }

    fn get_directional_color(&self,
			     point: &Vec3f,
			     normal: &Vec3f,
			     _cam_posn: &Vec3f,
			     lights: &Vec::<Box<dyn LightSource + Sync>>,
			     objects: &Vec::<(Box<dyn SDF + Sync>,
					      Box<dyn Shader + Sync>)>
    ) -> ColorRgbF {

	// Cd * (N dot L)

	let mut accum_color = ColorRgbF::BLACK;

	let normalized_normal = normal.normalized();

	for l in lights {
	    match l.get_direction(point) {
		None => {},
		Some(light_vec) => {
		    let out_light_vec = (light_vec * -1.0).normalized(); // vector leaving point

		    match l.get_illumination(point,
					     normal,
					     objects) {
			None => {},
			Some ((i, light_color)) => {
			    accum_color = accum_color +
				(i * light_color).modulate(&(self.color * normalized_normal.dot(&out_light_vec)));
			}
		    }
		}
	    }
	}
	
	accum_color
    }
}
