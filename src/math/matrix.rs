use super::{
    point::Point,
    vector::Vector,
    normal::Normal,
    ray::Ray
};

use super::traits::{Dot};

/// 4x4 matrix
//
/// # Data storage
/// Matrix data is stored in row major order.
//
/// Array indices [0,...,16] are mapped out like so:
//
///  0  1  2  3
///  4  5  6  7
///  8  9 10 11
/// 12 13 14 15
#[derive(Clone,Copy)]
pub struct Matrix {
    pub m: [f64; 16]
}

/// helpful constants
pub const IDENTITY: Matrix = Matrix {
    m: [
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0
    ]
};

/// implement display trait
impl std::fmt::Display for Matrix {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!
        (
            f,
            "| {} {} {} {} |\n
             | {} {} {} {} |\n
             | {} {} {} {} |\n
             | {} {} {} {} |\n",
            self.m[0],self.m[1],self.m[2],self.m[3],
            self.m[4],self.m[5],self.m[6],self.m[7],
            self.m[8],self.m[9],self.m[10],self.m[11],
            self.m[12],self.m[13],self.m[14],self.m[15]
        )
    }
}

/// Scale matrix entries by scalar
impl std::ops::Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self,scale: f64) -> Matrix {
        let mut scaled: Matrix = Matrix {
            m: [0.0; 16]
        };

        for i in 0..16 {
            scaled.m[i] = self.m[i] * scale;
        }

        scaled // return
    }
}

/// Multiply two matrices
impl std::ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self,m: Matrix) -> Matrix {
        Matrix {
            m: [
                // row 1
                self.m[0]*m.m[0] + self.m[1]*m.m[4] + self.m[2]*m.m[8] + self.m[3]*m.m[12],
                self.m[0]*m.m[1] + self.m[1]*m.m[5] + self.m[2]*m.m[9] + self.m[3]*m.m[13],
                self.m[0]*m.m[2] + self.m[1]*m.m[6] + self.m[2]*m.m[10] + self.m[3]*m.m[14],
                self.m[0]*m.m[3] + self.m[1]*m.m[7] + self.m[2]*m.m[11] + self.m[3]*m.m[15],
                // row 2
                self.m[4]*m.m[0] + self.m[5]*m.m[4] + self.m[6]*m.m[8] + self.m[7]*m.m[12],
                self.m[4]*m.m[1] + self.m[5]*m.m[5] + self.m[6]*m.m[9] + self.m[7]*m.m[13],
                self.m[4]*m.m[2] + self.m[5]*m.m[6] + self.m[6]*m.m[10] + self.m[7]*m.m[14],
                self.m[4]*m.m[3] + self.m[5]*m.m[7] + self.m[6]*m.m[11] + self.m[7]*m.m[15],
                // row 3
                self.m[8]*m.m[0] + self.m[9]*m.m[4] + self.m[10]*m.m[8] + self.m[11]*m.m[12],
                self.m[8]*m.m[1] + self.m[9]*m.m[5] + self.m[10]*m.m[9] + self.m[11]*m.m[13],
                self.m[8]*m.m[2] + self.m[9]*m.m[6] + self.m[10]*m.m[10] + self.m[11]*m.m[14],
                self.m[8]*m.m[3] + self.m[9]*m.m[7] + self.m[10]*m.m[11] + self.m[11]*m.m[15],
                // row 4
                self.m[12]*m.m[0] + self.m[13]*m.m[4] + self.m[14]*m.m[8] + self.m[15]*m.m[12],
                self.m[12]*m.m[1] + self.m[13]*m.m[5] + self.m[14]*m.m[9] + self.m[15]*m.m[13],
                self.m[12]*m.m[2] + self.m[13]*m.m[6] + self.m[14]*m.m[10] + self.m[15]*m.m[14],
                self.m[12]*m.m[3] + self.m[13]*m.m[7] + self.m[14]*m.m[11] + self.m[15]*m.m[15]
            ]
        }
    }
}

/// Vector transformation
/// We get each row of the matrix as a 3 dimensional vector
impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self,v: Vector) -> Vector {
        Vector {
            x: v.dot(self.get_row_as_vector(0)),
            y: v.dot(self.get_row_as_vector(1)),
            z: v.dot(self.get_row_as_vector(2))
        }
    }
}

/// Point (affine) transformation
/// 
/// Convert homogeneous point to ordinary points:
/// (x,y,z,w) = (x/w,y/w,z/w)
impl std::ops::Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self,p: Point) -> Point {
        let r1 = self.get_row(0);
        let r2 = self.get_row(1);
        let r3 = self.get_row(2);
        let r4 = self.get_row(3);

        let x: f64 = r1.0*p.x + r1.1*p.y + r1.2*p.z + r1.3;
        let y: f64 = r2.0*p.x + r2.1*p.y + r2.2*p.z + r2.3;
        let z: f64 = r3.0*p.x + r3.1*p.y + r3.2*p.z + r3.3;
        let w: f64 = r4.0*p.x + r4.1*p.y + r4.2*p.z + r4.3;

        // Divide by w to convert point back to a nonhomogenous point
        let w_recip = 1.0 / w; // TODO: handle zero and case when w = 1.0

        Point {
            x: x * w_recip,
            y: y * w_recip,
            z: z * w_recip
        }
    }
}

/// Normal transformation
impl std::ops::Mul<Normal> for Matrix {
    type Output = Normal;
    fn mul(self,n: Normal) -> Normal {
        Normal {
            x: n.dot(self.get_row_as_vector(0)),
            y: n.dot(self.get_row_as_vector(1)),
            z: n.dot(self.get_row_as_vector(2))
        }
    }
}

/// Ray transformation
impl std::ops::Mul<Ray> for Matrix {
    type Output = Ray;
    fn mul(self,r: Ray) -> Ray {
        Ray {
            o: self * r.o,
            d: self * r.d
        }
    }
}

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

    /// Access (row i, column j) of matrix
    pub fn at(&self,i: usize,j: usize) -> Result<f64,String> {
        if i<4 && j<4 {
            Ok(self.m[i*4 + j])
        } else {
            Err(format!("({},{}) is out of range",i,j))
        }
    }

    /// Set with tuples in row order
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

    /// Get i'th row as a 4 dimensional tuple
    pub fn get_row(&self,i: usize) -> (f64,f64,f64,f64) {
        (
            self.m[i*4 + 0],
            self.m[i*4 + 1],
            self.m[i*4 + 2],
            self.m[i*4 + 3],
        )
    }

    /// Get i'th row as a 3 dimensional vector (take w = 0)
    /// so 4th column of each row is ignored in multiplication
    pub fn get_row_as_vector(&self,i: usize) -> Vector {
        Vector {
            x: self.m[i*4 + 0],
            y: self.m[i*4 + 1],
            z: self.m[i*4 + 2]
        }
    }

    /// Transpose of matrix (immutable)
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
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // is IDENTITY constant correct?
    fn test_identity_constant() {
        // row 1
        assert_eq!(IDENTITY.m[0],1.);
        assert_eq!(IDENTITY.m[1],0.);
        assert_eq!(IDENTITY.m[2],0.);
        assert_eq!(IDENTITY.m[3],0.);
        // row 2
        assert_eq!(IDENTITY.m[4],0.);
        assert_eq!(IDENTITY.m[5],1.);
        assert_eq!(IDENTITY.m[6],0.);
        assert_eq!(IDENTITY.m[7],0.);
        // row 3
        assert_eq!(IDENTITY.m[8],0.);
        assert_eq!(IDENTITY.m[9],0.);
        assert_eq!(IDENTITY.m[10],1.);
        assert_eq!(IDENTITY.m[11],0.);
        // row 4
        assert_eq!(IDENTITY.m[12],0.);
        assert_eq!(IDENTITY.m[13],0.);
        assert_eq!(IDENTITY.m[14],0.);
        assert_eq!(IDENTITY.m[15],1.);
    }

    #[test]
    // should construct to identity
    fn test_new() {
        let m: Matrix = Matrix::new();
        // row 1
        assert_eq!(m.m[0],1.);
        assert_eq!(m.m[1],0.);
        assert_eq!(m.m[2],0.);
        assert_eq!(m.m[3],0.);
        // row 2
        assert_eq!(m.m[4],0.);
        assert_eq!(m.m[5],1.);
        assert_eq!(m.m[6],0.);
        assert_eq!(m.m[7],0.);
        // row 3
        assert_eq!(m.m[8],0.);
        assert_eq!(m.m[9],0.);
        assert_eq!(m.m[10],1.);
        assert_eq!(m.m[11],0.);
        // row 4
        assert_eq!(m.m[12],0.);
        assert_eq!(m.m[13],0.);
        assert_eq!(m.m[14],0.);
        assert_eq!(m.m[15],1.);
    }

    #[test]
    // Should access correct (i,j) element
    fn test_at() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        assert_eq!(m.at(0,0).unwrap(),1.);
        assert_eq!(m.at(2,3).unwrap(),12.);
        assert_eq!(m.at(3,1).unwrap(),14.);
        assert_eq!(m.at(0,3).unwrap(),4.);
    }

    #[test]
    // should set with tuples
    fn test_set() {
        let row1 = (1.,2.,3.,4.);
        let row2 = (5.,6.,7.,8.);
        let row3 = (9.,10.,11.,12.);
        let row4 = (13.,14.,15.,16.);
        let mut m: Matrix = Matrix::new();
        m.set(row1,row2,row3,row4);
        // row 1
        assert_eq!(m.m[0],1.);
        assert_eq!(m.m[1],2.);
        assert_eq!(m.m[2],3.);
        assert_eq!(m.m[3],4.);
        // row 2
        assert_eq!(m.m[4],5.);
        assert_eq!(m.m[5],6.);
        assert_eq!(m.m[6],7.);
        assert_eq!(m.m[7],8.);
        // row 3
        assert_eq!(m.m[8],9.);
        assert_eq!(m.m[9],10.);
        assert_eq!(m.m[10],11.);
        assert_eq!(m.m[11],12.);
        // row 4
        assert_eq!(m.m[12],13.);
        assert_eq!(m.m[13],14.);
        assert_eq!(m.m[14],15.);
        assert_eq!(m.m[15],16.);
    }

    #[test]
    // test scaling matrix
    fn test_scale() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let factor: f64 = 2.0;
        let scaled: Matrix = m * factor;
        // row 1
        assert_eq!(scaled.m[0],2.);
        assert_eq!(scaled.m[1],4.);
        assert_eq!(scaled.m[2],6.);
        assert_eq!(scaled.m[3],8.);
        // row 2
        assert_eq!(scaled.m[4],10.);
        assert_eq!(scaled.m[5],12.);
        assert_eq!(scaled.m[6],14.);
        assert_eq!(scaled.m[7],16.);
        // row 3
        assert_eq!(scaled.m[8],18.);
        assert_eq!(scaled.m[9],20.);
        assert_eq!(scaled.m[10],22.);
        assert_eq!(scaled.m[11],24.);
        // row 4
        assert_eq!(scaled.m[12],26.);
        assert_eq!(scaled.m[13],28.);
        assert_eq!(scaled.m[14],30.);
        assert_eq!(scaled.m[15],32.);
    }

    #[test]
    // multiply two matrices
    fn test_mul() {
        let mut a: Matrix = Matrix::new();
        a.set
        (
            (5.,7.,9.,10.),
            (2.,3.,3.,8.),
            (8.,10.,2.,3.),
            (3.,3.,4.,8.)
        );
        let mut b: Matrix = Matrix::new();
        b.set
        (
            (3.,10.,12.,18.),
            (12.,1.,4.,9.),
            (9.,10.,12.,2.),
            (3.,12.,4.,10.)
        );
        let c: Matrix = a * b;
        // row 1
        assert_eq!(c.m[0],210.);
        assert_eq!(c.m[1],267.);
        assert_eq!(c.m[2],236.);
        assert_eq!(c.m[3],271.);
        // row 2
        assert_eq!(c.m[4],93.);
        assert_eq!(c.m[5],149.);
        assert_eq!(c.m[6],104.);
        assert_eq!(c.m[7],149.);
        // row 3
        assert_eq!(c.m[8],171.);
        assert_eq!(c.m[9],146.);
        assert_eq!(c.m[10],172.);
        assert_eq!(c.m[11],268.);
        // row 4
        assert_eq!(c.m[12],105.);
        assert_eq!(c.m[13],169.);
        assert_eq!(c.m[14],128.);
        assert_eq!(c.m[15],169.);
    }

    #[test]
    // get i'th row as 4d tuple
    fn test_get_row() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let row1 = m.get_row(0);
        let row2 = m.get_row(1);
        let row3 = m.get_row(2);
        let row4 = m.get_row(3);
        // row 1
        assert_eq!(row1.0,1.);
        assert_eq!(row1.1,2.);
        assert_eq!(row1.2,3.);
        assert_eq!(row1.3,4.);
        // row 2
        assert_eq!(row2.0,5.);
        assert_eq!(row2.1,6.);
        assert_eq!(row2.2,7.);
        assert_eq!(row2.3,8.);
        // row 3
        assert_eq!(row3.0,9.);
        assert_eq!(row3.1,10.);
        assert_eq!(row3.2,11.);
        assert_eq!(row3.3,12.);
        // row 4
        assert_eq!(row4.0,13.);
        assert_eq!(row4.1,14.);
        assert_eq!(row4.2,15.);
        assert_eq!(row4.3,16.);
    }

    #[test]
    // get i'th row as 3d vector
    fn test_get_row_as_vector() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let row1 = m.get_row_as_vector(0);
        let row2 = m.get_row_as_vector(1);
        let row3 = m.get_row_as_vector(2);
        let row4 = m.get_row_as_vector(3);
        // row 1
        assert_eq!(row1.x,1.);
        assert_eq!(row1.y,2.);
        assert_eq!(row1.z,3.);
        // row 2
        assert_eq!(row2.x,5.);
        assert_eq!(row2.y,6.);
        assert_eq!(row2.z,7.);
        // row 3
        assert_eq!(row3.x,9.);
        assert_eq!(row3.y,10.);
        assert_eq!(row3.z,11.);
        // row 4
        assert_eq!(row4.x,13.);
        assert_eq!(row4.y,14.);
        assert_eq!(row4.z,15.);
    }

    #[test]
    // transform vector by matrix
    fn test_mul_vector() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let v: Vector = Vector{x:4.,y:3.,z:2.};
        let mxv: Vector = m * v;
        assert_eq!(mxv.x,16.);
        assert_eq!(mxv.y,52.);
        assert_eq!(mxv.z,88.);
    }

    #[test]
    // transform point by matrix
    fn test_mul_point() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let p: Point = Point{x:4.,y:3.,z:2.};
        let mxp: Point = m * p;
        let w: f64 = 140.;
        assert_eq!(mxp.x,20./w);
        assert_eq!(mxp.y,60./w);
        assert_eq!(mxp.z,100./w);
    }

    #[test]
    // transform normal by matrix
    fn test_mul_normal() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let n: Normal = Normal{x:4.,y:3.,z:2.};
        let mxn: Normal = m * n;
        assert_eq!(mxn.x,16.);
        assert_eq!(mxn.y,52.);
        assert_eq!(mxn.z,88.);
    }

    #[test]
    // transform ray by matrix
    fn test_mul_ray() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let o: Point = Point{x:4.,y:3.,z:2.};
        let d: Vector = Vector{x:4.,y:3.,z:2.};
        let mut r: Ray = Ray{o,d};
        r = m * r;
        // ray origin
        let w: f64 = 140.;
        assert_eq!(r.o.x,20./w);
        assert_eq!(r.o.y,60./w);
        assert_eq!(r.o.z,100./w);
        // ray direction
        assert_eq!(r.d.x,16.);
        assert_eq!(r.d.y,52.);
        assert_eq!(r.d.z,88.);
    }

    #[test]
    // test transpose of matrix
    fn test_transpose() {
        let mut m: Matrix = Matrix::new();
        m.set
        (
            (1.,2.,3.,4.),
            (5.,6.,7.,8.),
            (9.,10.,11.,12.),
            (13.,14.,15.,16.)
        );
        let t = m.transpose();
        // row 1
        assert_eq!(t.m[0],1.);
        assert_eq!(t.m[1],5.);
        assert_eq!(t.m[2],9.);
        assert_eq!(t.m[3],13.);
        // row 2
        assert_eq!(t.m[4],2.);
        assert_eq!(t.m[5],6.);
        assert_eq!(t.m[6],10.);
        assert_eq!(t.m[7],14.);
        // row 3
        assert_eq!(t.m[8],3.);
        assert_eq!(t.m[9],7.);
        assert_eq!(t.m[10],11.);
        assert_eq!(t.m[11],15.);
        // row 4
        assert_eq!(t.m[12],4.);
        assert_eq!(t.m[13],8.);
        assert_eq!(t.m[14],12.);
        assert_eq!(t.m[15],16.);
    }
}
