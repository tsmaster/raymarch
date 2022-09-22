// camera.rs
// will contain at least a perspective camera

use math::Vec3f;

pub struct PerspectiveCamera {
    posn: Vec3f,
    look_at: Vec3f,
    up: Vec3f,
    fov: f32,  // horizontal FOV in degrees(?!)
    aspect: f32, // h/v aspect ratio
}

