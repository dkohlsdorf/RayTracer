use crate::image::*;
use crate::tuples::*;
use crate::geometry::*;
use crate::light::*;
use crate::material::*;
use crate::transformations::*;

pub struct Camera { 
    hsize: f64,
    vsize: f64,    
    half_width: f64,
    half_height: f64,
    pub pxl_sze: f64,    
    cam_transform: Transformation
}

impl Camera {
    pub fn default(hsize: f64, vsize: f64, fov: f64) -> Camera {
        let cam_transform = Transformation::view(
            Tuple4D::new_point(0.0, 0.0, 0.0), 
            Tuple4D::new_point(0.0, 0.0, -1.0),
            Tuple4D::new_vector(0.0, 1.0, 0.0));
        Camera::new(hsize, vsize, fov, cam_transform)  
    }
    pub fn new(hsize: f64, vsize: f64, fov: f64, cam_transform: Transformation) -> Camera {        
        let fov = fov.to_radians();
        let half_view = f64::tan(fov / 2.0);
        let aspect = hsize / vsize;
        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };
        let pxl_sze = (half_width * 2.0) / hsize;
        Camera {hsize, vsize, half_width, half_height, pxl_sze, cam_transform}
    }
    pub fn ray4pxl(&self, x: f64, y: f64) -> Ray {
        let x_offset = (x + 0.5) * self.pxl_sze;
        let y_offset = (y + 0.5) * self.pxl_sze;
        let world_x  = self.half_width - x_offset; 
        let world_y  = self.half_height - y_offset; 
        let inv_transform = self.cam_transform.inverse().unwrap();
        let pixel = inv_transform.transform(
                &Tuple4D::new_point(world_x, world_y, -1.0)
        );
        let origin = inv_transform.transform(
            &Tuple4D::new_point(0.0, 0.0, 0.0)
        );
        let direction = pixel.sub(&origin).normalized();
        Ray::new(origin, direction)
    }
}

pub struct World {
    objects: Vec<Box<Primitive>>,
    lights: Vec<PointLight>
}

impl World {
    pub fn new(objects: Vec<Box<Primitive>>, lights: Vec<PointLight>) -> World {
        World{objects, lights}
    }
    pub fn default() -> World {
        let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Tuple4D::new_point(-10.0, 10.0, -10.0));
        let mut world = World{objects: vec![], lights: vec![light]};
        world.objects.push(Box::from(UnitSphere::new(0, Transformation::identity(), Material::new(Color::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0, 0.0))));
        world.objects.push(Box::from(UnitSphere::new(1, Transformation::scale(Tuple4D::new_vector(0.5, 0.5, 0.5)), Material::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.7, 0.2, 200.0, 0.0))));
        world
    }
    pub fn is_shadowed(&self, light: &PointLight, point: &Tuple4D) -> bool {
        let v = light.pos.sub(&point);
        let distance = v.magnitude();
        let direction = v.normalized();
        let ray = Ray::new(point.clone(), direction);
        let intersections = self.intersect(&ray);
        let hit = Intersection::hit(&intersections);
        if let Some(hit) = hit {
            hit.dist < distance
        } 
        else {
            false
        }
    }
    pub fn reflected_color(&self, comps: &IntersectionPrecomp, steps_left: usize) -> Color {
        let reflection = self.objects[comps.intersection.object_id].material().reflection;
        if  reflection == 0.0 || steps_left == 0 {
            Color::black()
        } else {
            let point = comps.point.add(&comps.normal.scale(0.00001));
            let reflect_ray = Ray::new(point, comps.reflection.clone());
            let color = self.color_at(&reflect_ray, steps_left - 1);
            color.scale(reflection)
        }
    }
    pub fn color_at(&self, ray: &Ray, steps_left: usize) -> Color {
        let intersections = self.intersect(ray);
        let hit = Intersection::hit(&intersections);        
        if let Some(hit)= hit {
            let precomp = IntersectionPrecomp::new(&hit, ray, self.objects[hit.object_id].as_ref());
            self.shade_hit(&precomp, steps_left)            
        } else {
            Color::black()
        }
    }
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for object in self.objects.iter() {
            intersections.extend(object.intersect(ray));
        }
        intersections.sort_by(|a, b| (a.dist).partial_cmp(&b.dist).unwrap());
        intersections
    }
    pub fn shade_hit(&self, precomp: &IntersectionPrecomp, steps_left: usize) -> Color {
        let material = self.objects[precomp.intersection.object_id].material();
        let mut color = Color::black();
        for light in self.lights.iter() {
            let point = precomp.point.add(&precomp.normal.scale(0.00001));
            let in_shadow = self.is_shadowed(light, &point);
            color = color.add(&light.lighting(&material, &point, &precomp.eye, &precomp.normal, in_shadow));
        }
        let reflected = self.reflected_color(precomp, steps_left);
        color.add(&reflected)
    }
}

pub struct RayTracer {
    cam: Camera,
    world: World
}

impl RayTracer {    

    pub fn new(cam: Camera, world: World) -> RayTracer {
        RayTracer{cam, world}
    }   

    pub fn trace(&self, reflection_steps: usize) -> Image {  
        let h = self.cam.hsize as usize;
        let v = self.cam.vsize as usize;
        let mut img = Image::new_rgba(h, v);
        for y in 0 .. v {
            for x in 0 .. h {
                let ray = self.cam.ray4pxl(x as f64, y as f64);
                let color = self.world.color_at(&ray, reflection_steps);
                img.set_rgb(x, y, &color);
            }
        }
        img
    }
}
