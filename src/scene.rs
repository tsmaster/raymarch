// scene.rs
//
// a struct that holds the scene information
// TODO add JSON loading using serde

pub mod test_scenes;

use crate::sdf::SDF;
use crate::camera::PerspectiveCamera;
use crate::lights::lightsource::LightSource;
use crate::shaders::shader::Shader;
use crate::sky;


pub struct Scene {
    pub lights: Vec::<Box<dyn LightSource + Sync>>,
    pub objects: Vec::<(Box<dyn SDF + Sync>,
			Box<dyn Shader + Sync>)>,
    pub camera: PerspectiveCamera,
    pub sky_box: sky::SkySphere,
}

pub struct SceneBuilder {
    pub lights: Vec::<Box<dyn LightSource + Sync>>,
    pub objects: Vec::<(Box<dyn SDF + Sync>,
			Box<dyn Shader + Sync>)>,
    pub camera: PerspectiveCamera,
    pub sky_box: sky::SkySphere,
}

impl SceneBuilder {
    pub fn new(camera: PerspectiveCamera,
	       sky:sky::SkySphere) -> SceneBuilder {
	SceneBuilder {
	    lights: vec!(),
	    objects: vec!(),
	    camera: camera,
	    sky_box: sky
	}
    }

    pub fn add_light(&mut self,
		     light: Box<dyn LightSource + Sync>) {
	self.lights.push(light);
    }

    pub fn add_object(&mut self,
		      object_geom: Box<dyn SDF + Sync>,
		      object_shader: Box<dyn Shader + Sync>) {
	self.objects.push((object_geom,
			   object_shader));
    }
}
	

pub fn build_scene(sb: SceneBuilder) -> Scene {
    Scene {
	lights: sb.lights,
	objects: sb.objects,
	camera: sb.camera,
	sky_box: sb.sky_box
    }
}
    
