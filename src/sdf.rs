// sdf.rs
//
// define a trait for Signed Distance Fields

use crate::math::Vec3f;

pub trait SDF {
    fn dist(&self, point: &Vec3f) -> f32;

    fn bound(&self, _point: &Vec3f) -> Option<f32> {
	None
    }
}

// pub trait CopySDF: SDF + Copy{}

