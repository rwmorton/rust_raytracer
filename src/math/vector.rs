use super::normal::Normal;
use super::traits::{Len,LenSq,Dot,Cross,Normalize};

/// # Vector
/// 3 dimensional column vector
/// Implicitly represented as homogeneous column vector (x,y,z,0)
/// 
/// # Parameters
/// * x
/// * y
/// * z
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// Helpful constants
pub const ZERO  : Vector = Vector{x: 0.0,y: 0.0,z: 0.0};
pub const X_AXIS: Vector = Vector{x: 1.0,y: 0.0,z: 0.0};
pub const Y_AXIS: Vector = Vector{x: 0.0,y: 1.0,z: 0.0};
pub const Z_AXIS: Vector = Vector{x: 0.0,y: 0.0,z: 1.0};

impl Default for Vector {
    /// default
    fn default() -> Self {
        ZERO
    }
}

/// implement display trait
impl std::fmt::Display for Vector {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"[{},{},{}]",self.x,self.y,self.z)
    }
}

/// Add trait: vector + vector = vector
impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self,v: Self) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

/// Add trait: vector + normal = vector
impl std::ops::Add<Normal> for Vector {
    type Output = Vector;
    fn add(self,n: Normal) -> Vector {
        Vector {
            x: self.x + n.x,
            y: self.y + n.y,
            z: self.z + n.z
        }
    }
}

/// Sub trait
impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self,v: Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

/// Neg trait
impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

/// Multiplication trait (scale vector by scalar)
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self,factor: f64) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }
}

/// Dot product of two vectors
impl Dot<Vector> for Vector {
    fn dot(&self,v: Vector) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }
}

/// Dot product of vector and normal
impl Dot<Normal> for Vector {
    fn dot(&self,n: Normal) -> f64 {
        self.x*n.x + self.y*n.y + self.z*n.z
    }
}

/// Cross product of two vectors
impl Cross<Vector> for Vector {
    type Output = Vector;
    fn cross(&self,v: Vector) -> Vector {
        Vector {
            x: self.y*v.z - self.z*v.y,
            y: self.z*v.x - self.x*v.z,
            z: self.x*v.y - self.y*v.x
        }
    }
}

/// Normalize vector trait
impl Normalize for Vector {
    type Output = Vector;
    fn normalize(&self) -> Result<Vector,String> {
        let v_len = self.len();

        if v_len == 0.0 {
            Err(format!("Division by zero"))
        } else {
            let recip_len = 1.0 / v_len;
            Ok(
                Vector {
                    x: self.x * recip_len,
                    y: self.y * recip_len,
                    z: self.z * recip_len
                }
            )
        }
    }
}

/// Vector length
impl Len for Vector {
    fn len(&self) -> f64 {
        f64::sqrt(self.len_sq())
    }
}

/// Vector length squared
impl LenSq for Vector {
    fn len_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl Vector {
    /// Construct vector from (x,y,z) coords
    pub fn new(x: f64,y: f64,z: f64) -> Vector {
        Vector {x,y,z}
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // is X_AXIS constant correct?
    fn test_x_axis_constant() {
        assert_eq!(X_AXIS.x,1.);
        assert_eq!(X_AXIS.y,0.);
        assert_eq!(X_AXIS.z,0.);
    }

    #[test]
    // is Y_AXIS constant correct?
    fn test_y_axis_constant() {
        assert_eq!(Y_AXIS.x,0.);
        assert_eq!(Y_AXIS.y,1.);
        assert_eq!(Y_AXIS.z,0.);
    }

    #[test]
    // is Z_AXIS constant correct?
    fn test_z_axis_constant() {
        assert_eq!(Z_AXIS.x,0.);
        assert_eq!(Z_AXIS.y,0.);
        assert_eq!(Z_AXIS.z,1.);
    }

    #[test]
    // test vector construction correct
    fn test_new() {
        let a: Vector = Vector::new(1.,2.,3.);
        assert_eq!(a.x,1.);
        assert_eq!(a.y,2.);
        assert_eq!(a.z,3.);
    }

    #[test]
    // test vector add trait
    fn test_add_trait() {
        let a: Vector = Vector::new(3.,4.,3.);
        let b: Vector = Vector::new(9.,2.,4.);
        let c: Vector = a + b;
        assert_eq!(c.x,12.);
        assert_eq!(c.y,6.);
        assert_eq!(c.z,7.);
    }

    #[test]
    // test adding normal to vector (add trait)
    fn test_add_normal_trait() {
        let v: Vector = Vector::new(1.,-2.,3.);
        let n: Normal = Normal::new(2.,2.,2.);
        let w: Vector = v + n;
        assert_eq!(w.x,3.);
        assert_eq!(w.y,0.);
        assert_eq!(w.z,5.);
    }

    #[test]
    // test vector sub trait
    fn test_sub_trait() {
        let a: Vector = Vector::new(1.,-4.,11.);
        let b: Vector = Vector::new(3.,-8.,4.);
        let c: Vector = a - b;
        assert_eq!(c.x,-2.);
        assert_eq!(c.y,4.);
        assert_eq!(c.z,7.);
    }

    #[test]
    // test vector mul trait
    fn test_scale_trait() {
        let v: Vector = Vector::new(1.,5.,7.);
        let scale = 2f64;
        let scaled: Vector = v * scale;
        assert_eq!(scaled.x,2.);
        assert_eq!(scaled.y,10.);
        assert_eq!(scaled.z,14.);
    }

    #[test]
    // test vector neg trait
    fn test_neg_trait() {
        let v: Vector = Vector::new(-5.,2.,7.);
        let v_neg: Vector = -v;
        assert_eq!(v_neg.x,5.);
        assert_eq!(v_neg.y,-2.);
        assert_eq!(v_neg.z,-7.);
    }

    #[test]
    // test vector dot vector trait
    fn test_v_dot_v_trait() {
        let a: Vector = Vector::new(-1.,3.,5.);
        let b: Vector = Vector::new(6.,2.,11.);
        let dot: f64 = a.dot(b);
        assert_eq!(dot,55.);
    }

    #[test]
    // test dot product with normal
    fn test_dot_n_trait() {
        let v: Vector = Vector::new(1.,2.,3.);
        let n: Normal = Normal::new(-3.,-2.,-1.);
        let dot: f64 = v.dot(n);
        assert_eq!(dot,-10.);
    }

    #[test]
    // test vector cross product trait
    fn test_cross() {
        let a: Vector = Vector::new(3.,-3.,1.);
        let b: Vector = Vector::new(4.,9.,2.);
        let c: Vector = a.cross(b);
        assert_eq!(c.x,-15.);
        assert_eq!(c.y,-2.);
        assert_eq!(c.z,39.);
    }

    #[test]
    // test vector length squared
    fn test_len_sq() {
        let v: Vector = Vector::new(3.,4.,5.);
        let len_sq: f64 = v.len_sq();
        assert_eq!(len_sq,50.);
    }

    #[test]
    // test vector length
    fn test_len() {
        let v: Vector = Vector::new(3.,4.,5.);
        let len: f64 = v.len();
        assert_eq!(len,f64::sqrt(v.len_sq()));
    }

    #[test]
    // test vector normalization trait
    fn test_normalize_trait() {
        let v: Vector = Vector::new(8.,-1.,4.);
        let v_norm: Vector = v.normalize().unwrap();
        let len: f64 = v.len();
        assert_eq!(v_norm.x,v.x / len);
        assert_eq!(v_norm.y,v.y / len);
        assert_eq!(v_norm.z,v.z / len);
    }
}
