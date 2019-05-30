use crate::transformations::Transformation;
use crate::tuples::Tuple4D;
use crate::material::Material;

#[derive(Debug)]
pub struct Ray {
    origin: Tuple4D,
    dir: Tuple4D
}

impl Ray {

    pub fn new(origin: Tuple4D, dir: Tuple4D) -> Ray {
        assert!(origin.is_point() && dir.is_vec());
        Ray {origin, dir}
    }

    pub fn position(&self, dist: f64) -> Tuple4D {
        self.origin.add(&self.dir.scale(dist))
    }

    pub fn transform(&self, transformation: &Transformation) -> Ray {
        let origin = transformation.transform(&self.origin);
        let direction = transformation.transform(&self.dir);
        Ray::new(origin, direction)
    }

}

#[derive(Clone, Debug)]
pub struct Intersection {
    pub dist: f64,
    pub object_id: usize
}

impl Intersection {

    pub fn hit(intersections: &[Intersection]) -> Option<Intersection> {
        if intersections.len() == 0 {
            None
        } else {
            let mut offset = 0;
            while offset < intersections.len() {
                if intersections[offset].dist >= 0.0 {
                    break;
                }
                offset += 1;
            }
            if offset == intersections.len() {
                None
            } else {
                Some(intersections[offset].clone())
            }
        }
    }

}

pub struct IntersectionPrecomp {
    pub intersection: Intersection,
    pub point: Tuple4D,
    pub eye: Tuple4D,
    pub normal: Tuple4D,
    pub hit_inside: bool,
    pub reflection: Tuple4D
}

impl IntersectionPrecomp {

    pub fn new(intersection: &Intersection, ray: &Ray, shape: &Primitive) -> IntersectionPrecomp {
        let point = ray.position(intersection.dist);
        let eye = ray.dir.scale(-1.0);
        let normal = shape.surface_normal(&point);
        let intersection = intersection.clone();
        let reflection = reflect(&ray.dir, &normal); 
        let hit_inside = normal.dot(&eye) < 0.0;        
        if hit_inside {
            let normal = normal.scale(-1.0);
            IntersectionPrecomp{intersection, point, eye, normal, hit_inside, reflection}
        } else {
            IntersectionPrecomp{intersection, point, eye, normal, hit_inside, reflection}
        }
    }

}

pub trait Primitive {    
    fn object_id(&self) -> usize;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn surface_normal(&self, p: &Tuple4D) -> Tuple4D;
    fn material(&self) -> Material;
}

pub struct UnitSphere {
    id: usize,
    transformation: Transformation,
    material: Material
}

impl UnitSphere {

    pub fn new(id: usize, transformation: Transformation, material: Material) -> UnitSphere {
        UnitSphere{id, transformation, material}
    }

}

impl Primitive for UnitSphere {

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn object_id(&self) -> usize {
        self.id
    } 

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let r = ray.transform(&self.transformation.inverse().unwrap());
        let sphere2ray = r.origin.sub(&Tuple4D::new_point(0.0, 0.0, 0.0));
        let a = r.dir.dot(&r.dir);
        let b = 2.0 * r.dir.dot(&sphere2ray);
        let c = sphere2ray.dot(&sphere2ray) - 1.0;
        let discriminant = (b * b) - 4.0 * a * c;
        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);            
            vec![Intersection{dist: t1, object_id: self.id}, Intersection{dist: t2, object_id: self.id}] 
        }
    }

    fn surface_normal(&self, world_point: &Tuple4D) -> Tuple4D {
        let object_point = self.transformation.inverse().unwrap().transform(&world_point);
        let object_normal = object_point.sub(&Tuple4D::new_point(0.0, 0.0, 0.0));
        let mut world_normal = self.transformation.inverse().unwrap().transpose().transform(&object_normal);
        world_normal.w = 0.0;
        world_normal.normalized()
    }

}

pub struct Plane { 
    id: usize,
    transformation: Transformation,
    material: Material
}

impl Plane {

    pub fn new(id: usize, transformation: Transformation, material: Material) -> Plane {
        Plane{id, transformation, material}
    }

}

impl Primitive for Plane {
    fn object_id(&self) -> usize {
        self.id
    }
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let r = ray.transform(&self.transformation.inverse().unwrap());
        if r.dir.y.abs() < 1e-08 {
            vec![]
        } else {
            let t = -r.origin.y / r.dir.y;
            vec![Intersection{dist: t, object_id: self.id}]
        }
    }
    fn surface_normal(&self, _world_point: &Tuple4D) -> Tuple4D {
        let object_normal = Tuple4D::new_vector(0.0, 1.0, 0.0);
        let mut world_normal = self.transformation.inverse().unwrap().transpose().transform(&object_normal);
        world_normal.w = 0.0;
        world_normal.normalized()
    }
    fn material(&self) -> Material {
        self.material.clone()
    }
}

pub fn reflect(input: &Tuple4D, normal: &Tuple4D) -> Tuple4D {
    return input.sub(&normal.scale(2.0 * input.dot(normal)))
}


