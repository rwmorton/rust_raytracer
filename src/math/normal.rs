use super::vector::Vector;

/// # Normal
/// 3 dimensional normal
/// Implicitly represented as homogeneous column vector (x,y,z,0)
/// 
/// # Parameters
/// * x
/// * y
/// * z
pub struct Normal {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Normal {
    /// Construct normal from (x,y,z) coords
    pub fn new(x: f64,y: f64,z: f64) -> Normal {
        Normal {x,y,z}
    }

    /// Negate normal
    pub fn neg(&self) -> Normal {
        Normal {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    /// Scale normal
    pub fn scale(&self,factor: f64) -> Normal {
        Normal {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor
        }
    }

    /// Add normals
    pub fn add(&self,n: &Normal) -> Normal {
        Normal {
            x: self.x + n.x,
            y: self.y + n.y,
            z: self.z + n.z
        }
    }

    /// Add vector to normal
    pub fn add_vector(&self,v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }

    /// Take dot product with vector
    pub fn dot(&self,v: &Vector) -> f64 {
        self.x*v.x + self.y*v.y + self.z*v.z
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
        let n_neg: Normal = n.neg();
        assert_eq!(n_neg.x,-1.);
        assert_eq!(n_neg.y,3.);
        assert_eq!(n_neg.z,-5.);
    }

    #[test]
    // should correctly scale normal
    fn test_scale() {
        let n: Normal = Normal::new(3.,-7.,1.);
        let n_scale: Normal = n.scale(3.);
        assert_eq!(n_scale.x,9.);
        assert_eq!(n_scale.y,-21.);
        assert_eq!(n_scale.z,3.);
    }

    #[test]
    // should correctly add two normals
    fn test_add() {
        let n: Normal = Normal::new(-1.,2.,3.);
        let m: Normal = Normal::new(1.,-3.,-2.);
        let m_plus_n: Normal = n.add(&m);
        assert_eq!(m_plus_n.x,0.);
        assert_eq!(m_plus_n.y,-1.);
        assert_eq!(m_plus_n.z,1.);
    }

    #[test]
    // should add vector to normal to give vector
    fn test_add_vector() {
        let n: Normal = Normal::new(1.,-2.,3.);
        let v: Vector = Vector::new(-2.,1.,4.);
        let w: Vector = n.add_vector(&v);
        assert_eq!(w.x,-1.);
        assert_eq!(w.y,-1.);
        assert_eq!(w.z,7.);
    }

    #[test]
    // should return correct dot product with vector
    fn test_dot() {
        let n: Normal = Normal::new(-1.,-2.,3.);
        let v: Vector = Vector::new(5.,0.,3.);
        let dot: f64 = n.dot(&v);
        assert_eq!(dot,4.);
    }
}
