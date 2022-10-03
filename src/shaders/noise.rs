// shaders/noise.rs
//
// Implements noise-based materials
//
// reference:
//   https://raytracing.github.io/books/RayTracingTheNextWeek.html#perlinnoise
//   https://docs.rs/noise/latest/noise/



use noise::{NoiseFn, Perlin};

use crate::math::{Vec3f, clamped_map};
use crate::shaders::shader::Shader;
use crate::lights::lightsource::LightSource;
use crate::sdf::SDF;
use crate::bdg_color::ColorRgbF;


pub struct NoisePerlinShader {
    pub noise_fn: Perlin,
    pub scale: f32,
    pub depth: u32,
    pub offset: Vec3f,

    pub shader_0: Box<dyn Shader + Sync>,
    pub shader_1: Box<dyn Shader + Sync>,    
}

impl Shader for NoisePerlinShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>		      
    ) -> ColorRgbF {
	let color_0 = self.shader_0.get_color(point, normal, cam_posn, lights, objects);
	let color_1 = self.shader_1.get_color(point, normal, cam_posn, lights, objects);

	let v = turb(&(*point * self.scale + self.offset),
		     self.depth,
		     &self.noise_fn);
	clamped_map(v, 0.0, 1.0, color_0, color_1)
    }
}



pub struct NoiseMarbleYShader {
    pub noise_fn: Perlin,
    pub scale: f32,
    pub offset: Vec3f,
    
    pub depth: u32,

    pub shader_0: Box<dyn Shader + Sync>,
    pub shader_1: Box<dyn Shader + Sync>,    
}

impl Shader for NoiseMarbleYShader {
    fn get_color(&self,
		 point: &Vec3f,
		 normal: &Vec3f,
		 cam_posn: &Vec3f,
		 lights: &Vec::<Box<dyn LightSource + Sync>>,
		 objects: &Vec::<(Box<dyn SDF + Sync>,
				  Box<dyn Shader + Sync>)>		      
    ) -> ColorRgbF {
	let color_0 = self.shader_0.get_color(point, normal, cam_posn, lights, objects);
	let color_1 = self.shader_1.get_color(point, normal, cam_posn, lights, objects);

	let v = f32::sin(point.y * self.scale + 10.0 * turb(
	    &(*point + self.offset), self.depth, &self.noise_fn));
	
	clamped_map(v, -1.0, 1.0, color_0, color_1)
    }
}

fn turb<N : NoiseFn<[f64; 3]>>(point: &Vec3f,
			       depth: u32,
			       noise: &N) -> f32 {
    
    let mut accum = 0.0;
    let mut temp_point = *point;
    let mut weight = 1.0;
    
    for i in 0..depth {
	accum += weight * noise.get([(temp_point.x) as f64,
				     (temp_point.y) as f64,
				     (temp_point.z) as f64]) as f32;
	weight *= 0.5;
	temp_point = temp_point * 2.0;	    
    }
    
    accum.abs()
}



/*
transcribed from Renderman Companion, p 355

surface
blue_marble(
  float Ks = .4,
        Kd = .6,
        Ka = .1,
        roughness = .1,
        txtscale = 1;
  color specularcolor = 1)
{
  point PP;   /* scaled point in shader space */
  float csp;  /* color spline parameter */
  point Nf;   /* forward-facing normal */
  float pixelsize, twice, scale, weight, turbulence;

  /* Obtain a forward-facing normal for lighting calculations */
  Nf = faceforward(normalize(N), I);

  /*
     Compute "turbulence" a la [PERLIN85]. Turbulence is a sum of
     "noise" components with a "fractal" 1/f power spectrum. It gives the
     visual impression of turbulent fluid flow (for example, as in the
     formation of blue_marble from molten color splines!). Use the
     surface element area in texture space to control the number of
     noise components so that the frequency content is appropriate
     to the scale. This prevents aliasing of the texture.
 */

  PP = transform("shader", P) * txtscale;
  pixelsize = sqrt(area(PP))
  twice = 2 * pixelsize;
  turbulence = 0;
  for (scale = 1; scale > twice; scale /= 2)
    turbulence += scale * noise(PP/scale);
  
  /* Gradual fade out of highest-frequency component near limit */
  if (scale > pixelsize) {
    weight = (scale / pixelsize) - 1;
    weight = clamp(weight, 0, 1);
    turbulence += weight * scale * noise(PP/scale);
  }

  /*
     Magnify the upper part of the turbulence range 0.75:1
     to fill the range 0:1 and use it as the parameter of 
     a color spline through various shades of blue.
 */

  csp = clamp(4 * turbulence - 3, 0, 1);

  Ci = color spline(csp,
    color (0.25, 0.25, 0.35), /* pale blue */
    color (0.25, 0.25, 0.35), /* pale blue */
    color (0.20, 0.20, 0.30), /* medium blue */
    color (0.20, 0.20, 0.30), /* medium blue */
    color (0.20, 0.20, 0.30), /* medium blue */
    color (0.25, 0.25, 0.35), /* pale blue */
    color (0.25, 0.25, 0.35), /* pale blue */
    color (0.15, 0.15, 0.26), /* medium dark blue */
    color (0.15, 0.15, 0.26), /* medium dark blue */
    color (0.10, 0.10, 0.20), /* dark blue */
    color (0.10, 0.10, 0.20), /* dark blue */
    color (0.25, 0.25, 0.35), /* pale blue */
    color (0.10, 0.10, 0.20), /* dark blue */
   );

  /* Multiply this color by the diffusely reflected light. */
  Ci *= Ka*ambient() + Kd*diffuse(Nf);

  /* Adjust for opacity. */
  Oi = Os;
  Ci = Ci * Oi;

  /* Add in specular highlights. */
  Ci += specularcolor * Ks * specular(Nf, normalize(-1), roughness);
}

*/
