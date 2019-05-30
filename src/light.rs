use crate::image::*;
use crate::tuples::*;
use crate::material::*;
use crate::geometry::*;

pub struct PointLight {
    pub color: Color,
    pub pos: Tuple4D
}

impl PointLight {

    pub fn new(color: Color, pos: Tuple4D) -> PointLight {
        PointLight {color, pos}
    }

    pub fn lighting(&self, material: &Material, point: &Tuple4D, eye: &Tuple4D, normal: &Tuple4D, in_shadow: bool) -> Color {
        let effective_color = material.color.mul(&self.color);
        let light_dir = self.pos.sub(&point).normalized();   
        let ambient = effective_color.scale(material.ambient);
        let light2normal = light_dir.dot(&normal);
        if light2normal < 0.0 || in_shadow {
            let total = ambient;
            total            
        } else {
            let diffuse = effective_color.scale(material.diffuse).scale(light2normal);
            let reflection_vec = reflect(&light_dir.scale(-1.0), &normal);
            let reflection = reflection_vec.dot(&eye);
            if reflection <= 0.0 {
                let total = ambient.add(&diffuse);
                total
            } else {
                let factor = f64::powf(reflection, material.shininess);
                let specular = self.color.scale(material.specular).scale(factor);
                let total = ambient.add(&diffuse).add(&specular);
                total
            }
        }

    }

}