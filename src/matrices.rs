use crate::tuples::*;

#[derive(Clone, Debug)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    x: Vec<f64>
}

impl Mat {

    pub fn new(x: Vec<f64>, rows: usize) -> Mat {
        let cols = x.len() / rows;
        Mat {rows, cols, x}
    }    

    pub fn eye(n: usize) -> Mat {
        let mut x = vec![0.0; n * n];
        let rows = n;
        let cols = n;
        for i in 0 .. n {
            x[i * n + i] = 1.0;
        }
        Mat {rows, cols, x}
    } 

    pub fn zeros(rows: usize, cols: usize) -> Mat {
        let x = vec![0.0; rows * cols];
        Mat {rows, cols, x}
    }

    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.x[i * self.cols + j]
    }    

    pub fn col(&self, i: usize) -> &[f64] {
        &self.x[i * self.cols .. (i + 1) * self.cols]
    }

    pub fn mul(&self, tuple: &Tuple4D) ->  Tuple4D {
        assert!(self.cols == 4);
        let x = tuple.col_dot(self.col(0));
        let y = tuple.col_dot(self.col(1));
        let z = tuple.col_dot(self.col(2));
        let w = tuple.col_dot(self.col(3));
        Tuple4D::new(x, y, z, w)
    }
    
    pub fn mat_mul(&self, other: &Mat) -> Mat {
        assert!(self.cols == other.rows);
        let mut result = Mat::zeros(self.rows, other.cols);
        for i in 0 .. self.rows {
            for j in 0 .. other.cols {                
                for k in 0 .. self.cols {
                    result.x[i * other.cols + j] += self.at(i, k) * other.at(k, j);
                }
            }
        }
        result 
    }
    
    pub fn transpose(&self) -> Mat {
        let mut transposed = Mat::zeros(self.cols, self.rows);
        for i in 0 .. self.rows {
            for j in 0 .. self.cols {
                transposed.x[j * self.rows + i] = self.at(i, j);
            }
        }
        transposed
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Mat {
        let mut submatrix = Mat::zeros(self.rows - 1, self.cols - 1);
        let mut new_i = 0;
        for i in 0 .. self.rows {
            if i != row {
                let mut new_j = 0;
                for j in 0 .. self.cols {
                    if j != col {
                        submatrix.x[new_i * (self.cols - 1) + new_j] = self.at(i, j);
                        new_j += 1;
                    }
                }
                new_i += 1;
            }
        }
        submatrix
    }

    pub fn minor(&self, i: usize, j: usize) -> f64 {
        assert!(self.cols == self.rows);
        self.submatrix(i, j).determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        assert!(self.cols == self.rows);
        let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
        sign * self.minor(i, j)
    }

    pub fn determinant(&self) -> f64 {
        assert!(self.cols == self.rows);
        if self.rows == 2 {
            self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0)
        } else {
            let mut det = 0.0;
            for column in 0 .. self.cols {
                det += self.x[column] * self.cofactor(0, column);
            }
            det
        }        
    }

    pub fn inverse(&self) -> Option<Mat> {
        assert!(self.cols == self.rows);
        let det = self.determinant();
        if det == 0.0 {
            None
        } else {
            let mut inverted = Mat::zeros(self.cols, self.rows);
            for row in 0 .. self.rows {
                for col in 0 .. self.cols {
                    let c = self.cofactor(row, col);
                    inverted.x[col * self.rows + row] = c / det;
                }
            }
            Some(inverted)
        }
    }
}