use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::time::Instant;
use crossbeam_channel::bounded;

extern crate num_cpus;

mod bdg_color;
mod camera;
mod geom;
mod math;
mod sdf;
mod sky;

use math::{Vec3f, Ray};
use geom::plane::ZPlusPlane;
use geom::sphere::Sphere;
use bdg_color::{ColorRgbF, ColorRgb8};
use sdf::SDF;

fn main() {
    let start_time = Instant::now();

    let found_cpus = num_cpus::get();

    println!("found {} CPUs", found_cpus);

    let num_threads = ((found_cpus as f32) * 1.5) as usize;

    println!("using {} threads", num_threads);
    
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
	z: 2.0
    };
    
    let s = Sphere {
	center: sphere_center,
	r: 1.0
    };
    
    println!("Hello, world!");
    println!("Vec: {:?}", v);

    let p_dist = &p.dist(&v);
    println!("plane dist: {}", p_dist);

    let s_dist = &s.dist(&v);
    println!("sphere dist: {}", s_dist);

    let camera_posn = Vec3f{
	x: 10.0,
	y: -8.0,
	z: 6.0
    };

    let look_posn = Vec3f{
	x: 0.0,
	y: 0.0,
	z: 1.0
    };

    let cam = camera::PerspectiveCamera{
	posn: camera_posn,
	look_at: look_posn,
	up: Vec3f::UP,
	fov: 60.0,
    };

    let sky_box = sky::SkySphere {

    };

    let mut objects = Vec::<(Box<dyn SDF + Sync>, ColorRgbF)>::new();

    objects.push((Box::new(p), ColorRgbF::RED));

    for angle in (0..360).step_by(30) {
	let angle_radians = math::degrees_to_radians(angle as f32);
	let sphere_posn = Vec3f{
	    x: 6.0 * f32::sin(angle_radians),
	    y: 6.0 * f32::cos(angle_radians),
	    z: 2.0
	};
	
	let color = ColorRgbF::from_hsv(angle as f32, 1.0, 1.0);
	let colored_sphere = Sphere {
	    center: sphere_posn,
	    r: 1.0
	};
	objects.push((Box::new(colored_sphere), color));
    }    

    let bounds = (1600, 900);
    //let bounds = (160, 90);

    let mut pixels = vec![0; 3 * bounds.0 * bounds.1];

    let rays = cam.get_rays(bounds.0, bounds.1);

    let rays_per_chunk = rays.len() / num_threads + 1;

    println!("chunk size: {}", rays_per_chunk);

    {
	let ray_chunks: Vec<&[((usize, usize), Ray)]> =
	    rays.chunks(rays_per_chunk).collect();

	let rendered_chunk = crossbeam::scope(|spawner| {
	    //println!("inside crossbeam");

	    // bounded multiple producer channel
	    let (tx, rx) = bounded(0);
	    
	    for (_i, rc) in ray_chunks.into_iter().enumerate() {
		//println!("preparing thread {}", i);

		// we want an immutable list of objects
		let obj_copy = &objects;

		let tx_clone = tx.clone();

		spawner.spawn(move |_| {
		    let mut out_data = HashMap::<(usize, usize), ColorRgb8>::new();
		    
		    //render
		    for ((x,y),r) in rc {
			//println!("rendering {},{}", x, y);

			let hit = shoot_ray_at_objects(r, &obj_copy,
						       &cam, 1000, 10000.0);

			// This is a hack to get the ground to be
			// checkered.  I have not yet written an
			// object that has geometry and a shader.
			let c = match hit {
			    Some((idx, pos)) => {
				match idx {
				    0 => shade_checker(pos),
				    _ => obj_copy[idx].1,
				}
			    },
			    None => {
				sky_box.shoot_ray(*r)
			    }
			};
			
			let c_b = c.to_rgb8();

			out_data.insert((*x,*y), c_b);
		    }

		    tx_clone.send(out_data).unwrap();
		});
	    }

	    for _ri in 0..num_threads {
		let ret_data = rx.recv().unwrap();
		//println!("unpacking {}", ri);
		for ((x, y), c) in ret_data {
		    //println!("Got {} {} {:?}", x,y,c);
		    let i = (x + y * bounds.0) * 3;
		    pixels[i]   = c.r;
		    pixels[i+1] = c.g;
		    pixels[i+2] = c.b;
		}
	    }
	}).unwrap();
	
	println!("rendered chunk: {:?}", rendered_chunk);
    }

    let render_duration = start_time.elapsed();

    let save_start_time = Instant::now();

    write_image("OutImages/test_image.png",
		&pixels,
		bounds).expect("error writing image file");

    let save_duration = save_start_time.elapsed();
    let overall_duration = start_time.elapsed();

    println!("Render time: {:?}", render_duration);
    println!("Save time:   {:?}", save_duration);
    println!("Total time:  {:?}", overall_duration);
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

fn shoot_ray_at_objects(r: &Ray,
			obj_list: &Vec<(Box<dyn SDF + Sync>, ColorRgbF)>,
			cam: &camera::PerspectiveCamera,
			num_steps: usize,
			dist: f32) -> Option<(usize, Vec3f)> {

    let tolerance = 1.0e-6;

    let mut cur_pos = cam.posn;
    let r_step = r.direction.normalized();

    for _si in 0 .. num_steps {
	let mut best_dist = dist * 100.0;
	let mut best_obj_idx = 0;

	for obj_idx in 0 .. obj_list.len() {
	    let (obj, _) = &obj_list[obj_idx];
	    
	    let obj_dist = obj.dist(&cur_pos);
	    if obj_dist < best_dist {
		best_obj_idx = obj_idx;
		best_dist = obj_dist;
	    }
	}

	if best_dist < tolerance {
	    return Some((best_obj_idx, cur_pos));
	}

	cur_pos = cur_pos + r_step.scale(best_dist);

	if (cur_pos - cam.posn).len() > dist {
	    break;
	}	
    }
    
    return None;
}




fn shade_checker(v: Vec3f) -> ColorRgbF {
    let square_width = 8.0;
    
    let mut vx = v.x % (square_width * 2.0);
    let mut vy = v.y % (square_width * 2.0);

    if vx < 0.0 {
	vx += square_width * 2.0;
    }

    if vy < 0.0 {
	vy += square_width * 2.0;
    }

    if vx > square_width && vy > square_width {
	return ColorRgbF::WHITE;
    }

    if vx < square_width && vy < square_width {
	return ColorRgbF::WHITE;
    }

    return ColorRgbF::BLACK;
}
