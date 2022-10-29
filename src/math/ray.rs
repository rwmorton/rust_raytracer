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

impl Ray {
    // Construct ray given origin point and direction vector
    pub fn new(o: &Point,d: &Vector) -> Ray {
        Ray {
            o: Point{x: o.x,y: o.y,z: o.z,w: o.w},
            d: Vector{x: d.x,y: d.y,z: d.z}
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let o: Point = Point::new(1.,2.,3.,0.4);
        let d: Vector = Vector::new(-4.,5.,-7.);
        let r: Ray = Ray::new(&o,&d);
        // origin
        assert_eq!(r.o.x,1.);
        assert_eq!(r.o.y,2.);
        assert_eq!(r.o.z,3.);
        assert_eq!(r.o.w,0.4);
        // direction
        assert_eq!(r.d.x,-4.);
        assert_eq!(r.d.y,5.);
        assert_eq!(r.d.z,-7.);
    }
    // should construct correctly
}
