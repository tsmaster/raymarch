use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::fs::File;
use std::io::{Error, ErrorKind};

mod math;
mod geom;

use math::Vec3f;
use geom::plane::ZPlusPlane;
use geom::sphere::Sphere;

fn main() {
    let v = math::Vec3f {
	x: 0.0,
	y: 0.0,
	z: 8.0};

    let p = ZPlusPlane {
	z: 0.0
    };

    let sphere_center = Vec3f {
	x: 0.0,
	y: 0.0,
	z: 0.0
    };
    
    let s = Sphere {
	center: sphere_center,
	r: 1.0
    };
    
    println!("Hello, world!");
    println!("Vec: {:?}", v);

    let p_dist = p.dist(&v);
    println!("plane dist: {}", p_dist);

    let s_dist = s.dist(&v);
    println!("sphere dist: {}", s_dist);
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
	       -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);

    let encode_result = encoder.write_image(pixels,
					    bounds.0 as u32,
					    bounds.1 as u32,
					    ColorType::Rgb8);

    match encode_result {
	Ok(_) => Ok(()),
	Err(e) => Err(Error::new(ErrorKind::Other, format!("png encode error: {}", e)))
    }
}
