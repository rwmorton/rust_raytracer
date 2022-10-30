use super::vector::Vector;
use super::traits::{Len,LenSq};

/// # Point
/// 3 dimensional affine point
/// Implicitly represented as homogenous column vector (x,y,z,1)
///
/// # Parameters
/// * x
/// * y
/// * z
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// implement display trait
impl std::fmt::Display for Point {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"({},{},{})",self.x,self.y,self.z)
    }
}

/// helpful constants
pub const ZERO : Point = Point{x: 0.0,y: 0.0,z: 0.0};

impl Default for Point {
    fn default() -> Self {
        ZERO
    }
}

/// Add vector to point to get a new point
impl std::ops::Add<Vector> for Point {
    type Output = Point;
    fn add(self,v: Vector) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

/// Subtract two points to get vector between them
impl std::ops::Sub for Point {
    type Output = Vector;
    fn sub(self,p: Point) -> Vector {
        Vector {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z
        }
    }
}

/// Subtract vector from point to get a new point
impl std::ops::Sub<Vector> for Point {
    type Output = Point;
    fn sub(self,v: Vector) -> Point {
        Point {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl Point {
    /// Construct point from (x,y,z,w) coords
    pub fn new(x: f64,y: f64,z: f64) -> Point {
        Point {x,y,z}
    }

    /// Compute squared distance between two points
    pub fn distance_sq(self,p: Point) -> f64 {
        (self - p).len_sq()
    }

    /// Compute distance between two points
    pub fn distance(self,p: Point) -> f64 {
        (self - p).len()
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // is ZERO constant correct
    fn test_zero_constant() {
        assert_eq!(ZERO.x,0.);
        assert_eq!(ZERO.y,0.);
        assert_eq!(ZERO.z,0.);
    }

    #[test]
    // should construct to (x,y,z,w) point
    fn test_new() {
        let p: Point = Point::new(1.,2.,3.);
        assert_eq!(p.x,1.);
        assert_eq!(p.y,2.);
        assert_eq!(p.z,3.);
    }

    #[test]
    // adding a vector to self should result in
    // a new point
    fn test_add_vector() {
        let a: Point = Point::new(1.,-1.,3.);
        let v: Vector = Vector::new(1.,1.,1.);
        let b: Point = a + v;
        assert_eq!(b.x,2.);
        assert_eq!(b.y,0.);
        assert_eq!(b.z,4.);
    }

    #[test]
    // Subtracting a point from self should
    // result in vector between the two points
    fn test_sub() {
        let a: Point = Point::new(1.,2.,3.);
        let b: Point = Point::new(-2.,1.,4.);
        let v: Vector = a - b;
        assert_eq!(v.x,3.);
        assert_eq!(v.y,1.);
        assert_eq!(v.z,-1.);
    }

    #[test]
    // Subtracting a vector from this point should
    // result in a new point
    fn test_sub_vector() {
        let a: Point = Point::new(-4.,3.,1.);
        let v: Vector = Vector::new(1.,-5.,6.);
        let b: Point = a - v;
        assert_eq!(b.x,-5.);
        assert_eq!(b.y,8.);
        assert_eq!(b.z,-5.);
    }

    #[test]
    // squared distance between two points should
    // be the squared length of the vector from
    // self to second point
    fn test_distance_sq() {
        let a: Point = Point::new(3.,-7.,1.);
        let b: Point = Point::new(-5.,2.,7.);
        let len_sq = a.distance_sq(b);
        assert_eq!(len_sq,181.);
    }

    #[test]
    // distance between two points should
    // be the length of the vector from
    // self to second point
    fn test_distance() {
        let a: Point = Point::new(0.,0.,0.);
        let b: Point = Point::new(1.,1.,0.);
        let len = a.distance(b);
        assert_eq!(len,f64::sqrt(2.));
    }
}
