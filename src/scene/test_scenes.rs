// scene/test_scenes.rs
//
// hardcoded test scenes

// TODO clean this up
use crate::bdg_color::ColorRgbF;
use crate::geom::capsule::Capsule;
use crate::geom::cubebox::CubeBox;
use crate::geom::cylinder::{CylinderCappedY, CylinderCappedZ};
use crate::geom::cylinder::CylinderInfiniteX;
use crate::geom::cylinder::CylinderInfiniteY;
use crate::geom::cylinder::CylinderInfiniteZ;
use crate::geom::plane::ZPlusPlane;
use crate::geom::sphere::Sphere;
use crate::geom::torus::Torus;
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
