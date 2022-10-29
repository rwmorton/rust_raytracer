use super::vector::Vector;
use super::point::Point;

// 4x4 matrix
//
// # Data storage
// Matrix data is stored in row major order.
//
// Array indices [0,...,16] are mapped out like so:
//
//  0  1  2  3
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15
pub struct Matrix {
    pub m: [f64; 16]
}

// helpful constants
const IDENTITY: Matrix = Matrix {
    m: [
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0
    ]
};

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            m: [
                1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0
            ]
        }
    }

    // Set with tuples in row order
    pub fn set
    (
        &mut self,
        row1: (f64,f64,f64,f64),
        row2: (f64,f64,f64,f64),
        row3: (f64,f64,f64,f64),
        row4: (f64,f64,f64,f64)
    ) {
        // row 1
        (*self).m[0] = row1.0;
        (*self).m[1] = row1.1;
        (*self).m[2] = row1.2;
        (*self).m[3] = row1.3;
        // row 2
        (*self).m[4] = row2.0;
        (*self).m[5] = row2.1;
        (*self).m[6] = row2.2;
        (*self).m[7] = row2.3;
        // row 3
        (*self).m[8] = row3.0;
        (*self).m[9] = row3.1;
        (*self).m[10] = row3.2;
        (*self).m[11] = row3.3;
        // row 4
        (*self).m[12] = row4.0;
        (*self).m[13] = row4.1;
        (*self).m[14] = row4.2;
        (*self).m[15] = row4.3;
    }

    // Multiply by another matrix (immutable)
    pub fn mul(&self,m: &Matrix) -> Matrix {
        Matrix {
            m: [
                // row 1
                self.m[0]*m.m[0],
                self.m[1]*m.m[4],
                self.m[2]*m.m[8],
                self.m[3]*m.m[12],
                // row 2
                self.m[4]*m.m[1],
                self.m[5]*m.m[5],
                self.m[6]*m.m[9],
                self.m[7]*m.m[13],
                // row 3
                self.m[8]*m.m[2],
                self.m[9]*m.m[6],
                self.m[10]*m.m[10],
                self.m[11]*m.m[14],
                // row 4
                self.m[12]*m.m[3],
                self.m[13]*m.m[7],
                self.m[14]*m.m[11],
                self.m[15]*m.m[15]
            ]
        }
    }

    // Get i'th row as a 4 dimensional tuple
    pub fn get_row(&self,i: usize) -> (f64,f64,f64,f64) {
        (
            self.m[i*4 + 0],
            self.m[i*4 + 1],
            self.m[i*4 + 2],
            self.m[i*4 + 3],
        )
    }

    // Get i'th row as a 3 dimensional vector (take w = 0)
    // so 4th column of each row is ignored in multiplication
    pub fn get_row_as_vector(&self,i: usize) -> Vector {
        Vector {
            x: self.m[i*4 + 0],
            y: self.m[i*4 + 1],
            z: self.m[i*4 + 2]
        }
    }

    // Vector transformation
    // We get each row of the matrix as a 3 dimensional vector
    pub fn mul_vector(&self,v: &Vector) -> Vector {
        Vector {
            x: v.dot(&self.get_row_as_vector(0)),
            y: v.dot(&self.get_row_as_vector(1)),
            z: v.dot(&self.get_row_as_vector(2))
        }
    }

    // Point (affine) transformation
    pub fn mul_point(&self,p: &Point) -> Point {
        let r1 = self.get_row(0);
        let r2 = self.get_row(1);
        let r3 = self.get_row(2);
        let r4 = self.get_row(3);

        Point {
            x: r1.0*p.x + r1.1*p.y + r1.2*p.z + r1.3*p.w,
            y: r2.0*p.x + r2.1*p.y + r2.2*p.z + r2.3*p.w,
            z: r3.0*p.x + r3.1*p.y + r3.2*p.z + r3.3*p.w,
            w: r4.0*p.x + r4.1*p.y + r4.2*p.z + r4.3*p.w,
        }
    }

    // Transpose of matrix (immutable)
    pub fn transpose(&self) -> Matrix {
        Matrix {
            m: [
                // row 1
                self.m[0],
                self.m[4],
                self.m[8],
                self.m[12],
                // row 2
                self.m[1],
                self.m[5],
                self.m[9],
                self.m[13],
                // row 3
                self.m[2],
                self.m[6],
                self.m[10],
                self.m[14],
                // row 4
                self.m[3],
                self.m[7],
                self.m[11],
                self.m[15]
            ]
        }
    }

    // Scale
    // TODO: Is there a away to initialize an array with an expression?
    pub fn scale(&self,scale: f64) -> Matrix {
        let mut scaled: Matrix = Matrix {
            m: [0.0; 16]
        };

        for i in 0..16 {
            scaled.m[i] = self.m[i] * scale;
        }

        scaled // return
    }

    // TODO: Add formatting.
    pub fn print(&self) {
        println!("| {} {} {} {} |",self.m[0],self.m[1],self.m[2],self.m[3]);
        println!("| {} {} {} {} |",self.m[4],self.m[5],self.m[6],self.m[7]);
        println!("| {} {} {} {} |",self.m[8],self.m[9],self.m[10],self.m[11]);
        println!("| {} {} {} {} |",self.m[12],self.m[13],self.m[14],self.m[15]);
    }
}
