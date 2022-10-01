// lights/point.rs

use crate::math::{Vec3f, Ray, clamped_map};
use crate::cast;
use crate::bdg_color::ColorRgbF;
use crate::lights::lightsource::LightSource;
use crate::SDF;
use crate::Shader;

pub struct PointLight<F : DistanceFalloff> {
    pub posn:Vec3f,
    pub color: ColorRgbF,
    pub falloff: F,
}

impl<F: DistanceFalloff> LightSource for PointLight<F> {
    fn get_illumination(&self,
			point: &Vec3f,
			normal: &Vec3f,
			objects: &Vec::<(Box<dyn SDF + Sync>,
					 Box<dyn Shader + Sync>)>
    ) -> Option<(f32, ColorRgbF)> {
	let dir = self.get_direction(point).unwrap().normalized();
	let dist = (self.posn - *point).len();
	let dot = normal.normalized().dot(&(dir * -1.0));
	if dot <= 0.0 {
	    // facing away
	    None
	} else {
	    let step_away: Vec3f = *point + (dir * -1.0) * 0.01;
	    
	    let to_light_ray = Ray {
		origin: step_away,
		direction: dir * -1.0
	    };
	    
	    let cast = cast::shoot_ray_at_objects(&to_light_ray,
						  objects,
						  &step_away,
						  1000,
						  dist);
	    match cast {
		Some(_) => None,
		None => {
		    let intensity = self.falloff.get_intensity(dist);
		    Some((clamped_map(dot, 0.0, 1.0, 0.0, intensity),
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

pub trait DistanceFalloff {
    fn get_intensity(&self, dist:f32) -> f32;
}

pub struct FalloffConstant {
    pub intensity:f32
}

impl DistanceFalloff for FalloffConstant {
    fn get_intensity(&self, _dist: f32) -> f32 {
	self.intensity
    }
}

pub struct FalloffClampedLinear {
    pub near_intensity:f32,
    pub near_illum_dist:f32,
    pub far_intensity:f32,
    pub far_illum_dist:f32,
}

impl DistanceFalloff for FalloffClampedLinear {
    fn get_intensity(&self, dist: f32) -> f32 {
	clamped_map(dist, self.near_illum_dist, self.far_illum_dist,
		    self.near_intensity, self.far_intensity)
    }
}

pub struct FalloffQuadratic {
}
