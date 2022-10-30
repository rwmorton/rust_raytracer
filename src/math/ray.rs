use super::point::Point;
use super::vector::Vector;

/// # Ray
/// Ray has an origin (Point) and a direction (Vector)
/// 
/// # Parameters
/// * o (origin)
/// * d (direction)
pub struct Ray {
    pub o: Point,
    pub d: Vector
}

/// implement display trait
impl std::fmt::Display for Ray {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"[origin: {}, direction: {}]",self.o,self.d)
    }
}

impl Ray {
    /// Construct ray given origin point and direction vector
    pub fn new(o: &Point,d: &Vector) -> Ray {
        Ray {
            o: Point{x: o.x,y: o.y,z: o.z},
            d: Vector{x: d.x,y: d.y,z: d.z}
        }
    }

    /// Get point on ray
    pub fn at(&self,t: f64) -> Point {
        // self.o.add_vector(&self.d.scale(t))
        super::point::ZERO
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::math::point::ZERO;

    use super::*;

    #[test]
    // should construct correctly
    fn test_new() {
        let o: Point = Point::new(1.,2.,3.,);
        let d: Vector = Vector::new(-4.,5.,-7.);
        let r: Ray = Ray::new(&o,&d);
        // origin
        assert_eq!(r.o.x,1.);
        assert_eq!(r.o.y,2.);
        assert_eq!(r.o.z,3.);
        // direction
        assert_eq!(r.d.x,-4.);
        assert_eq!(r.d.y,5.);
        assert_eq!(r.d.z,-7.);
    }

    // should get correct point on ray
    fn test_at() {
        let o: Point = Point::new(2.,-1.,0.);
        let d: Vector = Vector::new(1.,2.,3.);
        let r: Ray = Ray::new(&o,&d);
        let t: f64 = 2.;
        let p = r.at(t);
        assert_eq!(p.x,4.);
        assert_eq!(p.y,3.);
        assert_eq!(p.z,6.);
    }
}
