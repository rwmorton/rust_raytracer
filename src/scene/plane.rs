use super::traits::Primitive;
use crate::math::{
    point::Point,
    vector::Vector,
    normal::Normal,
    ray::Ray,
    traits::Dot
};

pub struct Plane {
    pub point: Point,
    pub normal: Normal
}

/// Primitive trait
impl Primitive for Plane {
    fn hit(&self,ray: &Ray,tmin: &mut f64) -> Option<Point> {
        let t: f64 = (self.point - ray.o).dot(self.normal) / ray.d.dot(self.normal);

        if t > f64::EPSILON {
            *tmin = t;
            return Some(ray.o + ray.d*(*tmin))
        }

        None
    }
}

impl Plane {
    /// Construct plane
    pub fn new(p: &Point,n: &Normal) -> Plane {
        Plane {
            point: *p,
            normal: *n
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
    // should construct plane correctly
    fn test_new() {
        let plane: Plane = Plane::new(
            &Point::new(1.,2.,3.),
            &Normal::new(-9.,3.,-1.)
        );
        assert_eq!(plane.point.x,1.);
        assert_eq!(plane.point.y,2.);
        assert_eq!(plane.point.z,3.);
        assert_eq!(plane.normal.x,-9.);
        assert_eq!(plane.normal.y,3.);
        assert_eq!(plane.normal.z,-1.);
    }

    #[test]
    // test hit
    fn test_hit() {
        let mut plane: Plane = Plane::new(
            &Point::new(0.,0.,0.),
            &Normal::new(0.,1.,0.)
        );
        let ray: Ray = Ray::new(
            &Point::new(0.,10.,0.),
            &Vector::new(0.,-1.,0.)
        );

        let mut tmin: f64 = f64::INFINITY;

        // test 1
        let mut hit: Point = plane.hit(&ray,&mut tmin).unwrap();
        assert_eq!(hit.x,0.);
        assert_eq!(hit.y,0.);
        assert_eq!(hit.z,0.);

        // test 2
        plane.point.y = 5.;
        hit = plane.hit(&ray,&mut tmin).unwrap();
        assert_eq!(hit.x,0.);
        assert_eq!(hit.y,5.);
        assert_eq!(hit.z,0.);
    }
}
