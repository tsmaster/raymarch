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
