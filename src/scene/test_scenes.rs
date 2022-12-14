// scene/test_scenes.rs
//
// hardcoded test scenes

use std::sync::Arc;

// TODO clean this up
use crate::bdg_color::ColorRgbF;
use crate::geom::capsule::Capsule;
use crate::geom::cone::CappedCone;
use crate::geom::cubebox::CubeBox;
use crate::geom::cylinder::{CylinderCappedY, CylinderCappedZ};
use crate::geom::cylinder::CylinderInfiniteX;
use crate::geom::cylinder::CylinderInfiniteY;
use crate::geom::cylinder::CylinderInfiniteZ;
use crate::geom::plane::ZPlusPlane;
use crate::geom::sphere::Sphere;
use crate::geom::torus::Torus;
use crate::geom::triangle_mesh::{TriangleMesh, TriangleBucket};
use crate::geom::translate::OpTranslate;
use crate::lights::ambient::AmbientLight;
use crate::lights::cone::ConeLight;
use crate::lights::directional::DirectionalLight;
use crate::lights::point::{PointLight, FalloffConstant};
use crate::math::Vec3f;
use crate::math;
use crate::operators::boolean::{OpSubtraction};
use crate::scene;
use crate::shaders::checker::CheckerShader;
use crate::shaders::diffuse::DiffuseShader;
use crate::shaders::distance_fade::DistanceFadeShader;
use crate::shaders::graphpaper::GraphPaperXYShader;
use crate::shaders::noise::{NoisePerlinShader,
			    NoiseMarbleYShader};
use crate::shaders::specular::SpecularShader;

pub fn add_checkerboard_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(CheckerShader{
			  x_width: 6.0,
			  y_width: 6.0,
			  odd_shader: Box::new(SpecularShader {
			      ambient_color: ColorRgbF::CRAYOLA_ALMOND,
			      diffuse_color: ColorRgbF::CRAYOLA_ALMOND,
			      specular_color: ColorRgbF::CRAYOLA_WHITE,
			      specular_power: 2.0
			  }),
			  even_shader: Box::new(SpecularShader {
			      ambient_color: ColorRgbF::CRAYOLA_MIDNIGHT_BLUE,
			      diffuse_color: ColorRgbF::CRAYOLA_MIDNIGHT_BLUE,
			      specular_color: ColorRgbF::CRAYOLA_WHITE,
			      specular_power: 4.0
			  }),
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_BLACK}),
		  }));
}


pub fn add_marble_checkerboard_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(CheckerShader{
			  x_width: 6.0,
			  y_width: 6.0,
			  odd_shader: Box::new(NoiseMarbleYShader{
			      noise_fn: noise::Perlin::new(),
			      scale: 2.0,
			      offset: Vec3f {
				  x: 1.2,
				  y: 3.5,
				  z: 8.12,
			      },
			      depth: 7,
			      shader_0: Box::new(DiffuseShader {
				  color: ColorRgbF::GRAY_70,
			      }),
			      shader_1: Box::new(DiffuseShader {
				  color: ColorRgbF::WHITE,
			      })
			  }),
			  even_shader: Box::new(NoiseMarbleYShader{
			      noise_fn: noise::Perlin::new(),
			      scale: 2.0,
			      offset: Vec3f {
				  x: 1.2,
				  y: 3.5,
				  z: 8.12,
			      },
			      depth: 7,
			      shader_0: Box::new(DiffuseShader {
				  color: ColorRgbF::CRAYOLA_BLACK,
			      }),
			      shader_1: Box::new(DiffuseShader {
				  color: ColorRgbF::GRAY_30,
			      })
			  }),			  
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_BLACK}),
		  }));
}



pub fn add_graphpaper_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(GraphPaperXYShader{
			  line_period: 5.0,
			  line_width: 0.1,
			  line_fade_width: 0.05,
			  paper_shader: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_ALMOND,
			  }),
			  line_shader: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_BLACK,
			  }),
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GRAY}),
		  }));
}

pub fn add_graphpaper_5_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(GraphPaperXYShader{
			  line_period: 5.0,
			  line_width: 0.1,
			  line_fade_width: 0.05,
			  paper_shader: Box::new(GraphPaperXYShader{
			      line_period: 1.0,
			      line_width: 0.02,
			      line_fade_width: 0.01,
			      paper_shader: Box::new(DiffuseShader {
				  color: ColorRgbF::CRAYOLA_WHITE,
			      }),
			      line_shader: Box::new(DiffuseShader {
				  color: ColorRgbF::CRAYOLA_GRAY,
			      })
			  }),
			  line_shader: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_BLACK,
			  }),
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GRAY}),
		  }));
}


pub fn add_turbulent_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(NoisePerlinShader{
			  noise_fn: noise::Perlin::new(),
			  scale: 2.0,
			  offset: Vec3f {
			      x: 1.5,
			      y: 2.3,
			      z: 3.9,
			  },
			  depth: 7,
			  shader_0: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_BLACK,
			  }),
			  shader_1: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_WHITE,
			  })
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GRAY}),
		  }));
}


pub fn add_marble_floor(sb: &mut scene::SceneBuilder) {
    let p = ZPlusPlane {
	z: 0.0
    };

    sb.add_object(Box::new(p),
		  Box::new(DistanceFadeShader {
		      near_dist: 50.0,
		      far_dist: 250.0,
		      near_shader: Box::new(NoiseMarbleYShader{
			  noise_fn: noise::Perlin::new(),
			  scale: 2.0,
			  offset: Vec3f {
			      x: 1.2,
			      y: 3.5,
			      z: 8.12,
			  },
			  depth: 7,
			  shader_0: Box::new(DiffuseShader {
			      color: ColorRgbF::GRAY_50,
			  }),
			  shader_1: Box::new(DiffuseShader {
			      color: ColorRgbF::CRAYOLA_ALMOND,
			  })
		      }),
		      far_shader: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_GRAY}),
		  }));
}





pub fn add_three_dir_lights(sb: &mut scene::SceneBuilder) {
    sb.add_light(Box::new(AmbientLight {
	color: ColorRgbF::WHITE,
	intensity: 0.2
    }));

    sb.add_light(Box::new(DirectionalLight {
	direction: Vec3f {
	    x: -1.0,
	    y: 1.0,
	    z: -4.0
	},
	color: ColorRgbF::WHITE,
	intensity: 0.5
    }));

    sb.add_light(Box::new(DirectionalLight {
	direction: Vec3f {
	    x: 1.0,
	    y: 1.0,
	    z: -0.2
	},
	color: ColorRgbF::CYAN,
	intensity: 0.3
    }));
}



pub fn add_point_light(sb: &mut scene::SceneBuilder) {
    sb.add_light(Box::new(PointLight {
	posn: Vec3f {
	    x: 8.0,
	    y: 0.0,
	    z: 10.0
	},
	color: ColorRgbF::WHITE,
	falloff: FalloffConstant {
	    intensity: 1.0
	}
    }));
}



pub fn add_cone_light(sb: &mut scene::SceneBuilder) {
    sb.add_light(Box::new(ConeLight {
	posn: Vec3f {
	    x: 2.0,
	    y: 0.0,
	    z: 10.0
	},
	direction: Vec3f {
	    x: -2.0,
	    y: 0.0,
	    z: -10.0
	},
	color: ColorRgbF::CRAYOLA_ALMOND,
	intensity: 1.0,
	intensity_angle_full_degrees: 20.0,
	intensity_angle_zero_degrees: 45.0,
    }));
}



pub fn add_single_sphere_object(sb: &mut scene::SceneBuilder) {
    let sphere_posn = Vec3f {
	x: 0.0,
	y: 0.0,
	z: 3.0
    };
    let white_sphere = Sphere {
	center: sphere_posn,
	r:2.0
    };
    sb.add_object(Box::new(white_sphere),
		  Box::new(SpecularShader {
		      ambient_color: ColorRgbF::CRAYOLA_WHITE,
		      diffuse_color: ColorRgbF::CRAYOLA_WHITE,
		      specular_color: ColorRgbF::CRAYOLA_WHITE,
		      specular_power: 6.0
		  }));
}


pub fn add_single_marble_sphere_object(sb: &mut scene::SceneBuilder) {
    let sphere_posn = Vec3f {
	x: 0.0,
	y: 0.0,
	z: 3.0
    };
    let white_sphere = Sphere {
	center: sphere_posn,
	r:2.0
    };
    sb.add_object(Box::new(white_sphere),
		  Box::new(NoiseMarbleYShader{
		      noise_fn: noise::Perlin::new(),
		      scale: 2.0,
		      offset: Vec3f {
			  x: 3.1,
			  y: 4.1,
			  z: 5.9,
		      },
		      depth: 7,
		      shader_0: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_BLACK,
		      }),
		      shader_1: Box::new(DiffuseShader {
			  color: ColorRgbF::CRAYOLA_WHITE,
		      })
		  }),
		  /*
		  Box::new(SpecularShader {
		      ambient_color: ColorRgbF::CRAYOLA_WHITE,
		      diffuse_color: ColorRgbF::CRAYOLA_WHITE,
		      specular_color: ColorRgbF::CRAYOLA_WHITE,
		      specular_power: 6.0
    })*/
    );
}



pub fn add_ring_of_spheres_objects(sb: &mut scene::SceneBuilder) {
    // ring of spheres

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

	sb.add_object(Box::new(colored_sphere),
		      Box::new(DiffuseShader {
			  color}));
    }
}


pub fn add_crayola_crayon_spheres_objects(sb: &mut scene::SceneBuilder) {
    // crayola crayon spheres

    let spc = 5.0;
    let sphere_rad = 2.0;

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: spc,
	    y: -spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_WHITE}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_GRAY}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: -spc,
	    y: spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_BLACK}));



    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: -spc,
	    y: 0.0,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_RED}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: spc,
	    y: 0.0,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_GREEN}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: 0.0,
	    y: -spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_YELLOW}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: 0.0,
	    y: spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_PURPLE}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: spc,
	    y: spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_BLUE}));

    sb.add_object(Box::new(Sphere {
	center: Vec3f {
	    x: -spc,
	    y: -spc,
	    z: 3.0
	},
	r: sphere_rad
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_ORANGE}));
}

pub fn add_box_test_objects(sb: &mut scene::SceneBuilder) {
    // Box test

    sb.add_object(Box::new(CubeBox {
	half_size: Vec3f {
	    x: 5.0,
	    y: 2.5,
	    z: 1.5
	}}),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_ORANGE}));

}

pub fn add_torus_test_object(sb: &mut scene::SceneBuilder) {
    // Torus test

    sb.add_object(Box::new(Torus {
	r1: 2.0,
	r2: 1.0,
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_GREEN}));
}

pub fn add_translated_torus_test_object(sb: &mut scene::SceneBuilder) {
    // Translated Torus test

    sb.add_object(Box::new(OpTranslate {
	v: Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: 3.0
	},
	primitive: Box::new(Torus {
	    r1: 2.0,
	    r2: 1.0,
	})
    }),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_GREEN}));
}

pub fn add_infinite_cylinder_test_objects(sb: &mut scene::SceneBuilder) {
    // infinite cylinder test

    sb.add_object(Box::new(CylinderInfiniteX {
	y: 1.0,
	z: 1.0,
	r: 1.0
    }),
		  Box::new(DiffuseShader {
		      color:ColorRgbF::CRAYOLA_RED}));

    sb.add_object(Box::new(CylinderInfiniteY {
	x: 1.0,
	z: 3.0,
	r: 1.0
    }),
		  Box::new(DiffuseShader {
		      color:ColorRgbF::CRAYOLA_BLUE}));

    sb.add_object(Box::new(CylinderInfiniteZ {
	x: -1.0,
	y: -1.0,
	r: 1.0
    }),
		  Box::new(DiffuseShader {
		      color:ColorRgbF::CRAYOLA_YELLOW}));

    sb.add_object(Box::new(CylinderCappedY {
	h: 4.0,
	r: 1.5
    }),
		  Box::new(DiffuseShader {
		      color:ColorRgbF::CRAYOLA_GREEN}));

    sb.add_object(Box::new(Capsule {
	a: Vec3f {
	    x: 3.0,
	    y: -3.0,
	    z: 1.0,
	},
	b: Vec3f {
	    x: 3.0,
	    y: -3.0,
	    z: 3.0,
	},
	r: 1.0
    }),
		  Box::new(DiffuseShader {
		      color:ColorRgbF::CRAYOLA_VIOLET_PURPLE}));

}



pub fn add_disc_with_holes_objects(sb: &mut scene::SceneBuilder) {

    let disk = Box::new(OpTranslate {
	v: Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: 3.0
	},
	primitive: Box::new(CylinderCappedZ {
	    h:0.5,
	    r:3.0
	})
    });

    let cyl1 = 
	Box::new(OpTranslate {
	    v: Vec3f {
		x: 1.0,
		y: 1.0,
		z: 1.5
	    },
	    primitive: Box::new(CylinderCappedZ {
		h:3.0,
		r:0.8
	    })
	});
    
    let cyl2 = 
	Box::new(OpTranslate {
	    v: Vec3f {
		x: -1.0,
		y: -1.0,
		z: 1.5
	    },
	    primitive: Box::new(CylinderCappedZ {
		h:3.0,
		r:0.8
	    })
	});

    let disk_minus_cyls = 
	Box::new(OpSubtraction {
	    primitive1: Box::new(OpSubtraction {
		primitive1: disk,
		primitive2: cyl1
	    }),
	    primitive2: cyl2
	});
    
    let sphere_nw = Box::new(Sphere {
	center: Vec3f {
	    x: -1.0,
	    y: 1.0,
	    z: 3.5
	},
	r: 0.8
    });

    let sphere_se = Box::new(Sphere {
	center: Vec3f {
	    x: 1.0,
	    y: -1.0,
	    z: 3.5
	},
	r: 0.8
    });

    let disk_minus_spheres =
	Box::new(OpSubtraction {
	    primitive1: Box::new(OpSubtraction {
		primitive1: disk_minus_cyls,
		primitive2: sphere_nw
	    }),
	    primitive2: sphere_se
	});

    sb.add_object(
	disk_minus_spheres,
	Box::new(DiffuseShader {
	    color:ColorRgbF::CRAYOLA_YELLOW}));

}



pub fn add_car_object(sb: &mut scene::SceneBuilder) {
    let scenes = easy_gltf::load("tools/Kenney/CarKit/Models/GLTF/raceFuture.glb").unwrap();

    println!("num scenes: {}", scenes.len());
    for scene in scenes {
	walk_scene(&scene, sb);
    }

    //walk_scene(&scenes[1], sb);
}

fn walk_scene(scene: &easy_gltf::Scene, sb: &mut scene::SceneBuilder) {
    println!("num models {}", &scene.models.len());

    let ONLY_BODY:bool = false;

    if !ONLY_BODY {
	for model in &scene.models {
	    walk_model(&model, sb);
	}
    } else {
	walk_model(&scene.models[1], sb);
    }

    for _camera in &scene.cameras {
	println!("camera");
    }

    for _light in &scene.lights {
	println!("light");
    }
}

fn walk_model(model: &easy_gltf::model::Model, sb: &mut scene::SceneBuilder) {
    println!("---begin model---");

    let mat = visit_material(model.material());
/*
    let mut trimesh = TriangleMesh {
	triangles: vec!(),
};*/

    let mut tri_bucket = TriangleBucket::new();
    

    //let mut trimesh = TriangleMesh::new();    
    
    println!("mode: {:?}", model.mode());
    println!("vert count {}", model.vertices().len());
    match model.indices() {
	Some(indices) => { println!("index count {}", indices.len()); }
	None => {}
    };
    match model.mode() {
	easy_gltf::model::Mode::Triangles | easy_gltf::model::Mode::TriangleFan | easy_gltf::model::Mode::TriangleStrip => {
	    match model.triangles() {
		Ok(triangles) => {
		    println!("triangle count {}", triangles.len());
		    insert_triangles(&triangles, &mut tri_bucket);
		},
		Err(badmode) => {println!("bad mode: {:?}", badmode);}
	    };

	    tri_bucket.subdivide_box(600, 1.0, 10);
	},
	_ => {}
    };
    //println!("model has normals? {}", if model.has_normals() { "true" } else { "false"});
    //println!("model has tangents? {}", if model.has_tangents() { "true" } else { "false"});
    //println!("model has tex coords? {}", if model.has_tex_coords() { "true" } else { "false"});
    //println!("---end model---");

    //trimesh.bake();

    sb.add_object(TriangleMesh::make_tm_tree_from_triangle_bucket(tri_bucket),
		  Box::new(mat));

    /*
    sb.add_object(Box::new(trimesh),
		  Box::new(mat));*/
}

fn visit_material(mat: Arc<easy_gltf::model::Material>) -> impl crate::shaders::shader::Shader {
    println!("----- begin Material -----");

    println!("------PBR begin------");
    println!("base color factor {:?}", mat.pbr.base_color_factor);

    DiffuseShader {
	color: ColorRgbF{
	    r: mat.pbr.base_color_factor[0] * 255.0,
	    g: mat.pbr.base_color_factor[1] * 255.0,
	    b: mat.pbr.base_color_factor[2] * 255.0
	}
    }
	
    
    /*
    match &mat.pbr.base_color_texture {
	Some(rgba) => { println!("has base color texture");},
	None => {}
    };
    
    match &mat.pbr.metallic_texture {
	Some(gray) => { println!("has metallic texture");},
	None => {}
    };
    
    println!("metallic_factor: {:?}", mat.pbr.metallic_factor);
    
    match &mat.pbr.roughness_texture {
	Some(gray) => { println!("has roughness texture");},
	None => {}
    };
    
    println!("roughness_factor: {:?}", mat.pbr.roughness_factor);
    
    println!("------PBR end------");

    match &mat.normal {
	Some(normal_map) => { println!("has normal map");},
	None => {}
    }

    match &mat.occlusion {
	Some(normal_map) => { println!("has occlusion map");},
	None => {}
    }

    println!("------EM begin------");
    match &mat.emissive.texture {
	Some(normal_map) => { println!("has occlusion map");},
	None => {}
    }

    println!("emissive factor {:?}", &mat.emissive.factor);
    
    println!("------EM end------");
*/
    //println!("----- end Material -----");
}

fn insert_triangles(triangles: &Vec<easy_gltf::model::Triangle>,
		    tri_bucket: &mut TriangleBucket) {
    for t in triangles {
	//println!("triangle");
	for vi in 0..3 {
	    //println!("  vert {}", vi);
	    //visit_vertex(&t[vi]);
	}

	let dbg_scale = 8.0;
	
	let v0 = make_vec_from_vertex(&t[0]) * dbg_scale;
	let v1 = make_vec_from_vertex(&t[1]) * dbg_scale;
	let v2 = make_vec_from_vertex(&t[2]) * dbg_scale;

	tri_bucket.add_tri(&[v0, v1, v2]);
    }
}

/*
fn insert_triangles(triangles: &Vec<easy_gltf::model::Triangle>, tri_mesh: &mut TriangleMesh) {
    for t in triangles {
	//println!("triangle");
	for vi in 0..3 {
	    //println!("  vert {}", vi);
	    //visit_vertex(&t[vi]);
	}

	let dbg_scale = 8.0;
	
	let v0 = make_vec_from_vertex(&t[0]) * dbg_scale;
	let v1 = make_vec_from_vertex(&t[1]) * dbg_scale;
	let v2 = make_vec_from_vertex(&t[2]) * dbg_scale;

	tri_mesh.triangles.push([v0, v1, v2]);
    }
}*/

fn visit_vertex(vert: &easy_gltf::model::Vertex) {

//    println!("pos {:?}", vert.position);
/*    println!("norm {:?}", vert.normal);
    //println!("tangent {:?}", vert.tangent);
    println!("tex_coords {:?}", vert.tex_coords);*/
}

fn make_vec_from_vertex(vert: &easy_gltf::model::Vertex) -> Vec3f {
    Vec3f {
	x: vert.position[0],
	y: vert.position[2],
	z: vert.position[1]
    }
}

pub fn add_cube_object(sb: &mut scene::SceneBuilder) {
    /*
    let mut trimesh = TriangleMesh {
	triangles: vec!(),
};*/

    let mut trimesh = TriangleMesh::new();

    let v0 = Vec3f {
	x: 1.0,
	y: 1.0,
	z: 0.0
    };

    let v1 = Vec3f {
	x: 1.0,
	y: -1.0,
	z: 0.0,
    };

    let v2 = Vec3f {
	x: 1.0,
	y: -1.0,
	z: 2.0
    };

    let v3 = Vec3f {
	x: 1.0,
	y: 1.0,
	z: 2.0
    };

    let v4 = Vec3f {
	x: -1.0,
	y: 1.0,
	z: 0.0
    };

    let v5 = Vec3f {
	x: -1.0,
	y: -1.0,
	z: 0.0,
    };

    let v6 = Vec3f {
	x: -1.0,
	y: -1.0,
	z: 2.0
    };

    let v7 = Vec3f {
	x: -1.0,
	y: 1.0,
	z: 2.0
    };
    
    trimesh.triangles.push([v0, v1, v2]);
    trimesh.triangles.push([v0, v2, v3]);

    trimesh.triangles.push([v4, v5, v6]);
    trimesh.triangles.push([v4, v6, v7]);

    trimesh.triangles.push([v1, v5, v6]);
    trimesh.triangles.push([v1, v6, v2]);

    trimesh.triangles.push([v0, v4, v7]);
    trimesh.triangles.push([v0, v7, v3]);

    trimesh.triangles.push([v3, v2, v6]);
    trimesh.triangles.push([v3, v6, v7]);

    trimesh.bake();

    sb.add_object(Box::new(trimesh),
		  Box::new(DiffuseShader {
		      color: ColorRgbF::CRAYOLA_RED}));
}


pub fn add_cube_gltf_object(sb: &mut scene::SceneBuilder) {
    let scenes = easy_gltf::load("tools/cube.gltf").unwrap();

    for scene in scenes {
	walk_scene(&scene, sb);
    }
}


pub fn add_cube_glv_object(sb: &mut scene::SceneBuilder) {
    let scenes = easy_gltf::load("tools/cube.glb").unwrap();

    for scene in scenes {
	walk_scene(&scene, sb);
    }
}


pub fn add_capped_cone_object(sb: &mut scene::SceneBuilder) {
    let cone_posn = Vec3f {
	x: 0.0,
	y: 0.0,
	z: 0.6
    };
    let cone = CappedCone {
	height: 1.0,
	radius_1: 0.4,
	radius_2: 0.2,
    };
    
    sb.add_object(Box::new(OpTranslate {
	v: Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: 3.0
	},
	primitive: Box::new(cone)
    }),
		  Box::new(SpecularShader {
		      ambient_color: ColorRgbF::CRAYOLA_CARNATION_PINK,
		      diffuse_color: ColorRgbF::CRAYOLA_CARNATION_PINK,
		      specular_color: ColorRgbF::CRAYOLA_CARNATION_PINK,
		      specular_power: 6.0
		  }
		  ));	
}

pub fn add_crayon_object(sb: &mut scene::SceneBuilder, color: ColorRgbF) {

    // total len = 3.6
    // diameter = 0.3
    // tip len = 0.3
    // tip major diameter = 0.2
    // tip minor diameter = 0.1

    let total_len = 3.6;
    let diameter = 0.3;
    let tip_len = 0.3;
    let tip_major_diameter = 0.2;
    let tip_minor_diameter = 0.1;

    // derived dimensions

    let radius = diameter / 2.0;
    let body_len = total_len - tip_len;
    let tip_major_radius = tip_major_diameter / 2.0;
    let tip_minor_radius = tip_minor_diameter / 2.0;

    // TODO probably should OpUnion these
    
    let cone_posn = Vec3f {
	x: 0.0,
	y: 0.0,
	z: body_len + tip_len / 2.0
    };
    let cone = CappedCone {
	height: tip_len,
	radius_1: tip_major_radius,
	radius_2: tip_minor_radius,
    };
    
    sb.add_object(Box::new(OpTranslate {
	v: cone_posn,
	primitive: Box::new(cone)
    }),
		  Box::new(SpecularShader {
		      ambient_color: color,
		      diffuse_color: color,
		      specular_color: color,
		      specular_power: 6.0
		  }
		  ));

    sb.add_object(Box::new(OpTranslate {
	v: Vec3f {
	    x: 0.0,
	    y: 0.0,
	    z: body_len / 2.0
	},
	primitive: Box::new(CylinderCappedZ {
	    h: body_len,
	    r: radius,
	}
	)
    }),
		  Box::new(SpecularShader {
		      ambient_color: color,
		      diffuse_color: color,
		      specular_color: color,
		      specular_power: 6.0
		  }
		  ));
}



pub fn add_crayon_object_at_loc(sb: &mut scene::SceneBuilder, color: ColorRgbF, loc: Vec3f) {

    // total len = 3.6
    // diameter = 0.3
    // tip len = 0.3
    // tip major diameter = 0.2
    // tip minor diameter = 0.1

    let total_len = 3.6;
    let diameter = 0.3;
    let tip_len = 0.3;
    let tip_major_diameter = 0.2;
    let tip_minor_diameter = 0.1;

    // derived dimensions

    let radius = diameter / 2.0;
    let body_len = total_len - tip_len;
    let tip_major_radius = tip_major_diameter / 2.0;
    let tip_minor_radius = tip_minor_diameter / 2.0;

    // TODO probably should OpUnion these
    
    let cone_posn = Vec3f {
	x: loc.x,
	y: loc.y,
	z: loc.z + body_len + tip_len / 2.0
    };
    let cone = CappedCone {
	height: tip_len,
	radius_1: tip_major_radius,
	radius_2: tip_minor_radius,
    };
    
    sb.add_object(Box::new(OpTranslate {
	v: cone_posn,
	primitive: Box::new(cone)
    }),
		  Box::new(SpecularShader {
		      ambient_color: color,
		      diffuse_color: color,
		      specular_color: color,
		      specular_power: 6.0
		  }
		  ));

    sb.add_object(Box::new(OpTranslate {
	v: Vec3f {
	    x: loc.x,
	    y: loc.y,
	    z: loc.z + body_len / 2.0
	},
	primitive: Box::new(CylinderCappedZ {
	    h: body_len,
	    r: radius,
	}
	)
    }),
		  Box::new(SpecularShader {
		      ambient_color: color,
		      diffuse_color: color,
		      specular_color: color,
		      specular_power: 6.0
		  }
		  ));
}


pub fn add_crayon_box_8_objects(sb: &mut scene::SceneBuilder) {
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_WHITE, Vec3f{x: 0.0, y: -2.0, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_RED, Vec3f{x: 0.0, y: -1.5, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_ORANGE, Vec3f{x: 0.0, y: -1.0, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_YELLOW, Vec3f{x: 0.0, y: -0.5, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_GREEN, Vec3f{x: 0.0, y: 0.0, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_BLUE, Vec3f{x: 0.0, y: 0.5, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_PURPLE, Vec3f{x: 0.0, y: 1.0, z: 0.0});
    add_crayon_object_at_loc(sb, ColorRgbF::CRAYOLA_BLACK, Vec3f{x: 0.0, y: 1.5, z: 0.0});
}




