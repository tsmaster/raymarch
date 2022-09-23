use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::time::{Duration, Instant};

mod bdg_color;
mod camera;
mod geom;
mod math;
mod sdf;
mod sky;

use math::{Vec3f, Ray};
use geom::plane::ZPlusPlane;
use geom::sphere::Sphere;
use bdg_color::ColorRGB_f;
use sdf::SDF;

fn main() {
    let start_time = Instant::now();
    
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

    let mut objects = Vec::<(Box<dyn SDF>, ColorRGB_f)>::new();

    objects.push((Box::new(p), ColorRGB_f::RED));

    for angle in (0..360).step_by(30) {
	let angle_radians = math::degrees_to_radians(angle as f32);
	let sphere_posn = Vec3f{
	    x: 6.0 * f32::sin(angle_radians),
	    y: 6.0 * f32::cos(angle_radians),
	    z: 2.0
	};
	let red_val = if angle > 250 {
	    250.0
	} else {
	    angle as f32
	};
	let green_val = 250.0 - red_val;
	/*
	let color = ColorRGB_f{
	    r: red_val,
	    g: green_val,
	    b: 50.0
    };*/
	let color = ColorRGB_f::from_hsv(angle as f32, 1.0, 1.0);
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

    for ((x,y),r) in rays {
	//let (hit_idx, hit_pos) = shoot_ray_at_scene(&r, &p, &s, &cam, 1000, 10000.0);

	let mut c = ColorRGB_f::MAGENTA;

	let hit = shoot_ray_at_objects(&r, &objects,
				       &cam, 1000, 10000.0);

	match hit {
	    Some((idx, pos)) => {
		c = match idx {
		    0 => shade_checker(pos),
		    _ => objects[idx].1,
		};
	    },
	    None => {
		c = sky_box.shoot_ray(r);
	    }
	};

	/*
	let c = match hit_idx {
	    0 => sky_box.shoot_ray(r),
	    1 => shade_checker(hit_pos),
	    2 => ColorRGB_f::GRAY_50,
	    _ => ColorRGB_f::MAGENTA
	};*/

	//println!("r: {:?} c: {:?}", r, c);

	let c_b = c.to_rgb8();
	
	//println!("r: {:?} c: {:?} x {} y {}", r, c_b, x, y);
	let i = (x + y * bounds.0) * 3;
	pixels[i] = c_b.r;
	pixels[i+1] = c_b.g;
	pixels[i+2] = c_b.b;
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

fn shoot_ray_at_scene(r: &Ray, p: &ZPlusPlane, s: &Sphere, cam: &camera::PerspectiveCamera,
		      num_steps: usize, dist: f32) -> (u8, Vec3f) {

    let tolerance = 1.0e-6;

    let mut cur_pos = cam.posn;
    let r_step = r.direction.normalized();

    for si in 0 .. num_steps {
	let plane_dist = p.dist(&cur_pos);
	let sphere_dist = s.dist(&cur_pos);

	if plane_dist < tolerance {
	    return (1, cur_pos);
	}

	if sphere_dist < tolerance {
	    return (2, cur_pos);
	}

	let min_dist = if plane_dist < sphere_dist {
	    plane_dist
	} else {
	    sphere_dist
	};

	cur_pos = cur_pos + r_step.scale(min_dist);

	if (cur_pos - cam.posn).len() > dist {
	    break;
	}	
    }
    return (0, cur_pos);
}


fn shoot_ray_at_objects(r: &Ray,
			obj_list: &Vec::<(Box<dyn SDF>, ColorRGB_f)>,
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




fn shade_checker(v: Vec3f) -> ColorRGB_f {
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
	return ColorRGB_f::WHITE;
    }

    if vx < square_width && vy < square_width {
	return ColorRGB_f::WHITE;
    }

    return ColorRGB_f::BLACK;
}
