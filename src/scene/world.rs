use super::traits::Primitive;

use crate::math::point::Point;
use crate::math::vector::Vector;
use crate::math::ray::Ray;

pub struct World {
    pub primitives: Vec<Box<dyn Primitive>>
}

impl World {
    /// Construct world
    pub fn new(num_primitives: usize) -> World {
        World {
            // primitives: vec![]
            primitives: Vec::with_capacity(num_primitives)
        }
    }

    /// Add primitive
    pub fn add_primitive(&mut self,primitive: Box<dyn Primitive>) {
        self.primitives.push(primitive);
    }

    /// test hit - not final just for testing purposes
    /// returns the very first hit point, none otherwise
    pub fn hit(&self,ray: &Ray) -> Option<Point> {
        let mut thit: f64 = f64::INFINITY;

        let mut result: Option<Point>;

        for primitive in self.primitives.iter() {
            result = primitive.hit(&ray,&mut thit);
            match result {
                Some(hit) => {
                    return Some(hit);
                },
                None => {} // check next primitive
            }
        }

        None // no hit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::sphere::Sphere;
    use crate::scene::plane::Plane;
    use crate::math::normal::Normal;

    #[test]
    // Should construct correctly
    fn test_new() {
        let mut world = World::new(0);
        assert_eq!(world.primitives.len(),0);
        assert_eq!(world.primitives.capacity(),0);

        // construct with size
        world = World::new(5);
        assert_eq!(world.primitives.len(),0);
        assert_eq!(world.primitives.capacity(),5);
    }

    #[test]
    // should add primitive(s)
    fn test_add_primitive() {
        let mut world = World::new(3);
        assert_eq!(world.primitives.len(),0);

        // add sphere
        let mut primitive: Box<dyn Primitive> = Box::new(Sphere::new(1.,Point::new(0.,0.,0.)));
        world.add_primitive(primitive);
        assert_eq!(world.primitives.len(),1);

        // add plane
        primitive = Box::new(Plane::new(&Point::new(0.,0.,0.),&Normal::new(0.,1.,0.)));
        world.add_primitive(primitive);
        assert_eq!(world.primitives.len(),2);
    }

    #[test]
    /// test hit
    fn test_hit_true() {
        let mut world = World::new(1);

        // add sphere
        let primitive: Box<dyn Primitive> = Box::new(Sphere::new(1.,Point::new(0.,0.,5.)));
        world.add_primitive(primitive);

        let ray = Ray::new(&Point::new(0.,0.,0.),&Vector::new(0.,0.,1.));
        
        // we have a hit
        let result = world.hit(&ray);
        let p = result.unwrap();
        assert_eq!(p.x,0.);
        assert_eq!(p.y,0.);
        assert_eq!(p.z,4.);
    }

    #[test]
    #[should_panic]
    /// test hit
    fn test_hit_false() {
        let mut world = World::new(1);

        // add sphere
        let primitive: Box<dyn Primitive> = Box::new(Sphere::new(1.,Point::new(0.,0.,5.)));
        world.add_primitive(primitive);

        let ray = Ray::new(&Point::new(0.,0.,0.),&Vector::new(0.,0.,-1.));
        
        // no hit
        let result = world.hit(&ray);
        result.unwrap(); // should panic
    }
}
