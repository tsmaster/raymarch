extern crate num_cpus;

mod bdg_color;
mod camera;
mod cast;
mod crayola_color;
mod geom;
mod lights;
mod math;
mod operators;
mod scene;
mod sdf;
mod shaders;
mod sky;
mod xkcd_color;

use clap::Parser;
use crossbeam_channel::bounded;
use image::{ColorType, ImageEncoder};
use image::codecs::png::PngEncoder;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::time::Instant;
use std::str::FromStr;

use bdg_color::{ColorRgbF, ColorRgb8};
use crate::cast::shoot_ray_at_objects;
use math::{Vec3f, Ray};
use sdf::SDF;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    num_threads: Option<i32>,

    #[clap(short = 'o', long = "output", forbid_empty_values = true)]
    output_name: Option<String>,

    #[clap(long, default_value_t = 0)]
    frame_num: i32,

    #[clap(long, action)] // workaround for boolean flag
    animate: bool,

    #[clap(short, long, value_parser = parse_res)]
    resolution: Option<(usize, usize)>,
}

fn parse_res(s: &str) -> Result<(usize, usize), String> {
    match split_res(s, ',') {
	Some((x, y)) => Ok((x, y)),
	_ => match split_res(s, 'x') {
	    Some((x, y)) => Ok((x,y)),
	    _ => Err(format!(
		"Unparsable resolution str {}. Use , or x",
		s
	    ))
	}
    }		    
}

fn split_res<T:FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
	None => None,
	Some(index) => {
	    match (T::from_str(&s[..index]),
		   T::from_str(&s[index +1 ..])) {
		(Ok(l), Ok(r)) => Some((l,r)),
		_ => None
	    }
	}
    }
}
	

fn main() {
    let args = Args::parse();
    
    let start_time = Instant::now();

    let num_threads = match args.num_threads {
	Some(n) => n as usize,
	None => {
	    let found_cpus = num_cpus::get();

	    println!("found {} CPUs", found_cpus);

	    ((found_cpus as f32) * 1.5) as usize
	}
    };

    println!("using {} threads", num_threads);
    
    let camera_posn = if args.animate {
	println!("animated");
	let anim_duration = 20.0; // seconds
	let fps = 20.0; //frames per second
	let total_frames = (anim_duration * fps) as u32;
	
	let anim_complete_frac = (args.frame_num as f32) / (total_frames as f32);


	let theta = anim_complete_frac * 2.0 * std::f32::consts::PI;

	Vec3f{
	    x: 10.0 * f32::cos(theta),
	    y: 10.0 * f32::sin(theta),
	    z: 6.0
	}
    } else {
	println!("not animated");

	// normal cam

	Vec3f{
	    x: 10.0,
	    y: -8.0,
	    z: 6.0
	}


	// high cam
	/*
	Vec3f {
	    x: 5.0,
	    y: -4.0,
	    z: 10.0
    }*/

	// low cam
	/*
	Vec3f {
	    x: 5.0,
	    y: -4.0,
	    z: 1.0
	}*/
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

    let mut sb = scene::SceneBuilder::new(cam, sky_box);
    //scene::test_scenes::add_checkerboard_floor(&mut sb);
    scene::test_scenes::add_marble_checkerboard_floor(&mut sb);
    //scene::test_scenes::add_graphpaper_floor(&mut sb);
    //scene::test_scenes::add_graphpaper_5_floor(&mut sb);
    //scene::test_scenes::add_turbulent_floor(&mut sb);
    //scene::test_scenes::add_marble_floor(&mut sb);
    scene::test_scenes::add_three_dir_lights(&mut sb);
    //scene::test_scenes::add_point_light(&mut sb);
    //scene::test_scenes::add_cone_light(&mut sb);
    //scene::test_scenes::add_ring_of_spheres_objects(&mut sb);
    //scene::test_scenes::add_single_sphere_object(&mut sb);
    scene::test_scenes::add_single_marble_sphere_object(&mut sb);
    //scene::test_scenes::add_crayola_crayon_spheres_objects(&mut sb);
    //scene::test_scenes::add_disc_with_holes_objects(&mut sb);
    
    let scene = scene::build_scene(sb);

    
    let bounds = match args.resolution {
	None => (1600, 900),
	Some(r) => r
    };

    println!("rendering {:?}", bounds);

    let mut pixels = vec![0; 3 * bounds.0 * bounds.1];

    let rays = cam.get_rays(bounds.0, bounds.1);

    let rays_per_chunk = rays.len() / num_threads + 1;

    {
	let ray_chunks: Vec<&[((usize, usize), Ray)]> =
	    rays.chunks(rays_per_chunk).collect();

	crossbeam::scope(|spawner| {
	    // bounded multiple producer channel
	    let (tx, rx) = bounded(0);
	    
	    for (_i, rc) in ray_chunks.into_iter().enumerate() {
		// we want an immutable list of objects
		let immut_objects = &scene.objects;

		let immut_lights = &scene.lights;

		let tx_clone = tx.clone();

		spawner.spawn(move |_| {
		    let mut out_data = HashMap::<(usize, usize), ColorRgb8>::new();
		    
		    //render
		    for ((x,y),r) in rc {
			let hit = shoot_ray_at_objects(r,
						       &immut_objects,
						       &cam.posn,
						       1000, 10000.0);

			let c = match hit {
			    Some((idx, pos)) => {
				let normal = calc_normal(&immut_objects[idx].0, pos);
				let shaded_color = immut_objects[idx].1.get_color(&pos,
										  &normal,
										  &cam.posn,
										  &immut_lights,
										  &immut_objects
										       
				);
				shaded_color
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
		for ((x, y), c) in ret_data {
		    let i = (x + y * bounds.0) * 3;
		    pixels[i]   = c.r;
		    pixels[i+1] = c.g;
		    pixels[i+2] = c.b;
		}
	    }
	}).unwrap();
    }

    let render_duration = start_time.elapsed();

    let save_start_time = Instant::now();

    let out_file_name = match args.output_name {
	Some(n) => n,
	None => "OutImages/test_image.png".to_string()
    };
    
    write_image(&out_file_name,
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

fn calc_normal(obj:&Box<dyn SDF + Sync>, pos: Vec3f) -> Vec3f {
    let epsilon = 0.001;
    let center_dist = obj.dist(&pos);
    let x_dist = obj.dist(&(pos + Vec3f{x: epsilon, y:0.0, z:0.0}));
    let y_dist = obj.dist(&(pos + Vec3f{x:0.0, y:epsilon, z:0.0}));
    let z_dist = obj.dist(&(pos + Vec3f{x:0.0, y:0.0, z:epsilon}));

    Vec3f{
	x:x_dist - center_dist,
	y:y_dist - center_dist,
	z:z_dist - center_dist
    }.normalized()
}
