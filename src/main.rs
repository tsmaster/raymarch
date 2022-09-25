use image::ColorType;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::time::Instant;
use clap::Parser;
use crossbeam_channel::bounded;

extern crate num_cpus;

mod bdg_color;
mod camera;
mod cast;
mod crayola_color;
mod geom;
mod lights;
mod math;
mod sdf;
mod shaders;
mod sky;
mod xkcd_color;

use crate::cast::shoot_ray_at_objects;
use math::{Vec3f, Ray};
use geom::plane::ZPlusPlane;
use geom::sphere::Sphere;
use bdg_color::{ColorRgbF, ColorRgb8};
use lights::ambient::AmbientLight;
use lights::cone::ConeLight;
use lights::directional::DirectionalLight;
use lights::point::PointLight;
use lights::lightsource::LightSource;
use sdf::SDF;

use shaders::diffuse::DiffuseShader;
use shaders::specular::SpecularShader;
use shaders::reflective::ReflectiveShader;
use shaders::checker::CheckerShader;
use shaders::distance_fade::DistanceFadeShader;
use shaders::shader::Shader;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    num_threads: Option<i32>,

    #[clap(short = 'o', long = "output")]
    output_name: Option<String>,
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
    
    let p = ZPlusPlane {
	z: 0.0
    };

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

    let mut objects = Vec::<(Box<dyn SDF + Sync>, Box<dyn Shader + Sync>)>::new();
    
    objects.push((Box::new(p), Box::new(DistanceFadeShader {
	near_dist: 50.0,
	far_dist: 250.0,
	near_shader: Box::new(CheckerShader{
	    x_width: 6.0,
	    y_width: 6.0,
	    odd_shader: Box::new(DiffuseShader {
		color: ColorRgbF::CRAYOLA_ALMOND}),
	    even_shader: Box::new(DiffuseShader {
		color: ColorRgbF::CRAYOLA_MIDNIGHT_BLUE}),
	}),
	far_shader: Box::new(DiffuseShader {
	    color: ColorRgbF::CRAYOLA_WILD_BLUE_YONDER}),
    })));

    let mut lights = Vec::<Box<dyn LightSource + Sync>>::new();

    lights.push(Box::new(AmbientLight {
	color: ColorRgbF::WHITE,
	intensity: 0.15
    }));

    lights.push(Box::new(DirectionalLight {
	direction: Vec3f {
	    x: -1.0,
	    y: 1.0,
	    z: -4.0
	},
	color: ColorRgbF::WHITE,
	intensity: 0.55
    }));

    lights.push(Box::new(DirectionalLight {
	direction: Vec3f {
	    x: 1.0,
	    y: 1.0,
	    z: -0.2
	},
	color: ColorRgbF::CYAN,
	intensity: 0.3
    }));

    
    // ring of spheres
    /*
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
} */

    // single sphere scene
    /*
    {
	let sphere_posn = Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: 3.0
	};
	let white_sphere = Sphere {
	    center: sphere_posn,
	    r:2.0
	};
	objects.push((Box::new(white_sphere), ColorRgbF::CRAYOLA_CARNATION_PINK));
    }*/

    // crayola crayon box
    {
	let spc = 5.0;

	let white_sphere_posn = Vec3f {
	    x: spc,
	    y: -spc,
	    z: 3.0
	};
	let white_sphere = Sphere {
	    center: white_sphere_posn,
	    r:2.0
	};
	objects.push((Box::new(white_sphere),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_WHITE})));

	let red_sphere_posn = Vec3f {
	    x: -spc,
	    y: 0.0,
	    z: 3.0
	};
	let red_sphere = Sphere {
	    center: red_sphere_posn,
	    r:2.0
	};
	objects.push((Box::new(red_sphere),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_RED})));
	
	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: spc,
		y: 0.0,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GREEN})));

	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: 0.0,
		y: -spc,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_YELLOW})));
	
	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: 0.0,
		y: spc,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_VIOLET_PURPLE})));

	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: spc,
		y: spc,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_BLUE})));
	
	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: -spc,
		y: -spc,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_ORANGE})));

	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: 0.0,
		y: 0.0,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GRAY})));
	
	objects.push((Box::new(Sphere {
	    center: Vec3f {
		x: -spc,
		y: spc,
		z: 3.0},
	    r: 2.0}),
		      Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_BLACK})));
    }

    let bounds = (1600, 900);
    //let bounds = (160, 90);

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
		let immut_objects = &objects;

		let immut_lights = &lights;

		let tx_clone = tx.clone();

		spawner.spawn(move |_| {
		    let mut out_data = HashMap::<(usize, usize), ColorRgb8>::new();
		    
		    //render
		    for ((x,y),r) in rc {
			let hit = shoot_ray_at_objects(r,
						       &immut_objects,
						       &cam.posn,
						       1000, 10000.0);

			// This is a hack to get the ground to be
			// checkered.  I have not yet written an
			// object that has geometry and a shader.
			let c = match hit {
			    Some((idx, pos)) => {
				let normal = calc_normal(&immut_objects[idx].0, pos);
				let base_color = immut_objects[idx].1.get_base_color(&pos,
										     &normal,
										     &cam.posn);

				let illum_color = get_illumination(pos,
								   normal,
								   &immut_lights,
								   immut_objects
				);

				let r = (illum_color.r / 255.0) * base_color.r;
				let g = (illum_color.g / 255.0) * base_color.g;
				let b = (illum_color.b / 255.0) * base_color.b;
				    
				ColorRgbF {
				    r: r,
				    g: g,
				    b: b
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

/*

fn shade_sphere(v: Vec3f,
		n: Vec3f,
		_cam_pos: &Vec3f,
		material_color: &ColorRgbF,
		lights: &Vec::<Box<dyn LightSource + Sync>>) -> ColorRgbF {

    let mut illum_color = ColorRgbF::BLACK;

    for light in lights {
	match light.get_illumination(&v, &n) {
	    None => {}
            Some((intensity, color)) => illum_color = illum_color + (color * intensity)
	}
    }

    let r = (illum_color.r / 255.0) * material_color.r;
    let g = (illum_color.g / 255.0) * material_color.g;
    let b = (illum_color.b / 255.0) * material_color.b;

    ColorRgbF {
	r: r,
	g: g,
	b: b
    }
}*/


fn get_illumination(v: Vec3f,
		    n: Vec3f,
		    lights: &Vec::<Box<dyn LightSource + Sync>>,
		    objects: &Vec::<(Box<dyn SDF + Sync>,
				     Box<dyn Shader + Sync>)>) -> ColorRgbF {

    let mut illum_color = ColorRgbF::BLACK;

    for light in lights {
	match light.get_illumination(&v, &n, objects) {
	    None => {}
            Some((intensity, color)) => illum_color = illum_color + (color * intensity)
	}
    }

    illum_color
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
