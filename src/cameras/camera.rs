// cameras/camera.rs
//
// defines the camera trait

use crate::math::Ray;


pub trait Camera {
    fn get_rays(self, width: usize, height: usize) -> Vec::<((usize, usize), Ray)>;
}
