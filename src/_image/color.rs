/// # Color
/// RGBA color type with range[0,1]
///
/// # Parameters
/// * r
/// * g
/// * b
/// * a
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64
}

/// helpful constants
pub const WHITE: Color = Color{r: 1.0,g: 1.0,b: 1.0,a: 1.0};
pub const BLACK: Color = Color{r: 0.0,g: 0.0,b: 0.0,a: 1.0};
pub const RED  : Color = Color{r: 1.0,g: 0.0,b: 0.0,a: 1.0};
pub const GREEN: Color = Color{r: 0.0,g: 1.0,b: 0.0,a: 1.0};
pub const BLUE : Color = Color{r: 0.0,g: 0.0,b: 1.0,a: 1.0};

impl Color {
    /// Construct (r,g,b,a) color
    pub fn new(r: f64,g: f64,b: f64,a: f64) -> Color {

        // assert correct ranges
        assert_eq!(r >= 0.0,true);
        assert_eq!(r <= 1.0,true);
        assert_eq!(g >= 0.0,true);
        assert_eq!(g <= 1.0,true);
        assert_eq!(b >= 0.0,true);
        assert_eq!(b <= 1.0,true);
        assert_eq!(a >= 0.0,true);
        assert_eq!(a <= 1.0,true);
        
        Color {r,g,b,a}
    }
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// UNIT TESTS //////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // is WHITE constant correct?
    fn test_white_constant() {
        assert_eq!(WHITE.r,1.);
        assert_eq!(WHITE.g,1.);
        assert_eq!(WHITE.b,1.);
        assert_eq!(WHITE.a,1.);
    }
    #[test]
    // is BLACK constant correct?
    fn test_black_constant() {
        assert_eq!(BLACK.r,0.);
        assert_eq!(BLACK.g,0.);
        assert_eq!(BLACK.b,0.);
        assert_eq!(BLACK.a,1.);
    }

    #[test]
    // is RED constant correct?
    fn test_red_constant() {
        assert_eq!(RED.r,1.);
        assert_eq!(RED.g,0.);
        assert_eq!(RED.b,0.);
        assert_eq!(RED.a,1.);
    }

    #[test]
    // is GREEN constant correct?
    fn test_green_constant() {
        assert_eq!(GREEN.r,0.);
        assert_eq!(GREEN.g,1.);
        assert_eq!(GREEN.b,0.);
        assert_eq!(GREEN.a,1.);
    }

    #[test]
    // is BLUE constant correct?
    fn test_blue_constant() {
        assert_eq!(BLUE.r,0.);
        assert_eq!(BLUE.g,0.);
        assert_eq!(BLUE.b,1.);
        assert_eq!(BLUE.a,1.);
    }

    #[test]
    // test color construction correct
    fn test_new() {
        let c: Color = Color::new(0.1,0.2,0.3,0.4);
        assert_eq!(c.r,0.1);
        assert_eq!(c.g,0.2);
        assert_eq!(c.b,0.3);
        assert_eq!(c.a,0.4);
    }
}
