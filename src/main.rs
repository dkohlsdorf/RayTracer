pub mod tuples;
pub mod matrices;
pub mod transformations;
pub mod geometry;
pub mod image;
pub mod raytracer;
pub mod material;
pub mod light;

use image::*;
use transformations::*;
use tuples::*;
use geometry::*;
use raytracer::*;
use light::*;
use material::*;

fn main() {    
    println!("Ray Tracer!");
    let floor = Plane::new(
        0, 
        Transformation::identity(), 
        Material::new(Color::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0, 0.5)
    );
    let left_wall = Plane::new(
        1,
        Transformation::chain(&vec![
            Transformation::translation(Tuple4D::new_vector(0.0, 0.0, 5.0)),
            Transformation::rotate_y((- std::f64::consts::PI / 4.0).to_degrees()),
            Transformation::rotate_x((  std::f64::consts::PI / 2.0).to_degrees()),
        ]),
        Material::new(Color::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0, 0.5)
    );
    let right_wall = Plane::new(
        2,
        Transformation::chain(&vec![
            Transformation::translation(Tuple4D::new_vector(0.0, 0.0, 5.0)),
            Transformation::rotate_y((std::f64::consts::PI / 4.0).to_degrees()),
            Transformation::rotate_x((std::f64::consts::PI / 2.0).to_degrees()),
        ]),
        Material::new(Color::new(1.0, 0.9, 0.9), 0.1, 0.9, 0.0, 200.0, 0.5)
    );
    let middle = UnitSphere::new(
        3,
        Transformation::chain(&vec![
            Transformation::translation(Tuple4D::new_vector(-0.5, 1.0, 0.5))
        ]),
        Material::new(Color::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0, 0.0)
    );
    let right = UnitSphere::new(
        4,
        Transformation::chain(&vec![
            Transformation::translation(Tuple4D::new_vector(1.5, 0.5, -1.5)),
            Transformation::scale(Tuple4D::new_vector(0.5, 0.5, 0.5))
        ]),
        Material::new(Color::new(0.1, 1.0, 0.5), 0.1, 0.7, 0.3, 200.0, 0.0)
    );
    let left = UnitSphere::new(
        5,
        Transformation::chain(&vec![
            Transformation::translation(Tuple4D::new_vector(-1.5, 0.33, -0.75)),
            Transformation::scale(Tuple4D::new_vector(0.33, 0.33, 0.33))
        ]),
        Material::new(Color::new(1.0, 0.8, 0.1), 0.1, 0.7, 0.3, 200.0, 0.0)
    );
    let light = PointLight::new(
        Color::new(1.0, 1.0, 1.0), Tuple4D::new_point(-10.0, 10.0, -10.0));
    let cam = Camera::new(
        512.0, 
        256.0, 
        (std::f64::consts::PI / 3.0).to_degrees(),
        Transformation::view(
            Tuple4D::new_point(0.0, 2.5, -8.0),
            Tuple4D::new_point(0.0, 1.0, 0.0),
            Tuple4D::new_vector(0.0, 1.0, 0.0)
        ) );
    let world = World::new(
        vec![
            Box::from(floor),
            Box::from(left_wall),
            Box::from(right_wall),
            Box::from(middle),
            Box::from(right),
            Box::from(left)
        ], 
        vec![light]
    );   
    let tracer = RayTracer::new(cam, world);
    let img = tracer.trace(5);
    img.write_ppm("scene.png".to_string());
}
