// lights/cone.rs

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;

pub struct ConeLight {
    pub posn: Vec3f,
    pub direction: Vec3f, // the direction that the light goes
    pub color: ColorRgbF,
    pub intensity: f32,
    pub intensity_angle_full_degrees: f32,
    pub intensity_angle_zero_degrees: f32,
    pub distance_falloff: f32,
    pub distance_falloff_scale: f32,
}
