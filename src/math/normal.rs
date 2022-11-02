use super::{
    vector::Vector,
    traits::{Len,LenSq,Dot,Normalize}
};

/// # Normal
/// 3 dimensional normal
/// Implicitly represented as homogeneous column vector (x,y,z,0)
/// 
/// # Parameters
/// * x
/// * y
/// * z
#[derive(Clone,Copy)]
pub struct Normal {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// implement display trait
impl std::fmt::Display for Normal {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"[{},{},{}]",self.x,self.y,self.z)
    }
}

/// Add trait: normal + normal = normal
impl std::ops::Add for Normal {
    type Output = Normal;
    fn add(self,v: Self) -> Normal {
        Normal {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

/// Add trait: normal + vector = vector
impl std::ops::Add<Vector> for Normal {
    type Output = Vector;
    fn add(self,v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

/// Neg trait
impl std::ops::Neg for Normal {
    type Output = Normal;
    fn neg(self) -> Normal {
        Normal {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

/// Multiplication trait (scale vector by scalar)
impl std::ops::Mul<f64> for Normal {
    type Output = Normal;
    fn mul(self,factor: f64) -> Normal {
        Normal {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }
}

/// Dot product of normal and vector
impl Dot<Vector> for Normal {
    fn dot(&self,v: Vector) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
    }
}

/// Normalize normal trait
impl Normalize for Normal {
    type Output = Normal;
    fn normalize(&self) -> Result<Normal,String> {
        let n_len = self.len();

        if n_len == 0.0 {
            Err(format!("Division by zero"))
        } else {
            let recip_len = 1.0 / n_len;
            Ok(
                Normal {
                    x: self.x * recip_len,
                    y: self.y * recip_len,
                    z: self.z * recip_len
                }
            )
        }
    }
}

/// Normal length
impl Len for Normal {
    fn len(&self) -> f64 {
        f64::sqrt(self.len_sq())
    }
}

/// Normal length squared
impl LenSq for Normal {
    fn len_sq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
}

impl Normal {
    /// Construct normal from (x,y,z) coords
    pub fn new(x: f64,y: f64,z: f64) -> Normal {
        Normal {x,y,z}
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // should construct normal given (x,y,z) coords
    fn test_new() {
        let n: Normal = Normal::new(-8.,5.,2.);
        assert_eq!(n.x,-8.);
        assert_eq!(n.y,5.);
        assert_eq!(n.z,2.);
    }

    #[test]
    // should correctly negate normal
    fn test_neg() {
        let n: Normal = Normal::new(1.,-3.,5.);
        let n_neg: Normal = -n;
        assert_eq!(n_neg.x,-1.);
        assert_eq!(n_neg.y,3.);
        assert_eq!(n_neg.z,-5.);
    }

    #[test]
    // should correctly scale normal
    fn test_scale() {
        let n: Normal = Normal::new(3.,-7.,1.);
        let n_scale: Normal = n * 3.;
        assert_eq!(n_scale.x,9.);
        assert_eq!(n_scale.y,-21.);
        assert_eq!(n_scale.z,3.);
    }

    #[test]
    // should correctly add two normals
    fn test_add() {
        let n: Normal = Normal::new(-1.,2.,3.);
        let m: Normal = Normal::new(1.,-3.,-2.);
        let m_plus_n: Normal = n + m;
        assert_eq!(m_plus_n.x,0.);
        assert_eq!(m_plus_n.y,-1.);
        assert_eq!(m_plus_n.z,1.);
    }

    #[test]
    // should add vector to normal to give vector
    fn test_add_vector() {
        let n: Normal = Normal::new(1.,-2.,3.);
        let v: Vector = Vector::new(-2.,1.,4.);
        let w: Vector = n + v;
        assert_eq!(w.x,-1.);
        assert_eq!(w.y,-1.);
        assert_eq!(w.z,7.);
    }

    #[test]
    // should return correct dot product with vector
    fn test_dot() {
        let n: Normal = Normal::new(-1.,-2.,3.);
        let v: Vector = Vector::new(5.,0.,3.);
        let dot: f64 = n.dot(v);
        assert_eq!(dot,4.);
    }
}
