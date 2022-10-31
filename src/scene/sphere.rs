use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::math::ray::Ray;
use crate::math::traits::Dot;

use super::traits::Primitive;

struct Sphere {
    pub radius: f64,
    pub center: Point
}

/// Primitive trait
impl Primitive for Sphere {
    fn hit(&self,ray: &Ray,tmin: &mut f64) -> Option<Point> {

        let m: Vector = ray.o - self.center;
        let b: f64 = m.dot(ray.d);
        let c: f64 = m.dot(m) - self.radius*self.radius;
        if c > 0.0 && b > 0.0 {
            return None
        }
        let discrim: f64 = b*b - c;
        if discrim < 0.0 {
            return None
        }
        *tmin = -b - f64::sqrt(discrim);
        if *tmin < 0.0 {
            *tmin = 0.0;
        }

        // return hit point
        Some(ray.o + ray.d*(*tmin))
    }
}

impl Sphere {
    /// Construct sphere with given center and radius
    pub fn new(radius: f64,center: Point) -> Sphere {
        Sphere {
            radius,
            center
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
    // should construct sphere correctly
    fn test_new() {
        let sphere: Sphere = Sphere::new(
            1.234,
            Point::new(1.,2.,3.)
        );
        assert_eq!(sphere.radius,1.234);
        assert_eq!(sphere.center.x,1.);
        assert_eq!(sphere.center.y,2.);
        assert_eq!(sphere.center.z,3.);
    }

    #[test]
    // test hit
    fn test_hit() {
        //
        let sphere: Sphere = Sphere::new(
            1.,
            Point::new(0.,0.,5.)
        );
        let ray: Ray = Ray::new(
            &Point::new(0.,0.,0.),
            &Vector::new(0.,0.,1.)
        );

        let mut tmin: f64 = f64::INFINITY;
        let hit: Point = sphere.hit(&ray,&mut tmin).unwrap();

        assert_eq!(hit.x,0.0);
        assert_eq!(hit.y,0.0);
        assert_eq!(hit.z,4.0);
    }
}
