use crate::tuples::Tuple4D;
use crate::matrices::Mat;

pub struct Transformation {
    pub transformation: Mat
}

impl Transformation {

    pub fn view(from: Tuple4D, to: Tuple4D, up: Tuple4D) -> Transformation {
        let fwd = to.sub(&from).normalized();
        let up_norm = up.normalized();
        let left = fwd.cross(&up_norm);
        let true_up = left.cross(&fwd);
        let orientation = Transformation{ transformation: Mat::new(vec![
            left.x,    left.y,    left.z,    0.0,
            true_up.x, true_up.y, true_up.z, 0.0,
            -fwd.x,    -fwd.y,    -fwd.z,    0.0,
            0.0,       0.0,       0.0,       1.0
        ], 4)
        };

        let trans = Transformation::translation(Tuple4D::new_point(-from.x, -from.y, -from.z));
        Transformation::chain(&vec![orientation, trans])
    }

    pub fn chain(transformations: &[Transformation]) -> Transformation {
        let mut transformation = transformations[0].transformation.clone();
        for i in 1 .. transformations.len() {
            transformation = transformation.mat_mul(&transformations[i].transformation);
        }
        Transformation{transformation}
    }

    pub fn identity() -> Transformation {
        let transformation = Mat::new(vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn translation(by: Tuple4D) -> Transformation {
        let transformation = Mat::new(vec![
            1.0, 0.0, 0.0, by.x,
            0.0, 1.0, 0.0, by.y,
            0.0, 0.0, 1.0, by.z,
            0.0, 0.0, 0.0, 1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn scale(by: Tuple4D) -> Transformation {
        let transformation = Mat::new(vec![
            by.x, 0.0, 0.0, 0.0,
            0.0, by.y, 0.0, 0.0,
            0.0, 0.0, by.z, 0.0,
            0.0, 0.0, 0.0,  1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn rotate_x(by: f64) -> Transformation {
        let r = by.to_radians();
        let transformation = Mat::new(vec![
            1.0, 0.0,          0.0,         0.0,
            0.0, f64::cos(r), -f64::sin(r), 0.0,
            0.0, f64::sin(r),  f64::cos(r), 0.0,
            0.0, 0.0,          0.0,         1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn rotate_y(by: f64) -> Transformation {
        let r = by.to_radians();
        let transformation = Mat::new(vec![
             f64::cos(r),  0.0,  f64::sin(r), 0.0,
             0.0,          1.0,  0.0,         0.0,
            -f64::sin(r),  0.0,  f64::cos(r), 0.0,
             0.0,          0.0,  0.0,         1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn rotate_z(by: f64) -> Transformation {
        let r = by.to_radians();
        let transformation = Mat::new(vec![
             f64::cos(r), -f64::sin(r), 0.0, 0.0,
             f64::sin(r),  f64::cos(r), 0.0, 0.0,
             0.0,          0.0,         1.0, 0.0,
             0.0,          0.0,         0.0, 1.0
        ], 4);
        Transformation{transformation}
    }

    pub fn inverse(&self) -> Option<Transformation> {
        match self.transformation.inverse() {
            None => None, 
            Some(mat) => Some(Transformation{transformation: mat})
        }
    }

    pub fn transpose(&self) -> Transformation {
        Transformation {transformation: self.transformation.transpose()}
    }

    pub fn transform(&self, tuple: &Tuple4D) -> Tuple4D {
        self.transformation.mul(tuple)
    }    

}

