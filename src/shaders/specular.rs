// shaders/specular.rs
//
// do the full ambient + diffuse + specular Phong shading model
// https://www.geeksforgeeks.org/phong-model-specular-reflection-in-computer-graphics/

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;
use crate::Shader;
use crate::LightSource;
use crate::SDF;

pub struct SpecularShader {
    pub ambient_color: ColorRgbF,
    pub diffuse_color: ColorRgbF,
    pub specular_color: ColorRgbF,
    pub specular_power: f32,
}

impl Shader for SpecularShader {
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

impl SpecularShader {
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
			    //println!("found illum {} {:?}", i, color);
			    let color_w_intensity = color * i;
			    //println!("color w intensity {:?}", color_w_intensity);

			    //println!("ambient color: {:?}", self.ambient_color);

			    let modulated_color = color_w_intensity.modulate(&self.ambient_color);
			    //println!("modulated color {:?}", modulated_color);
			    
			    accum_color = accum_color + (color * i).modulate(
				&self.ambient_color);
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
			     cam_posn: &Vec3f,
			     lights: &Vec::<Box<dyn LightSource + Sync>>,
			     objects: &Vec::<(Box<dyn SDF + Sync>,
					      Box<dyn Shader + Sync>)>
    ) -> ColorRgbF {

	// Cd * (N dot L) + Cs * (R dot V)^Ps

	let mut accum_color = ColorRgbF::BLACK;

	let normalized_normal = normal.normalized();
	let view_vec = &(*cam_posn - *point).normalized(); // vector TO camera

	for l in lights {
	    match l.get_direction(point) {
		None => {},
		Some(light_vec) => {
		    let out_light_vec = (light_vec * -1.0).normalized(); // vector leaving point

		    // R = N * (N dot L * 2) - L
		    let reflected_light_vec = normalized_normal *
			(normalized_normal.dot(&out_light_vec) * 2.0) -
			out_light_vec;

		    match l.get_illumination(point,
					     normal,
					     objects) {
			None => {},
			Some ((i, light_color)) => {
			    accum_color = accum_color +
				(i * light_color).modulate(&(self.diffuse_color * normalized_normal.dot(&out_light_vec) +
							     self.specular_color *
							     reflected_light_vec.dot(view_vec).powf(self.specular_power)));
			}
		    }
		}
	    }
	}
	
	accum_color
    }
}
