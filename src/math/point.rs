/// # Point
/// 4 dimensional affine point (w != 0)
///
/// # Parameters
/// * x
/// * y
/// * z
/// * w
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

/// helpful constants
pub const ZERO : Point = Point{x: 0.0,y: 0.0,z: 0.0,w: 0.0};

impl Default for Point {
    fn default() -> Self {
        ZERO
    }
}

impl Point {
    /// Construct point from (x,y,z,w) coords
    pub fn new(x: f64,y: f64,z: f64,w: f64) -> Point {
        Point {x,y,z,w}
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
        assert_eq!(ZERO.w,0.);
    }

    #[test]
    // should construct to (x,y,z,w) point
    fn test_new() {
        let p: Point = Point::new(1.,2.,3.,4.);
        assert_eq!(p.x,1.);
        assert_eq!(p.y,2.);
        assert_eq!(p.z,3.);
        assert_eq!(p.w,4.);
    }
}
