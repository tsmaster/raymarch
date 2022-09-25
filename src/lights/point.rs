// lights/point.rs

use crate::math::Vec3f;
use crate::bdg_color::ColorRgbF;

pub struct PointLight {
    pub posn:Vec3f,
    pub color: ColorRgbF,
    pub intensity: f32,
    pub falloff: f32,
    pub falloff_scale: f32,
}
