#[derive(Debug, Clone)]
pub struct Tuple4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple4D {
        Tuple4D {x,y,z,w}
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple4D {
        let w = 1.0;
        Tuple4D {x,y,z,w}
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple4D {
        let w = 0.0;
        Tuple4D {x,y,z,w}
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vec(&self) -> bool {
        self.w == 0.0
    }

    pub fn add(&self, other: &Tuple4D) -> Tuple4D {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        let w = self.w + other.w;
        Tuple4D {x,y,z,w}
    }

    pub fn sub(&self, other: &Tuple4D) -> Tuple4D {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        let w = self.w - other.w;
        Tuple4D {x,y,z,w}
    }

    pub fn scale(&self, scaler: f64) -> Tuple4D {
        let x = self.x * scaler;
        let y = self.y * scaler;
        let z = self.z * scaler;
        let w = self.w * scaler;
        Tuple4D {x,y,z,w}
    }

    pub fn magnitude(&self) -> f64 {        
        f64::sqrt(self.dot(self))
    }

    pub fn normalized(&self) -> Tuple4D {
        let mag = self.magnitude();
        let x = self.x / mag;
        let y = self.y / mag;
        let z = self.z / mag;
        let w = self.w / mag;
        Tuple4D {x,y,z,w}
    }

    pub fn dot(&self, other: &Tuple4D) -> f64 {
        assert!(self.is_vec() && other.is_vec());
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        x + y + z
    }

    pub fn col_dot(&self, other: &[f64]) -> f64 {
        assert!(other.len() == 4);
        let x = self.x * other[0];
        let y = self.y * other[1];
        let z = self.z * other[2];
        let w = self.w * other[3];
        x + y + z + w
    }

    pub fn cross(&self, other: &Tuple4D) -> Tuple4D {
        assert!(self.is_vec() && other.is_vec());
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Tuple4D::new_vector(x, y, z)
    }
}