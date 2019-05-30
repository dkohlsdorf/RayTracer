use crate::image::Color;

#[derive(Clone)]
pub struct Material {
    pub color:   Color,
    pub ambient:    f64,
    pub diffuse:    f64,
    pub specular:   f64,
    pub shininess:  f64,
    pub reflection: f64
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64, reflection: f64) -> Material {
        Material{color, ambient, diffuse, specular, shininess, reflection}
    }        
    pub fn from_color(color: Color) -> Material {
        Material{color, ambient: 0.1, diffuse: 0.9, specular: 0.9 , shininess: 200.0, reflection: 0.0}
    }        

}
