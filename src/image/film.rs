use super::color::Color;

pub struct Film {
    width: usize,
    height: usize,
    frame_buffer: Vec<u8>
}

impl Film {
    /// Construct film
    pub fn new(width: usize,height: usize) -> Film {
        Film {
            width,
            height,
            // frame_buffer: Vec::with_capacity(width*height*4).fill(0)
            frame_buffer: std::iter::repeat(0).take(width*height*4).collect()
        }
    }

    /// Clear the film to given RGBA color
    pub fn clear(&mut self,color: Color) {
        let size: usize = self.width*self.height;
        for i in 0..size {
            self.frame_buffer[ i ] = (color.r*255.) as u8;
            self.frame_buffer[i+1] = (color.g*255.) as u8;
            self.frame_buffer[i+2] = (color.b*255.) as u8;
            self.frame_buffer[i+3] = (color.a*255.) as u8;
        }
    }

    /// Write RGBA color to the framebuffer
    pub fn write_pixel(&mut self,x: usize,y: usize,color: Color) {
        let index: usize = (x*self.height) + self.width;
        //
        self.frame_buffer[ index ] = (color.r*255.) as u8;
        self.frame_buffer[index+1] = (color.b*255.) as u8;
        self.frame_buffer[index+2] = (color.b*255.) as u8;
        self.frame_buffer[index+3] = (color.a*255.) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // should construct correctly
    fn test_new() {
        let film : Film = Film::new(800,600);
        assert_eq!(film.width,800);
        assert_eq!(film.height,600);
        assert_eq!(film.frame_buffer.len(),800*600*4);
        let size: usize = film.width * film.height * 4;
        for i in 0..size {
            assert_eq!(film.frame_buffer[i],0);
        }
    }

    #[test]
    // should clear correctly
    fn test_clear() {
        let mut film : Film = Film::new(800,600);
        let color: Color = Color::new(0.0,0.5,0.75,1.0).unwrap();
        film.clear(color);

        let size: usize = film.width * film.height * 4;
        let mut index: usize = 0;
        for i in 0..size {
            let r = (color.r * 255.) as u8;
            let g = (color.g * 255.) as u8;
            let b = (color.b * 255.) as u8;
            let a = (color.a * 255.) as u8;
            assert_eq!(film.frame_buffer[ index ],r);
            assert_eq!(film.frame_buffer[index+1],g);
            assert_eq!(film.frame_buffer[index+2],b);
            assert_eq!(film.frame_buffer[index+3],a);
            index += 4;
        }
    }
}
